use std::{mem, fmt, format, cmp};

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

    /// checks if all bits set in self are also set in other.
    pub fn all(&self, other: &Self) -> bool {
        for i in 0..self.ints.len() {
            if let Some(self_int) = self.ints.get(i) {

                if let Some(other_int) = other.ints.get(i) {

                    // int is both found in self and other.
                    println!("self_int = {:08b}, other_int = {:08b}", self_int, other_int);

                    if self_int & other_int != *self_int {
                        return false
                    }
                } else {
                    // int was not found in other, now return false unless self_int == 000000...
                    return *self_int == 0
                }

            } else {
                return true
            }
        }
        true
    }

    /// checks if any bit set in self is also set in other
    pub fn any(&self, other: &Self) -> bool {
        for i in 0..self.ints.len() {
            if let Some(self_int) = self.ints.get(i) {

                if let Some(other_int) = other.ints.get(i) {

                    // int is both found in self and other.
                    println!("self_int = {:08b}, other_int = {:08b}", self_int, other_int);

                    if self_int & other_int != 0 {
                        return true
                    }
                } else {
                    // int was not found in other
                    return false
                }

            } else {
                return false
            }
        }
        false
    }

    /// checks if all bits set in self are NOT set in other
    pub fn none(&self, other: &Self) -> bool {
        !self.any(other)
    }

}

impl fmt::Display for Bits {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut res = String::new();

        for i in (0..self.ints.len()).rev() {
            res = res + &format!("{:08b}", &self.ints.get(i).expect("test"));
        }

        write!(f, "{}", res)
    }

}

impl PartialEq for Bits {

    fn eq(&self, other: &Bits) -> bool {

        for i in 0..cmp::max(self.ints.len(), other.ints.len()) {

            let self_int = self.ints.get(i).unwrap_or(&0);
            let other_int = other.ints.get(i).unwrap_or(&0);

            if self_int != other_int {
                return false
            }
        }
        true
    }
}

impl Eq for Bits {}
