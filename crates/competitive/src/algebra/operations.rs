//! binary operaions

use super::magma::*;
use crate::num::{Bounded, One, Zero};

/// binary operation to select larger element
#[codesnip::entry("MaxOperation", include("algebra", "bounded"))]
pub struct MaxOperation<T: Clone + Ord + Bounded> {
    _marker: std::marker::PhantomData<fn() -> T>,
}
#[codesnip::entry("MaxOperation")]
mod max_operation_impl {
    use super::*;
    impl<T: Clone + Ord + Bounded> Magma for MaxOperation<T> {
        type T = T;
        #[inline]
        fn operate(x: &Self::T, y: &Self::T) -> Self::T {
            x.max(y).clone()
        }
    }
    impl<T: Clone + Ord + Bounded> Unital for MaxOperation<T> {
        #[inline]
        fn unit() -> Self::T {
            <T as Bounded>::minimum()
        }
    }
    impl<T: Clone + Ord + Bounded> Associative for MaxOperation<T> {}
    impl<T: Clone + Ord + Bounded> Commutative for MaxOperation<T> {}
    impl<T: Clone + Ord + Bounded> Idempotent for MaxOperation<T> {}
}

/// binary operation to select smaller element
#[codesnip::entry("MinOperation", include("algebra", "bounded"))]
pub struct MinOperation<T: Clone + Ord + Bounded> {
    _marker: std::marker::PhantomData<fn() -> T>,
}
#[codesnip::entry("MinOperation")]
mod min_operation_impl {
    use super::*;
    impl<T: Clone + Ord + Bounded> Magma for MinOperation<T> {
        type T = T;
        #[inline]
        fn operate(x: &Self::T, y: &Self::T) -> Self::T {
            x.min(y).clone()
        }
    }
    impl<T: Clone + Ord + Bounded> Unital for MinOperation<T> {
        #[inline]
        fn unit() -> Self::T {
            <T as Bounded>::maximum()
        }
    }
    impl<T: Clone + Ord + Bounded> Associative for MinOperation<T> {}
    impl<T: Clone + Ord + Bounded> Commutative for MinOperation<T> {}
    impl<T: Clone + Ord + Bounded> Idempotent for MinOperation<T> {}
}

/// retain the first element
#[codesnip::entry("FirstOperation", include("algebra"))]
pub struct FirstOperation<T: Clone> {
    _marker: std::marker::PhantomData<fn() -> T>,
}
#[codesnip::entry("FirstOperation")]
mod first_operation_impl {
    use super::*;
    impl<T: Clone> Magma for FirstOperation<T> {
        type T = Option<T>;
        #[inline]
        fn operate(x: &Self::T, y: &Self::T) -> Self::T {
            x.as_ref().or_else(|| y.as_ref()).cloned()
        }
    }
    impl<T: Clone> Unital for FirstOperation<T> {
        #[inline]
        fn unit() -> Self::T {
            None
        }
    }
    impl<T: Clone> Associative for FirstOperation<T> {}
    impl<T: Clone> Idempotent for FirstOperation<T> {}
}

/// retain the last element
#[codesnip::entry("LastOperation", include("algebra"))]
pub struct LastOperation<T: Clone> {
    _marker: std::marker::PhantomData<fn() -> T>,
}
#[codesnip::entry("LastOperation")]
mod last_operation_impl {
    use super::*;
    impl<T: Clone> Magma for LastOperation<T> {
        type T = Option<T>;
        #[inline]
        fn operate(x: &Self::T, y: &Self::T) -> Self::T {
            y.as_ref().or_else(|| x.as_ref()).cloned()
        }
    }
    impl<T: Clone> Unital for LastOperation<T> {
        #[inline]
        fn unit() -> Self::T {
            None
        }
    }
    impl<T: Clone> Associative for LastOperation<T> {}
    impl<T: Clone> Idempotent for LastOperation<T> {}
}

