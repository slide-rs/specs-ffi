#![no_mangle]

use specs;

c_struct!(SpecsWorld, specs::World, create_world =>  => specs::World::new() => free_world);
