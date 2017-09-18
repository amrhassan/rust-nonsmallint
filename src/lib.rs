
#![feature(conservative_impl_trait)] 

#[macro_use] #[cfg(test)] extern crate quickcheck;
#[cfg(test)] use quickcheck::Arbitrary;
#[cfg(test)] use quickcheck::Gen;

use std::iter::repeat;
use std::cmp::max;

/// Least-significant-digit first multi-word decimal number
#[derive(Eq, Debug, Clone)]
pub struct NonSmallInt { digits: Vec<u8> }

const RADIX: u64 = 10;

impl NonSmallInt {

    pub fn of(n: u64) -> Option<NonSmallInt> {
        let str_digits = format!("{}", n);
        let mut digits = Vec::new();
        let mut is_number = true;
        for c in str_digits.trim().chars() {
            if c.is_digit(RADIX as u32) {
                digits.push(c.to_digit(RADIX as u32).expect("Failed to parse digit") as u8)
            } else {
                is_number = false;
            }
        };
        digits.reverse();

        if is_number {
            Some(NonSmallInt { digits: digits})
        } else {
            None
        }
    }

    pub fn length(&self, radix: u64) -> usize {
        if radix == RADIX {
            self.digits.iter().rev().skip_while(|&n| *n == 0).count()
        } else {
            panic!("Unsupported feature: computing length of different radix")
        }
    }

//    pub fn add_to_digit(&mut self, ix: usize, rhs: u8) {
//        require(rhs <= MAX_DIGIT as u8, "input is bigger than a single digit");
//
//        // Resize if necessary
//        if ix >= self.digits.len() {
//            self.digits.resize(ix+1, 0);
//        }
//
//        // Add
//        self.digits[ix] += rhs;
//
//        // Smooth out the numbers bigger than RADIX by carrying over excess
//        let mut carry = 0;
//        for j in ix..self.digits.len() {
//            self.digits[j] += carry;
//            carry = 0;
//            if self.digits[j] > MAX_DIGIT {
//                carry = self.digits[j] / RADIX;
//                self.digits[j] = self.digits[j] % RADIX;
//            }
//            if carry == 0 { break }
//        }
//    }

    pub fn multiply_by(&self, rhs: u32) -> NonSmallInt {
        let mut out_digits = Vec::new();
        let mut carry = 0u64;
        for digit in self.digits.iter() {
            let temp: u64 = (rhs as u64) * (*digit as u64) + carry;
            let out: u8 = (temp % RADIX) as u8;
            carry = temp / RADIX;
            out_digits.push(out);
        }
        while carry != 0 {
            let out: u8 = (carry % RADIX) as u8;
            carry = carry / RADIX;
            out_digits.push(out);
        }
        NonSmallInt { digits: out_digits }
    }

    pub fn quotient(&self, rhs: u32) -> NonSmallInt {
        let mut out_digits = Vec::new();
        let mut carry = 0u64;
        for digit in self.digits.iter().rev() {
            let temp: u64 = carry * RADIX + (*digit as u64);
            let out: u8 = (temp / rhs as u64) as u8;
            carry = temp % (rhs as u64);
            out_digits.insert(0, out);
        }
        NonSmallInt { digits: out_digits }
    }

    pub fn remainder(&self, rhs: u32) -> NonSmallInt {
        let mut out_digits = Vec::new();
        let mut carry = 0u64;
        for digit in self.digits.iter().rev() {
            carry = (carry * RADIX + *digit as u64) % (rhs as u64);
        }
        while carry > 0 {
            let out = carry % RADIX;
            carry = carry / RADIX;
            out_digits.push(out as u8);
        }
        NonSmallInt { digits: out_digits }
    }

    /// Returns (quotient, remainder)
    pub fn div_by_u32(&self, rhs: u32) -> (NonSmallInt, NonSmallInt) {
        let mut quotient = Vec::new();
        let mut carry = 0u64;
        for digit in self.digits.iter().rev() {
            let temp: u64 = carry * RADIX + (*digit as u64);
            let out: u8 = (temp / rhs as u64) as u8;
            carry = temp % (rhs as u64);
            quotient.insert(0, out);
        }
        let mut remainder = Vec::new();
        while carry > 0 {
            let out = carry % RADIX;
            carry = carry / RADIX;
            remainder.push(out as u8);
        }
        (NonSmallInt { digits: quotient }, NonSmallInt { digits: remainder })
    }

    pub fn div(&self, rhs: &NonSmallInt) -> (NonSmallInt, NonSmallInt) {
        if rhs.length(RADIX) == 1 {
            self.div_by_u32(rhs.digits[0] as u32)
        } else if self.length(RADIX) < rhs.length(RADIX) {
            (NonSmallInt { digits: vec![] }, self.clone())
        } else {
            panic!("TODO")
        }
    }

