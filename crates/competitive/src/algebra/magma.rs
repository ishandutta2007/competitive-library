//! algebraic traits

/// binary operaion: $T \circ T \to T$
pub trait Magma {
    /// type of operands: $T$
    type T: Clone;
    /// binary operaion: $\circ$
    fn operate(x: &Self::T, y: &Self::T) -> Self::T;
    #[inline]
    fn reverse_operate(x: &Self::T, y: &Self::T) -> Self::T {
        Self::operate(y, x)
    }
    #[inline]
    fn operate_assign(x: &mut Self::T, y: &Self::T) {
        *x = Self::operate(x, y);
    }
}

/// $\forall a,\forall b,\forall c \in T, (a \circ b) \circ c = a \circ (b \circ c)$
pub trait Associative {}

/// associative binary operation
pub trait SemiGroup: Magma + Associative {}

impl<S: Magma + Associative> SemiGroup for S {}

/// $\exists e \in T, \forall a \in T, e \circ a = a \circ e = e$
pub trait Unital: Magma {
    /// identity element: $e$
    fn unit() -> Self::T;
    #[inline]
    fn is_unit(x: &Self::T) -> bool
    where
        <Self as Magma>::T: PartialEq,
    {
        x == &Self::unit()
    }
    #[inline]
    fn set_unit(x: &mut Self::T) {
        *x = Self::unit();
    }
}

/// associative binary operation and an identity element
pub trait Monoid: SemiGroup + Unital {
    /// binary exponentiation: $x^n = x\circ\ddots\circ x$
    fn pow(x: Self::T, n: usize) -> Self::T {
        let mut n = n;
        let mut res = Self::unit();
        let mut base = x;
        while n > 0 {
            if n & 1 == 1 {
                res = Self::operate(&res, &base);
            }
            base = Self::operate(&base, &base);
            n >>= 1;
        }
        res
    }
}

impl<M: SemiGroup + Unital> Monoid for M {}

/// $\exists e \in T, \forall a \in T, \exists b,c \in T, b \circ a = a \circ c = e$
pub trait Invertible: Magma {
    /// $a$ where $a \circ x = e$
    fn inverse(x: &Self::T) -> Self::T;
    #[inline]
    fn rinv_operate(x: &Self::T, y: &Self::T) -> Self::T {
        Self::operate(x, &Self::inverse(y))
    }
}

/// associative binary operation and an identity element and inverse elements
pub trait Group: Monoid + Invertible {}

impl<G: Monoid + Invertible> Group for G {}

/// $\forall a,\forall b \in T, a \circ b = b \circ a$
pub trait Commutative {}

/// commutative monoid
pub trait AbelianMonoid: Monoid + Commutative {}

impl<M: Monoid + Commutative> AbelianMonoid for M {}

/// commutative group
pub trait AbelianGroup: Group + Commutative {}

impl<G: Group + Commutative> AbelianGroup for G {}

/// $\forall a \in T, a \circ a = a$
pub trait Idempotent {}

/// idempotent monoid
pub trait IdempotentMonoid: Monoid + Idempotent {}

impl<M: Monoid + Idempotent> IdempotentMonoid for M {}

#[macro_export]
macro_rules! monoid_fold {
    ($m:ty) => { <$m as Unital>::unit() };
    ($m:ty,) => { <$m as Unital>::unit() };
    ($m:ty, $f:expr) => { $f };
    ($m:ty, $f:expr, $($ff:expr),*) => { <$m as Magma>::operate(&($f), &monoid_fold!($m, $($ff),*)) };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algebra::operations::MaxOperation;

    #[test]
    #[allow(clippy::eq_op)]
    fn test_monoid_fold() {
        assert_eq!(monoid_fold!(MaxOperation<u32>,), 0);
        assert_eq!(monoid_fold!(MaxOperation<u32>, 1), 1);
        assert_eq!(monoid_fold!(MaxOperation<u32>, 1, 2), 2);
        assert_eq!(monoid_fold!(MaxOperation<u32>, 0, 1, 5, 2), 5);
    }
}
