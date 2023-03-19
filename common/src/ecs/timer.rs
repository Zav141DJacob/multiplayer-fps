use std::time::{Duration, Instant};
use hecs::{Component, Entity, World};

#[derive(Debug, Clone)]
pub struct Timer<T> {
    start_time: Instant,
    end_time: Instant,
    data: T,
}

impl<T: Component> Timer<T> {
    /// Creates a new timer with a given data value.
    pub fn new(duration: Duration, data: T) -> Timer<T> {
        let start_time = Instant::now();
        let end_time = start_time + duration;
        Self {
            start_time,
            end_time,
            data,
        }
    }

    /// Checks for entities with finished timers.
    /// Finished timers are removed and their data is dropped.
    pub fn system(world: &mut World) {
        Self::finished_entities(world)
            .into_iter()
            .for_each(|entity| {
                world.remove_one::<Self>(entity).unwrap();
            })
    }

    /// Checks for entities with finished timers.
    /// All finished timers also call the given function with the relevant entity and the timer's data.
    pub fn system_with(world: &mut World, mut f: impl FnMut(&mut World, Entity, T)) {
        Self::finished_entities(world)
            .into_iter()
            .for_each(|entity| {
                let timer = world.remove_one::<Self>(entity).unwrap();
                f(world, entity, timer.data);
            })
    }

    /// Checks for entities with finished timers.
    /// All finished timers also insert their data back into the entity.
    /// This is equivalent to the following:
    /// ```
    /// Self::system_with(world, |world, entity, data| {
    ///     world.insert_one(entity, data).unwrap();
    /// })
    /// ```
    pub fn system_with_insert(world: &mut World) {
        Self::system_with(world, |world, entity, data| {
            world.insert_one(entity, data).unwrap();
        })
    }

    /// Queries a vec of entities that have finished their timers.
    pub fn finished_entities(world: &mut World) -> Vec<Entity> {
        let now = Instant::now();
        world.query_mut::<&Self>()
            .into_iter()
            .filter(|(_, timer)| timer.end_time <= now)
            .map(|(entity, _)| entity)
            .collect()
    }

    /// Returns the progress of this timer as a f32 in the range 0.0..1.0
    pub fn progress(&self) -> f32 {
        let now = Instant::now();
        (now - self.start_time).as_secs_f32() / (self.end_time - self.start_time).as_secs_f32()
    }
}
