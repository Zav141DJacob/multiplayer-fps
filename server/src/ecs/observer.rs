use hecs::{Bundle, Component, ComponentError, DynamicBundle, Entity, NoSuchEntity, World};
use std::ops::{Deref, DerefMut};
use std::vec::Drain;

use common::ecs::components::{
    EcsProtocol, InsertComponent, InsertComponentTuple, RemoveComponentHelper, RemoveComponentTuple,
};

/// Used to observe changes to a given [World] and record those changes as [EcsProtocol] messages.
#[derive(Debug, Default)]
pub struct Observer {
    reliable: Vec<EcsProtocol>,
    unreliable: Vec<EcsProtocol>,
}

impl Observer {
    /// Create an [ObservedWorld] with a given ECS [World]
    pub fn observe<'a>(&'a mut self, world: &'a mut World) -> ObservedWorld<'a> {
        ObservedWorld {
            observer: self,
            world,
            reliable: true,
        }
    }

    /// Wrap a component in a smart pointer that pushes changes to the observer once the component is dropped.
    pub fn observe_component<'a, T>(
        &'a mut self,
        entity: Entity,
        component: &'a mut T,
    ) -> ObservedComponent<'a, T>
    where
        T: Into<InsertComponent> + Clone,
    {
        ObservedComponent {
            observer: self,
            component,
            entity,
            reliable: true,
        }
    }

    /// [Drain] all reliable [EcsProtocol] items that were added since the last time this function was called
    pub fn drain_reliable(&mut self) -> Drain<'_, EcsProtocol> {
        self.reliable.drain(..)
    }

    /// [Drain] all unreliable [EcsProtocol] items that were added since the last time this function was called
    pub fn drain_unreliable(&mut self) -> Drain<'_, EcsProtocol> {
        self.unreliable.drain(..)
    }
}

/// Replicates some of the key functions of a [World].
/// Using these replicated functions will add an [EcsProtocol] item to the parent [Observer] queue.
pub struct ObservedWorld<'a> {
    observer: &'a mut Observer,
    world: &'a mut World,
    reliable: bool,
}

impl<'a> ObservedWorld<'a> {
    fn push(&mut self, item: EcsProtocol) {
        if self.reliable {
            self.observer.reliable.push(item);
        } else {
            self.observer.unreliable.push(item);
        }
    }

    /// Make any following calls add to the unreliable queue.
    /// Mostly intended for constantly updating things like movement.
    pub fn unreliable(mut self) -> Self {
        self.reliable = false;
        self
    }

    /// Same usage as [World::spawn]
    pub fn spawn(
        &mut self,
        components: impl DynamicBundle + InsertComponentTuple + Clone,
    ) -> Entity {
        let entity = self.world.spawn(components.clone());

        components
            .collect_insert()
            .into_iter()
            .for_each(|comp| self.push(EcsProtocol::Insert((entity.to_bits(), comp))));

        entity
    }

    /// Same usage as [World::insert]
    pub fn insert(
        &mut self,
        entity: Entity,
        components: impl DynamicBundle + InsertComponentTuple + Clone,
    ) -> Result<(), NoSuchEntity> {
        self.world.insert(entity, components.clone())?;

        components
            .collect_insert()
            .into_iter()
            .for_each(|comp| self.push(EcsProtocol::Insert((entity.to_bits(), comp))));

        Ok(())
    }

    /// Same usage as [World::insert_one]
    pub fn insert_one(
        &mut self,
        entity: Entity,
        component: impl Component + Into<InsertComponent> + Clone,
    ) -> Result<(), NoSuchEntity> {
        self.world.insert_one(entity, component.clone())?;

        self.push(EcsProtocol::Insert((entity.to_bits(), component.into())));

        Ok(())
    }

    /// Same usage as [World::remove]
    pub fn remove<T: Bundle + 'static + RemoveComponentTuple>(
        &mut self,
        entity: Entity,
    ) -> Result<T, ComponentError> {
        let res = self.world.remove::<T>(entity);

        T::collect_remove()
            .into_iter()
            .for_each(|comp| self.push(EcsProtocol::Remove((entity.to_bits(), comp))));

        res
    }

    /// Same usage as [World::remove_one]
    pub fn remove_one<T: Component + RemoveComponentHelper>(
        &mut self,
        entity: Entity,
    ) -> Result<T, ComponentError> {
        let res = self.world.remove_one::<T>(entity);

        self.push(EcsProtocol::Remove((entity.to_bits(), T::to_remove_enum())));

        res
    }

    /// Same usage as [World::despawn]
    pub fn despawn(&mut self, entity: Entity) -> Result<(), NoSuchEntity> {
        let res = self.world.despawn(entity);

        self.push(EcsProtocol::Despawn(entity.to_bits()));

        res
    }
}

/// Wraps a given component and pushes an insert to the observer once this is dropped.
/// Implements [Deref] and [DerefMut] for accessing the original component.
pub struct ObservedComponent<'a, T>
where
    T: Into<InsertComponent> + Clone,
{
    observer: &'a mut Observer,
    component: &'a mut T,
    entity: Entity,
    reliable: bool,
}

impl<'a, T> ObservedComponent<'a, T>
where
    T: Into<InsertComponent> + Clone,
{
    /// Make any following calls add to the unreliable queue.
    /// Mostly intended for constantly updating things like movement.
    pub fn unreliable(mut self) -> Self {
        self.reliable = false;
        self
    }
}

impl<'a, T> Deref for ObservedComponent<'a, T>
where
    T: Into<InsertComponent> + Clone,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.component
    }
}

impl<'a, T> DerefMut for ObservedComponent<'a, T>
where
    T: Into<InsertComponent> + Clone,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.component
    }
}

impl<'a, T> Drop for ObservedComponent<'a, T>
where
    T: Into<InsertComponent> + Clone,
{
    fn drop(&mut self) {
        // Commit changes once the ObservedComponent is dropped
        let comp = self.component.clone().into();
        let protocol = EcsProtocol::Insert((self.entity.to_bits(), comp));

        if self.reliable {
            self.observer.reliable.push(protocol);
        } else {
            self.observer.unreliable.push(protocol);
        }
    }
}
