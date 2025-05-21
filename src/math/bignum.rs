#![allow(dead_code)]
use std::cmp::{max, min};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

const CHUNK_SIZE: usize = 32;
const BASE: u64 = 1 << CHUNK_SIZE;

#[derive(Clone, Debug, Eq, PartialEq)]
struct BigNumber {
    chunks: Vec<u32>,
    negative: bool,
}

impl BigNumber {
    pub fn new(val: i128) -> Self {
        if val == 0 {
            return BigNumber {
                chunks: vec![0],
                negative: false,
            };
        }

        let negative = val < 0;
        let mut abs = val.unsigned_abs();

        let mut chunks = Vec::new();

        while abs > 0 {
            chunks.push((abs & 0xFFFF_FFFF) as u32);
            abs = abs.unbounded_shr(32);
        }

        BigNumber { negative, chunks }
    }

    pub fn is_zero(&self) -> bool {
        self.chunks.len() == 1 && self.chunks[0] == 0
    }

    pub fn abs(&self) -> Self {
        let mut result = self.clone();
        result.negative = false;
        result
    }

    pub fn one() -> Self {
        Self {
            chunks: [1].to_vec(),
            negative: false,
        }
    }

    pub fn zero() -> Self {
        Self {
            chunks: [0].to_vec(),
            negative: false,
        }
    }

    fn normalise(&mut self) {
        while self.chunks.len() > 1 && self.chunks.last() == Some(&0) {
            self.chunks.pop();
        }

        // If number is zero, ensure it's not marked as negative
        if self.is_zero() {
            self.negative = false;
        }
    }

    pub fn chunkadd(&self, rhs: &BigNumber) -> BigNumber {
        if self.negative != rhs.negative {
            // -a + b = b - a
            if self.negative {
                return rhs.chunksub(self);
            } else if rhs.negative {
                return self.chunksub(&rhs);
            }
        }

        let max_chunk_len = max(self.chunks.len(), rhs.chunks.len());
        let mut result: Vec<u32> = Vec::with_capacity(max_chunk_len + 1);
        let mut carry = 0u64;

        for i in 0..max_chunk_len {
            let a = if i < self.chunks.len() {
                self.chunks[i] as u64
            } else {
                0
            };
            let b = if i < rhs.chunks.len() {
                rhs.chunks[i] as u64
            } else {
                0
            };

            let sum = a + b + carry;
            result.push((sum % BASE) as u32);
            carry = sum / BASE;
        }

        if carry > 0 {
            result.push(carry as u32); // can never panic
        }

        BigNumber {
            negative: self.negative,
            chunks: result,
        }
    }

    pub fn chunkadd_inplace(&mut self, rhs: &BigNumber) -> BigNumber {
        todo!()
    }

    pub fn chunksub(&self, rhs: &BigNumber) -> BigNumber {
        todo!()
    }

    pub fn chunksub_inplace(&mut self, rhs: &BigNumber) -> BigNumber {
        todo!()
    }
}

// impl traits

impl Add for BigNumber {
    type Output = BigNumber;

    fn add(self, other: Self) -> Self::Output {
        self.chunkadd(&other)
    }
}

macro_rules! impl_commutative {
    (
        impl<$( $lt:lifetime ),*> $trait:ident<$rhs:ty> for $lhs:ty {
            type Output = $output:ty;
            fn $method:ident($self_:ident: $self_ty:ty, $other:ident: $other_ty:ty) -> $ret:ty $body:block
        }
    ) => {
        impl<$($lt),*> $trait<$rhs> for $lhs {
            type Output = $output;

            #[inline]
            fn $method($self_: $self_ty, $other: $other_ty) -> $ret $body

        }

        impl<$($lt),*> $trait<$lhs> for $rhs {
            type Output = $output;

            #[inline]
            fn $method($self_: $other_ty, $other: $self_ty) -> $ret {
                $other.$method($self_)
            }
        }
    };
}

macro_rules! impl_binop {
    (impl $imp:ident, $method:ident for $u:ty, $intmethod:ident) => {
        impl_commutative! {
            impl<'a> $imp<$u> for &'a $u {
                type Output = <$u as $imp<$u>>::Output;

                fn $method(self: &'a $u, other: $u) -> <$u as $imp<$u>>::Output {
                    <$u>::$intmethod(self, &other)
                }
            }
        }

        impl_commutative! {
            impl<'a> $imp<&'a mut $u> for $u {
                type Output = <$u as $imp<$u>>::Output;

                fn $method(self: $u, other: &'a mut $u) -> <$u as $imp<$u>>::Output {
                    <$u>::$intmethod(&self, other)
                }
            }
        }

        impl_commutative! {
            impl<'a, 'b> $imp<&'a mut $u> for &'b $u {
                type Output = <$u as $imp<$u>>::Output;

                fn $method(self: &'b $u, other: &'a mut $u) -> <$u as $imp<$u>>::Output {
                    <$u>::$intmethod(self, other)
                }
            }
        }

        impl<'a, 'b> $imp<&'a $u> for &'b $u {
            type Output = <$u as $imp<$u>>::Output;

            #[inline]
            fn $method(self, other: &'a $u) -> <$u as $imp<$u>>::Output {
                <$u>::$intmethod(self, other)
            }
        }

        impl<'a, 'b> $imp<&'a mut $u> for &'b mut $u {
            type Output = <$u as $imp<$u>>::Output;

            #[inline]
            fn $method(self, other: &'a mut $u) -> <$u as $imp<$u>>::Output {
                <$u>::$intmethod(self, other)
            }
        }
    };
}

impl_binop! { impl Add, add for BigNumber, chunkadd }

macro_rules! impl_bignum_for {
    ($($t:ty)*) => ($(
        impl From<$t> for BigNumber {
            fn from(val: $t) -> Self {
                return BigNumber::new(val as i128);
            }
        }
    )*)
}

impl_bignum_for! { u8 u16 u32 u64 i8 i16 i32 i64 }

#[cfg(test)]
mod bignum_tests {
    use super::*;

    #[test]
    fn test_eq() {
        assert_eq!(BigNumber::from(10), BigNumber::from(10));
        assert_eq!(BigNumber::from(3), BigNumber::from(3));
        assert_eq!(BigNumber::from(u64::MAX), BigNumber::from(u64::MAX));
    }

    #[test]
    fn test_add() {
        assert_eq!(BigNumber::from(1) + BigNumber::from(1), BigNumber::from(2));
        assert_eq!(
            BigNumber::from(u64::MAX) + BigNumber::from(u64::MAX),
            BigNumber::new(u64::MAX as i128 * 2)
        );

        let a = BigNumber::from(123);
        let b = &a;

        assert_eq!(b + BigNumber::one(), BigNumber::from(124));
        assert_eq!(BigNumber::one() + b, BigNumber::from(124));
    }
}

// types for:
//
// add chunks regular
// &[u32], &[u32] -> [u32]
//
// add chunks inplace
// &mut [u32], &[u32] -> None