/// $+$
#[codesnip::entry("AdditiveOperation", include("algebra", "zero_one"))]
pub struct AdditiveOperation<T: Clone + Zero + std::ops::Add<Output = T>> {
    _marker: std::marker::PhantomData<fn() -> T>,
}
#[codesnip::entry("AdditiveOperation")]
mod additive_operation_impl {
    use super::*;
    use std::ops::{Add, Neg, Sub};
    impl<T: Clone + Zero + Add<Output = T>> Magma for AdditiveOperation<T> {
        type T = T;
        #[inline]
        fn operate(x: &Self::T, y: &Self::T) -> Self::T {
            x.clone() + y.clone()
        }
    }
    impl<T: Clone + Zero + Add<Output = T>> Unital for AdditiveOperation<T> {
        #[inline]
        fn unit() -> Self::T {
            Zero::zero()
        }
    }
    impl<T: Clone + Zero + Add<Output = T>> Associative for AdditiveOperation<T> {}
    impl<T: Clone + Zero + Add<Output = T>> Commutative for AdditiveOperation<T> {}
    impl<T: Clone + Zero + Add<Output = T> + Sub<Output = T> + Neg<Output = T>> Invertible
        for AdditiveOperation<T>
    {
        #[inline]
        fn inverse(x: &Self::T) -> Self::T {
            -x.clone()
        }
        #[inline]
        fn rinv_operate(x: &Self::T, y: &Self::T) -> Self::T {
            x.clone() - y.clone()
        }
    }
}

/// $\times$
#[codesnip::entry("MultiplicativeOperation", include("algebra", "zero_one"))]
pub struct MultiplicativeOperation<T: Clone + One + std::ops::Mul<Output = T>> {
    _marker: std::marker::PhantomData<fn() -> T>,
}
#[codesnip::entry("MultiplicativeOperation")]
mod multiplicative_operation_impl {
    use super::*;
    use std::ops::{Div, Mul};
    impl<T: Clone + One + Mul<Output = T>> Magma for MultiplicativeOperation<T> {
        type T = T;
        #[inline]
        fn operate(x: &Self::T, y: &Self::T) -> Self::T {
            x.clone() * y.clone()
        }
    }
    impl<T: Clone + One + Mul<Output = T>> Unital for MultiplicativeOperation<T> {
        #[inline]
        fn unit() -> Self::T {
            One::one()
        }
    }
    impl<T: Clone + One + Mul<Output = T>> Associative for MultiplicativeOperation<T> {}
    impl<T: Clone + One + Mul<Output = T>> Commutative for MultiplicativeOperation<T> {}
    impl<T: Clone + One + Mul<Output = T> + Div<Output = T>> Invertible for MultiplicativeOperation<T> {
        #[inline]
        fn inverse(x: &Self::T) -> Self::T {
            Self::unit().div(x.clone())
        }
        #[inline]
        fn rinv_operate(x: &Self::T, y: &Self::T) -> Self::T {
            (x.clone()).div(y.clone())
        }
    }
}

/// $(a, b) \circ (c, d) = \lambda x. c \times (a \times x + b) + d$
#[codesnip::entry("LinearOperation", include("algebra", "zero_one"))]
pub struct LinearOperation<
    T: Clone + Zero + std::ops::Add<Output = T> + One + std::ops::Mul<Output = T>,
> {
    _marker: std::marker::PhantomData<fn() -> T>,
}
#[codesnip::entry("LinearOperation")]
mod linear_operation_impl {
    use super::*;
    use std::ops::{Add, Div, Mul, Neg, Sub};
    impl<T: Clone + Zero + One + Add<Output = T> + Mul<Output = T>> Magma for LinearOperation<T> {
        type T = (T, T);
        #[inline]
        fn operate(x: &Self::T, y: &Self::T) -> Self::T {
            (
                y.0.clone() * x.0.clone(),
                y.0.clone() * x.1.clone() + y.1.clone(),
            )
        }
    }
    impl<T: Clone + Zero + One + Add<Output = T> + Mul<Output = T>> Unital for LinearOperation<T> {
        #[inline]
        fn unit() -> Self::T {
            (One::one(), Zero::zero())
        }
    }
    impl<T: Clone + Zero + One + Add<Output = T> + Mul<Output = T>> Associative for LinearOperation<T> {}
    impl<
            T: Clone
                + Zero
                + One
                + Add<Output = T>
                + Sub<Output = T>
                + Neg<Output = T>
                + Mul<Output = T>
                + Div<Output = T>,
        > Invertible for LinearOperation<T>
    {
        fn inverse(x: &Self::T) -> Self::T {
            let y = <T as One>::one().div(x.0.clone());
            (y.clone(), -y.mul(x.1.clone()))
        }
    }
}

