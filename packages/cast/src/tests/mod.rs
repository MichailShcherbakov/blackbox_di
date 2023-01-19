use crate::{Cast, CastFrom};
use cast_codegen::cast;
use std::sync::Arc;

trait Injectable: CastFrom {}
trait IService: CastFrom {}

struct Service {}

#[cast]
#[cast(crate)]
impl Injectable for Service {}

#[cast]
#[cast(crate)]
impl IService for Service {}

#[test]
fn cast_to_traits() {
    let service = Arc::new(Service {});
    let _injectable = service.clone().cast::<dyn Injectable>().unwrap();
    let _interface = service.clone().cast::<dyn IService>().unwrap();
}

#[test]
fn cast_traits_among_themselves() {
    let service = Arc::new(Service {});
    let injectable_1 = service.cast::<dyn Injectable>().unwrap();
    let interface = injectable_1.cast::<dyn IService>().unwrap();
    let interface_2 = interface.cast::<dyn Injectable>().unwrap();
    let _initial_service = interface_2.cast::<Service>().unwrap();
}
