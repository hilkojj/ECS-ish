use std::{mem, fmt, format};

type Int = u8;
const BITS_PER_INT: usize = mem::size_of::<Int>() * 8;

pub struct Bits {
    ints: Vec<Int>,
}

impl Bits {
    pub fn new() -> Self {
        Self {
            ints: vec![0],
        }
    }

    pub fn set(&mut self, i: usize, val: bool) {
        let int_i = i / BITS_PER_INT;
        while int_i >= self.ints.len() {
            self.ints.push(0);
        }
        let int = self.ints.get_mut(int_i).expect("int_i < ints.len()");
        let i = i - int_i * BITS_PER_INT;
        if val {
            *int |= 1 << i;
        } else {
            *int &= 0 << i;
        }
    }

    pub fn get(&self, i: usize) -> bool {
        let int_i = i / BITS_PER_INT;
        if int_i >= self.ints.len() {
            return false
        }
        let i = i - int_i * BITS_PER_INT;
        let check = 1 << i;
        self.ints.get(int_i).expect("int_i < int.len()") & check == check
    }

    pub fn all(&self, other: &Self) -> bool {
        for i in 0..self.ints.len() {
            println!("{}", i);
            if let (Some(self_int), Some(other_int)) = (self.ints.get(i), other.ints.get(i)) {

                // int is both found in self and other.
                println!("self_int = {:08b}, other_int = {:08b}", self_int, other_int);

                if self_int & other_int != *self_int {
                    return false
                }
            } else {
                return false;
            }
        }

        true
    }

    // todo:
    // pub fn any(&self, other: Self) -> bool {
    //     for i in 0..self.ints.len() {
    //         if let (Some(self_int), Some(other_int)) = (self.ints.get(i), other.ints.get(i)) {

    //             // int is both found in self and other.

    //             if *self_int != 0 && self_int & other_int == 0 {
    //                 return false
    //             }
    //         }
    //     }

    //     true
    // }

    // pub fn none(&self, other: Self) -> bool {
    //     for i in 0..self.ints.len() {
    //         if let (Some(self_int), Some(other_int)) = (self.ints.get(i), other.ints.get(i)) {

    //             // int is both found in self and other.

    //             if self_int & other_int != 0 {
    //                 return false
    //             }
    //         }
    //     }

    //     true
    // }
}

impl fmt::Display for Bits {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut res = String::new();

        for i in 0..self.ints.len() {
            res = res + &format!("{:08b}", &self.ints.get(i).expect("test"));
        }

        write!(f, "{}", res)
    }

}
