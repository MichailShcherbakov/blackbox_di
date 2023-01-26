#![allow(unused)]

use std::sync::Arc;

use crate::cast;
use crate::Cast;
use crate::CastFrom;

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
    let injectable = service.clone().cast::<dyn Injectable>().unwrap();
    let interface = service.clone().cast::<dyn IService>().unwrap();
}

#[test]
fn cast_traits_among_themselves() {
    let service = Arc::new(Service {});
    let injectable_1 = service.cast::<dyn Injectable>().unwrap();
    let interface = injectable_1.cast::<dyn IService>().unwrap();
    let interface_2 = interface.cast::<dyn Injectable>().unwrap();
    let initial_service = interface_2.cast::<Service>().unwrap();
}
