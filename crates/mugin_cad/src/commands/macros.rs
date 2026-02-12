/// Basit command struct tanımlar: `points: Vec<Vector2>` alanı ve `new()` fonksiyonu üretir.
///
/// Kullanım:
///   define_command!(LineCommand);
///   define_command!(ArcCommand, filled: bool = false, clockwise: bool = false);
macro_rules! define_command {
    ($name:ident) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            points: Vec<crate::model::Vector2>,
        }
        impl $name {
            pub fn new() -> Self {
                Self { points: Vec::new() }
            }
        }
    };
    ($name:ident, $($field:ident : $ty:ty = $default:expr),+ $(,)?) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            points: Vec<crate::model::Vector2>,
            $(pub $field: $ty,)+
        }
        impl $name {
            pub fn new() -> Self {
                Self {
                    points: Vec::new(),
                    $($field: $default,)+
                }
            }
        }
    };
}

/// Manipulation command struct tanımlar:
/// `points: Vec<Vector2>` + `entity_indices: Vec<usize>` alanları ve `new()` üretir.
///
/// Kullanım:
///   define_manipulation_command!(MoveCommand);
///   define_manipulation_command!(CopyCommand, copied_entities: Vec<Entity> = Vec::new(), is_cut: bool = false);
macro_rules! define_manipulation_command {
    ($name:ident) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            points: Vec<crate::model::Vector2>,
            entity_indices: Vec<usize>,
        }
        impl $name {
            pub fn new() -> Self {
                Self {
                    points: Vec::new(),
                    entity_indices: Vec::new(),
                }
            }
        }
    };
    ($name:ident, $($field:ident : $ty:ty = $default:expr),+ $(,)?) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            points: Vec<crate::model::Vector2>,
            entity_indices: Vec<usize>,
            $(pub $field: $ty,)+
        }
        impl $name {
            pub fn new() -> Self {
                Self {
                    points: Vec::new(),
                    entity_indices: Vec::new(),
                    $($field: $default,)+
                }
            }
        }
    };
}

/// `Command` trait'inin tekrarlanan `get_points()` ve `clone_box()` impl'lerini üretir.
///
/// Kullanım: impl_command_common!(LineCommand);
macro_rules! impl_command_common {
    ($name:ident) => {
        fn get_points(&self) -> &[crate::model::Vector2] {
            &self.points
        }

        fn clone_box(&self) -> Box<dyn crate::commands::Command> {
            Box::new(self.clone())
        }
    };
}