/// &
#[codesnip::entry("BitAndOperation", include("algebra"))]
pub struct BitAndOperation<T: Clone + BitAndIdentity> {
    _marker: std::marker::PhantomData<fn() -> T>,
}
#[codesnip::entry("BitAndOperation")]
pub trait BitAndIdentity: Sized + std::ops::BitAnd<Output = Self> {
    fn all_one() -> Self;
}
#[codesnip::entry("BitAndOperation")]
mod bitand_operation_impl {
    use super::*;
    #[macro_export]
    macro_rules! impl_bitand_identity {
        ([$($wh:tt)*], $t:ty, $all_one:expr) => {
            impl<$($wh)*> BitAndIdentity for $t {
                #[inline]
                fn all_one() -> Self {
                    $all_one
                }
            }
        };
        ($t:ty, $all_one:expr) => {
            impl BitAndIdentity for $t {
                #[inline]
                fn all_one() -> Self {
                    $all_one
                }
            }
        };
    }
    impl_bitand_identity!(bool, true);
    impl_bitand_identity!(usize, std::usize::MAX);
    impl_bitand_identity!(u8, std::u8::MAX);
    impl_bitand_identity!(u16, std::u16::MAX);
    impl_bitand_identity!(u32, std::u32::MAX);
    impl_bitand_identity!(u64, std::u64::MAX);
    impl_bitand_identity!(isize, std::isize::MIN);
    impl_bitand_identity!(i8, std::i8::MIN);
    impl_bitand_identity!(i16, std::i16::MIN);
    impl_bitand_identity!(i32, std::i32::MIN);
    impl_bitand_identity!(i64, std::i64::MIN);
    impl<T: Clone + BitAndIdentity> Magma for BitAndOperation<T> {
        type T = T;
        #[inline]
        fn operate(x: &Self::T, y: &Self::T) -> Self::T {
            x.clone() & y.clone()
        }
    }
    impl<T: Clone + BitAndIdentity> Unital for BitAndOperation<T> {
        #[inline]
        fn unit() -> Self::T {
            BitAndIdentity::all_one()
        }
    }
    impl<T: Clone + BitAndIdentity> Associative for BitAndOperation<T> {}
    impl<T: Clone + BitAndIdentity> Commutative for BitAndOperation<T> {}
    impl<T: Clone + BitAndIdentity> Idempotent for BitAndOperation<T> {}
}

/// |
#[codesnip::entry("BitOrOperation", include("algebra"))]
pub struct BitOrOperation<T: Clone + BitOrIdentity> {
    _marker: std::marker::PhantomData<fn() -> T>,
}
#[codesnip::entry("BitOrOperation")]
pub trait BitOrIdentity: Sized + std::ops::BitOr<Output = Self> {
    fn all_zero() -> Self;
}
#[codesnip::entry("BitOrOperation")]
mod bitor_operation_impl {
    use super::*;
    #[macro_export]
    macro_rules! impl_bitor_identity {
        ([$($wh:tt)*], $t:ty, $all_zero:expr) => {
            impl<$($wh)*> BitOrIdentity for $t {
                #[inline]
                fn all_zero() -> Self {
                    $all_zero
                }
            }
        };
        ($t:ty, $all_zero:expr) => {
            impl BitOrIdentity for $t {
                #[inline]
                fn all_zero() -> Self {
                    $all_zero
                }
            }
        };
    }
    impl_bitor_identity!(bool, false);
    impl_bitor_identity!(usize, 0usize);
    impl_bitor_identity!(u8, 0u8);
    impl_bitor_identity!(u16, 0u16);
    impl_bitor_identity!(u32, 0u32);
    impl_bitor_identity!(u64, 0u64);
    impl_bitor_identity!(isize, 0isize);
    impl_bitor_identity!(i8, 0i8);
    impl_bitor_identity!(i16, 0i16);
    impl_bitor_identity!(i32, 0i32);
    impl_bitor_identity!(i64, 0i64);
    impl<T: Clone + BitOrIdentity> Magma for BitOrOperation<T> {
        type T = T;
        #[inline]
        fn operate(x: &Self::T, y: &Self::T) -> Self::T {
            x.clone() | y.clone()
        }
    }
    impl<T: Clone + BitOrIdentity> Unital for BitOrOperation<T> {
        #[inline]
        fn unit() -> Self::T {
            BitOrIdentity::all_zero()
        }
    }
    impl<T: Clone + BitOrIdentity> Associative for BitOrOperation<T> {}
    impl<T: Clone + BitOrIdentity> Commutative for BitOrOperation<T> {}
    impl<T: Clone + BitOrIdentity> Idempotent for BitOrOperation<T> {}
}

