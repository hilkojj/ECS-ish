mod utils;
mod ecs;
mod examples;

// use utils::Bits;
use ecs::*;
use examples::*;


fn main() {
    /*
    let mut bits = Bits::new();

    bits.set(5, true);
    bits.set(32, true);
    bits.set(31, true);

    println!("{}", bits.get(5));
    println!("{}", bits.get(1401));

    let mut bits2 = Bits::new();
    // bits2.set(5, true);
    // bits2.set(31, true);
    bits2.set(30, true);
    bits2.set(32, true);

    println!("bits:  {}\n\nbits2: {}\n\n", &bits, &bits2);

    println!("bits.all(&bits2) -> {}", bits.all(&bits2));

    println!("bits.any(&bits2) -> {}", bits.any(&bits2));

    println!("bits.none(&bits2) -> {}", bits.none(&bits2));
    */

    let mut world = World::new();

    let e = world.create_entity();

    world.add_component(e, {});
    world.add_component(e, 5u32);  // an integer can also be a component if you really want to

    world.add_system(TestSystem {});
    world.add_system(TestSystem {});

}
