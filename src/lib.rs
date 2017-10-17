#![no_mangle]

extern crate shred;
extern crate specs;

#[repr(C)]
pub struct SpecsWorld(pub specs::World);

pub fn create_world() -> *mut SpecsWorld {
    let world = SpecsWorld(specs::World::new());
    let world = Box::new(world);

    Box::into_raw(world)
}

pub fn free_world(w: *mut SpecsWorld) {
    unsafe { Box::from_raw(w); }
}
