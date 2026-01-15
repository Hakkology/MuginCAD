//! Structural Elements Module
//!
//! Defines structural entities for architectural drawings:
//! - Column: Vertical structural element at axis intersections
//! - Beam: Horizontal structural element connecting columns
//! - Flooring: Floor slab bounded by beams
//! - Door/Window: Openings in walls/beams

pub mod beam;
pub mod column;
pub mod door;
pub mod flooring;
pub mod manager;
pub mod types;
pub mod window;

pub use beam::Beam;
pub use column::Column;
pub use door::Door;
pub use flooring::Flooring;
pub use manager::StructuralTypeManager;
pub use window::Window;
