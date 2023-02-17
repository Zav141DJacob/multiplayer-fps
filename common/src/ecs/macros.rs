// WARNING: Lots of macro fuckery ahead

/// Creates two enums: InsertComponent and RemoveComponent.
/// Both contain all the provided types in different ways.
///
/// The generated code looks something like this:
/// ```rust
/// // Inputs to the macro
/// pub struct ComponentA {}
/// pub struct ComponentB {}
///
/// // Below is the generated part
/// pub enum InsertComponent {
///     ComponentA(ComponentA),
///     ComponentB(ComponentB),
///     // And so on...
/// }
///
/// pub enum RemoveComponent {
///     ComponentA,
///     ComponentB,
///     // And so on...
/// }
/// ```
#[macro_export]
macro_rules! register_shared_components {
    ($($name:ident),+$(,)?) => {
        use $crate::{insert_impls, remove_impls};

        // Create enums
        /// Represents inserting a single component into a world
        #[derive(Debug, Clone, serde::Serialize, serde::Deserialize, derive_more::From)]
        pub enum InsertComponent {
            $($name($name),)+
        }

        /// Represents removing a single component from the world
        #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
        pub enum RemoveComponent {
            $($name,)+
        }

        // Create methods to work on a world
        impl InsertComponent {
            /// Apply this component insertion to a world
            pub fn apply(self, world: &mut hecs::World, entity: hecs::Entity) -> Result<(), hecs::NoSuchEntity> {
                match self {
                    $(
                        InsertComponent::$name(comp) => world.insert_one(entity, comp),
                    )+
                }
            }

            pub fn query_all(world: &mut hecs::World) -> Vec<EcsProtocol> {
                let mut vec = Vec::new();

                $(vec.extend(
                    world.query_mut::<&$name>().into_iter()
                        .map(|(entity, component)| {
                            EcsProtocol::Insert((entity.to_bits(), InsertComponent::from(component.clone())))
                        })
                );)+

                vec
            }
        }

        impl RemoveComponent {
            /// Apply this component removal to a world
            pub fn apply(self, world: &mut hecs::World, entity: hecs::Entity) -> Result<(), hecs::ComponentError> {
                match self {
                    $(
                        RemoveComponent::$name => { world.remove_one::<$name>(entity)?; },
                    )+
                }
                Ok(())
            }
        }


        // Set up Bundle -> InsertComponent conversions
        pub trait InsertComponentTuple {
            fn collect_insert(self) -> Vec<InsertComponent>;
        }

        insert_impls!();


        // Set up Bundle -> RemoveComponent conversions
        pub trait RemoveComponentHelper {
            fn to_remove_enum() -> RemoveComponent;
        }

        $(impl RemoveComponentHelper for $name {
            fn to_remove_enum() -> RemoveComponent {
                RemoveComponent::$name
            }
        })+

        pub trait RemoveComponentTuple {
            fn collect_remove() -> Vec<RemoveComponent>;
        }

        remove_impls!();
    };
}

#[macro_export]
macro_rules! insert_impls {
    ( $( $name:ident )+ ) => {
        impl<$($name: Into<InsertComponent>),+> InsertComponentTuple for ($($name,)+)
        {
            fn collect_insert(self) -> Vec<InsertComponent> {
                #[allow(non_snake_case)]
                let ($($name,)+) = self;
                vec![$($name.into()),+]
            }
        }
    };
    () => {
        insert_impls! { A }
        insert_impls! { A B }
        insert_impls! { A B C }
        insert_impls! { A B C D }
        insert_impls! { A B C D E }
        insert_impls! { A B C D E F }
        insert_impls! { A B C D E F G }
        insert_impls! { A B C D E F G H }
        insert_impls! { A B C D E F G H I }
        insert_impls! { A B C D E F G H I J }
        insert_impls! { A B C D E F G H I J K }
        insert_impls! { A B C D E F G H I J K L }
    };
}

#[macro_export]
macro_rules! remove_impls {
    ( $( $name:ident )+ ) => {
        impl<$($name: RemoveComponentHelper),+> RemoveComponentTuple for ($($name,)+)

        {
            fn collect_remove() -> Vec<RemoveComponent> {
                vec![$($name::to_remove_enum()),+]
            }
        }
    };
    () => {
        remove_impls! { A }
        remove_impls! { A B }
        remove_impls! { A B C }
        remove_impls! { A B C D }
        remove_impls! { A B C D E }
        remove_impls! { A B C D E F }
        remove_impls! { A B C D E F G }
        remove_impls! { A B C D E F G H }
        remove_impls! { A B C D E F G H I }
        remove_impls! { A B C D E F G H I J }
        remove_impls! { A B C D E F G H I J K }
        remove_impls! { A B C D E F G H I J K L }
    }
}

#[macro_export]
macro_rules! bulk_attribute {
    ($derive:meta; $($item:item) +) => {
        $(
            #[$derive]
            $item
        ) +
    };
}