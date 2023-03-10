mod cast;
mod cast_from;
mod caster;
mod error;
mod tests;
mod vtable;

pub use blackbox_cast_codegen::cast;

pub use cast::Cast;
pub use cast_from::CastFrom;
pub use caster::Caster;
pub use error::Error;
pub use vtable::{BoxedTraitCaster, CasterId, TargetId, TRAITCASTERS};

pub use linkme;
