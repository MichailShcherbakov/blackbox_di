use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

use crate::caster::Caster;
use linkme::distributed_slice;
use once_cell::sync::Lazy;

pub type BoxedTraitCaster = Box<dyn Any + Send + Sync>;

pub type TargetId = TypeId;
pub type CasterId = TypeId;

#[distributed_slice]
pub static TRAITCASTERS: [fn() -> (TargetId, BoxedTraitCaster)] = [..];

static VTABLE: Lazy<HashMap<(TargetId, CasterId), BoxedTraitCaster>> = Lazy::new(|| {
    TRAITCASTERS
        .iter()
        .map(|f| {
            let (target_id, trait_caster) = f();
            let trait_caster_id = (*trait_caster).type_id();

            ((target_id, trait_caster_id), trait_caster)
        })
        .collect::<HashMap<_, _>>()
});

pub(crate) fn get_trait_caster<T: ?Sized + Any>(target_id: TargetId) -> Option<&'static Caster<T>> {
    let trait_caster_id: CasterId = get_trait_caster_id::<T>();

    VTABLE
        .get(&(target_id, trait_caster_id))
        .and_then(|trait_caster| trait_caster.downcast_ref::<Caster<T>>())
}

pub(crate) fn get_trait_caster_id<T: ?Sized + Any>() -> CasterId {
    TypeId::of::<Caster<T>>()
}