    pub fn lt(&self, rhs: &NonSmallInt) -> bool {
        if self.length(RADIX) < rhs.length(RADIX) {
            true
        } else {
            let lhs_digits = self.digits.iter().rev().skip_while(|&&n| n == 0);
            let rhs_digits = rhs.digits.iter().rev().skip_while(|&&n| n == 0);
            match lhs_digits.zip(rhs_digits).skip_while(|&(&lhs_d, &rhs_d)| lhs_d == rhs_d).next() {
                None => false,
                Some((lhs_d, rhs_d)) => lhs_d < rhs_d
            }
        }
    }

    fn digits_padded_to_length<'a>(&'a self, n: usize) -> impl Iterator<Item=&'a u8> {
        static ZERO: u8 = 0;
        self.digits.iter().chain(repeat(&ZERO).take(n - self.digits.len()))
    }

    /// Left-pads the shorter one if necessary
    fn digits_zipped_with_digits_from<'a>(&'a self, rhs: &'a NonSmallInt) -> impl Iterator<Item=(&'a u8, &u8)> {
        let length = max(self.digits.len(), rhs.digits.len());
        self.digits_padded_to_length(length).zip(rhs.digits_padded_to_length(length))
    }

    /// Result or None for underflow
    pub fn minus(&self, rhs: &NonSmallInt) -> Option<NonSmallInt> {
        let mut out = Vec::new();
        let mut borrow = 0u32;
        for (&l, &r) in self.digits_zipped_with_digits_from(rhs) {
            let diff: u32 = (RADIX as u32 + l as u32).wrapping_sub(r as u32 + borrow);
            out.push((diff % RADIX as u32) as u8);
            borrow = 1 - diff / RADIX as u32;
        }
        if borrow == 0 {
            Some(NonSmallInt { digits: out })
        } else {
            None
        }
    }
}

impl PartialEq for NonSmallInt {
    fn eq(&self, other: &NonSmallInt) -> bool {
        self.digits.iter().rev().skip_while(|&n| *n == 0).eq(other.digits.iter().rev().skip_while(|&n| *n == 0))
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    impl Arbitrary for NonSmallInt {
        fn arbitrary<G: Gen>(g: &mut G) -> NonSmallInt {
            NonSmallInt::of(u64::arbitrary(g)).unwrap()
        }
    }

    #[derive(Clone, Debug)]
    /// A NonSmallInt along with the same value as u64
    struct MinimalNonSmallInt { nsi: NonSmallInt, n: u64 }

    impl Arbitrary for MinimalNonSmallInt {
        fn arbitrary<G: Gen>(g: &mut G) -> MinimalNonSmallInt {
            let n = u64::arbitrary(g);
            let nsi = NonSmallInt::of(n).unwrap();
            MinimalNonSmallInt{nsi, n}
        }
    }

    quickcheck! {

        fn counts_length_correctly(x: MinimalNonSmallInt) -> bool {
            if x.n > 0 {
                x.nsi.length(RADIX) == format!("{}", x.n).len()
            } else {
                x.nsi.length(RADIX) == 0
            }
        }

        fn less_than(x: MinimalNonSmallInt, y: MinimalNonSmallInt) -> bool {
            if x.n < y.n {
                x.nsi.lt(&y.nsi)
            } else {
                true
            }
        }

        fn multiplies_by_u32(x: u32, y: u32) -> bool {
            let xnsi = NonSmallInt::of(x as u64).unwrap();
            let expected = NonSmallInt::of((x * y) as u64).unwrap();

            xnsi.multiply_by(y) == expected
        }

        fn quotient_by_u32(x: u64, y: u32) -> bool {
            if y != 0 {
                let xnsi = NonSmallInt::of(x).unwrap();
                let expected = NonSmallInt::of(x / y as u64).unwrap();

                xnsi.quotient(y) == expected
            } else {
                true
            }
        }

        fn remainder_by_u32(x: u64, y: u32) -> bool {
            if y != 0 {
                let xnsi = NonSmallInt::of(x).unwrap();
                let expected = NonSmallInt::of(x % y as u64).unwrap();

                xnsi.remainder(y) == expected
            } else {
                true
            }
        }

        fn div_by_u32(x: u64, y: u32) -> bool {
            if y != 0 {
                let xnsi = NonSmallInt::of(x).unwrap();
                let expected = (NonSmallInt::of((x / y as u64) as u64).unwrap(), NonSmallInt::of((x % y as u64) as u64).unwrap());
                xnsi.div_by_u32(y) == expected
            } else {
                true
            }
        }

        fn subtracts(x: MinimalNonSmallInt, y: MinimalNonSmallInt) -> bool {
            if x.n >= y.n {
                x.nsi.minus(&y.nsi).unwrap() == NonSmallInt::of(x.n - y.n).unwrap()
            } else {
                x.nsi.minus(&y.nsi).is_none()
            }
        }
    }

    //#[test]
    //fn adds_to_digits() {
    //    let mut x = NonSmallInt{digits: vec![8, 4, 9, 1]};
    //    x.add_to_digit(1, 8);
    //    let expected = NonSmallInt{digits: vec![8, 2, 0, 2]};

    //    assert_eq!(expected, x)
    //}
}
