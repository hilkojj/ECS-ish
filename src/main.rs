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
    bits2.set(5, true);
    bits2.set(31, true);
    bits2.set(30, true);
    bits2.set(32, true);

    println!("Does bits2 have all of bits? {}", bits.all(bits2));

}
