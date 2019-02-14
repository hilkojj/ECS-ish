mod utils;

use utils::Bits;

fn main() {
    
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

}