/// ^
#[codesnip::entry("BitXorOperation", include("algebra"))]
pub struct BitXorOperation<T: Clone + BitXorIdentity> {
    _marker: std::marker::PhantomData<fn() -> T>,
}
#[codesnip::entry("BitXorOperation")]
pub trait BitXorIdentity: Sized + std::ops::BitXor<Output = Self> {
    fn xor_zero() -> Self;
}
#[codesnip::entry("BitXorOperation")]
mod bitxor_operation_impl {
    use super::*;
    #[macro_export]
    macro_rules !impl_bitxor_identity {([$($wh :tt ) *] ,$t :ty ,$xor_zero :expr ) =>{impl <$($wh ) *>BitXorIdentity for $t {#[inline ] fn xor_zero () ->Self {$xor_zero } } } ;($t :ty ,$xor_zero :expr ) =>{impl BitXorIdentity for $t {#[inline ] fn xor_zero () ->Self {$xor_zero } } } ;}
    impl_bitxor_identity!(bool, false);
    impl_bitxor_identity!(usize, 0usize);
    impl_bitxor_identity!(u8, 0u8);
    impl_bitxor_identity!(u16, 0u16);
    impl_bitxor_identity!(u32, 0u32);
    impl_bitxor_identity!(u64, 0u64);
    impl_bitxor_identity!(isize, 0isize);
    impl_bitxor_identity!(i8, 0i8);
    impl_bitxor_identity!(i16, 0i16);
    impl_bitxor_identity!(i32, 0i32);
    impl_bitxor_identity!(i64, 0i64);
    impl<T: Clone + BitXorIdentity> Magma for BitXorOperation<T> {
        type T = T;
        #[inline]
        fn operate(x: &Self::T, y: &Self::T) -> Self::T {
            x.clone() ^ y.clone()
        }
    }
    impl<T: Clone + BitXorIdentity> Unital for BitXorOperation<T> {
        #[inline]
        fn unit() -> Self::T {
            BitXorIdentity::xor_zero()
        }
    }
    impl<T: Clone + BitXorIdentity> Associative for BitXorOperation<T> {}
    impl<T: Clone + BitXorIdentity> Commutative for BitXorOperation<T> {}
    impl<T: Clone + BitXorIdentity> Invertible for BitXorOperation<T> {
        fn inverse(x: &Self::T) -> Self::T {
            x.clone()
        }
    }
}

#[codesnip::entry("TupleOperation", include("algebra"))]
mod tuple_operation_impl {
    #![allow(unused_variables)]
    use super::*;
    macro_rules! impl_tuple_operation {
        ($($M:ident)*, $($i:tt)*) => {
            impl<$($M: Magma),*> Magma for ($($M,)*) {
                type T = ($(<$M as Magma>::T,)*);
                #[inline]
                fn operate(x: &Self::T, y: &Self::T) -> Self::T {
                    ($(<$M as Magma>::operate(&x.$i, &y.$i),)*)
                }
            }
            impl<$($M: Unital),*> Unital for ($($M,)*) {
                #[inline]
                fn unit() -> Self::T {
                    ($(<$M as Unital>::unit(),)*)
                }
            }
            impl<$($M: Associative),*> Associative for ($($M,)*) {}
            impl<$($M: Commutative),*> Commutative for ($($M,)*) {}
            impl<$($M: Idempotent),*> Idempotent for ($($M,)*) {}
            impl<$($M: Invertible),*> Invertible for ($($M,)*) {
                #[inline]
                fn inverse(x: &Self::T) -> Self::T {
                    ($(<$M as Invertible>::inverse(&x.$i),)*)
                }
            }
        };
    }
    impl_tuple_operation!(,);
    impl_tuple_operation!(A, 0);
    impl_tuple_operation!(A B, 0 1);
    impl_tuple_operation!(A B C, 0 1 2);
    impl_tuple_operation!(A B C D, 0 1 2 3);
    impl_tuple_operation!(A B C D E, 0 1 2 3 4);
    impl_tuple_operation!(A B C D E F, 0 1 2 3 4 5);
    impl_tuple_operation!(A B C D E F G, 0 1 2 3 4 5 6);
    impl_tuple_operation!(A B C D E F G H, 0 1 2 3 4 5 6 7);
    impl_tuple_operation!(A B C D E F G H I, 0 1 2 3 4 5 6 7 8);
    impl_tuple_operation!(A B C D E F G H I J, 0 1 2 3 4 5 6 7 8 9);
}

#[codesnip::entry("CountingOperation", include("algebra"))]
pub struct CountingOperation<M> {
    _marker: std::marker::PhantomData<fn() -> M>,
}
#[codesnip::entry("CountingOperation")]
mod counting_operation_impl {
    use super::*;
    impl<M: Magma> Magma for CountingOperation<M>
    where
        M::T: PartialEq,
    {
        type T = (M::T, usize);
        #[inline]
        fn operate(x: &Self::T, y: &Self::T) -> Self::T {
            if x.0 == y.0 {
                (x.0.clone(), x.1 + y.1)
            } else {
                let z = M::operate(&x.0, &y.0);
                if z == x.0 {
                    (z, x.1)
                } else if z == y.0 {
                    (z, y.1)
                } else {
                    (z, 1)
                }
            }
        }
    }
    impl<M: Unital> Unital for CountingOperation<M>
    where
        M::T: PartialEq,
    {
        #[inline]
        fn unit() -> Self::T {
            (M::unit(), 0)
        }
    }
    impl<M: Associative> Associative for CountingOperation<M> {}
    impl<M: Commutative> Commutative for CountingOperation<M> {}
    impl<M: Idempotent> Idempotent for CountingOperation<M> {}
}

#[codesnip::entry("ReverseOperation", include("algebra"))]
pub struct ReverseOperation<M> {
    _marker: std::marker::PhantomData<fn() -> M>,
}
#[codesnip::entry("ReverseOperation")]
mod reverse_operation_impl {
    use super::*;
    impl<M: Magma> Magma for ReverseOperation<M> {
        type T = M::T;
        #[inline]
        fn operate(x: &Self::T, y: &Self::T) -> Self::T {
            M::operate(&y, &x)
        }
    }
    impl<M: Unital> Unital for ReverseOperation<M> {
        #[inline]
        fn unit() -> Self::T {
            M::unit()
        }
    }
    impl<M: Associative> Associative for ReverseOperation<M> {}
    impl<M: Commutative> Commutative for ReverseOperation<M> {}
    impl<M: Invertible> Invertible for ReverseOperation<M> {
        #[inline]
        fn inverse(x: &Self::T) -> Self::T {
            M::inverse(&x)
        }
    }
    impl<M: Idempotent> Idempotent for ReverseOperation<M> {}
}

#[codesnip::entry("Top2Operation", include("algebra", "bounded"))]
pub struct Top2Operation<T: Clone + Ord + Bounded> {
    _marker: std::marker::PhantomData<fn() -> T>,
}
#[codesnip::entry("Top2Operation")]
mod top2_operation_impl {
    use super::*;
    impl<T: Clone + Ord + Bounded> Magma for Top2Operation<T> {
        type T = (T, T);
        fn operate(x: &Self::T, y: &Self::T) -> Self::T {
            if x.0 < y.0 {
                (y.0.clone(), if x.0 < y.1 { &y.1 } else { &x.0 }.clone())
            } else {
                (x.0.clone(), if x.1 < y.0 { &y.0 } else { &x.1 }.clone())
            }
        }
    }
    impl<T: Clone + Ord + Bounded> Unital for Top2Operation<T> {
        fn unit() -> Self::T {
            (<T as Bounded>::minimum(), <T as Bounded>::minimum())
        }
    }
    impl<T: Clone + Ord + Bounded> Associative for Top2Operation<T> {}
    impl<T: Clone + Ord + Bounded> Commutative for Top2Operation<T> {}
}

#[codesnip::entry("Bottom2Operation", include("algebra", "bounded"))]
pub struct Bottom2Operation<T: Clone + Ord + Bounded> {
    _marker: std::marker::PhantomData<fn() -> T>,
}
#[codesnip::entry("Bottom2Operation")]
mod bottom2_operation_impl {
    use super::*;
    impl<T: Clone + Ord + Bounded> Magma for Bottom2Operation<T> {
        type T = (T, T);
        fn operate(x: &Self::T, y: &Self::T) -> Self::T {
            if x.0 > y.0 {
                (y.0.clone(), if x.0 > y.1 { &y.1 } else { &x.0 }.clone())
            } else {
                (x.0.clone(), if x.1 > y.0 { &y.0 } else { &x.1 }.clone())
            }
        }
    }
    impl<T: Clone + Ord + Bounded> Unital for Bottom2Operation<T> {
        fn unit() -> Self::T {
            (<T as Bounded>::maximum(), <T as Bounded>::maximum())
        }
    }
    impl<T: Clone + Ord + Bounded> Associative for Bottom2Operation<T> {}
    impl<T: Clone + Ord + Bounded> Commutative for Bottom2Operation<T> {}
}

#[codesnip::entry("PermutationOperation", include("algebra"))]
pub struct PermutationOperation;
#[codesnip::entry("PermutationOperation")]
mod permutation_operation_impl {
    use super::*;
    impl Magma for PermutationOperation {
        type T = Vec<usize>;
        fn operate(x: &Self::T, y: &Self::T) -> Self::T {
            y.iter()
                .map(|&y| if y < x.len() { x[y] } else { y })
                .collect()
        }
    }
    impl Associative for PermutationOperation {}
    impl Unital for PermutationOperation {
        fn unit() -> Self::T {
            Vec::new()
        }
    }
    impl Invertible for PermutationOperation {
        fn inverse(x: &Self::T) -> Self::T {
            let mut y = vec![0; x.len()];
            for (i, x) in x.iter().enumerate() {
                y[*x] = i;
            }
            y
        }
    }
}

#[codesnip::entry("FindMajorityOperation", include("algebra"))]
/// Find majority(strict) of a sequence.
///
/// fold $x \in S$ with `(Some(x), 1)`
///
/// `(Some(m), _)` represents `m` may be a majority of $S$.
///
/// `(None, _)` represents that there is no majority value.
pub struct FindMajorityOperation<T> {
    _marker: std::marker::PhantomData<fn() -> T>,
}
#[codesnip::entry("FindMajorityOperation")]
mod find_majority_operation_impl {
    use super::*;
    impl<T: Clone + Eq> Magma for FindMajorityOperation<T> {
        type T = (Option<T>, usize);
        fn operate(x: &Self::T, y: &Self::T) -> Self::T {
            if y.0.is_none() {
                x.clone()
            } else if x.0.is_none() {
                y.clone()
            } else {
                match (x.0.eq(&y.0), x.1.cmp(&y.1)) {
                    (true, _) => (x.0.clone(), x.1 + y.1),
                    (_, std::cmp::Ordering::Less) => (y.0.clone(), y.1 - x.1),
                    (_, std::cmp::Ordering::Equal) => (None, 0),
                    (_, std::cmp::Ordering::Greater) => (x.0.clone(), x.1 - y.1),
                }
            }
        }
    }
    impl<T: Clone + Eq> Unital for FindMajorityOperation<T> {
        fn unit() -> Self::T {
            (None, 0)
        }
    }
    impl<T> Associative for FindMajorityOperation<T> {}
}
