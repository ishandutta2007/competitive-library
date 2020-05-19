//! algebraic traits

/// binary operaion: T ∘ T → T
#[cargo_snippet::snippet("algebra")]
pub trait Magma {
    /// type of operands: T
    type T: Clone + PartialEq;
    /// binary operaion: ∘
    fn operate(&self, x: &Self::T, y: &Self::T) -> Self::T;
}

/// ∀a,b,c ∈ T, (a ∘ b) ∘ c = a ∘ (b ∘ c)
#[cargo_snippet::snippet("algebra")]
pub trait Associative {}

/// associative binary operation
#[cargo_snippet::snippet("algebra")]
pub trait SemiGroup: Magma + Associative {}

/// ∃e ∈ T, ∀a ∈ T, e ∘ a = a ∘ e = e
#[cargo_snippet::snippet("algebra")]
pub trait Unital: Magma {
    /// identity element: e
    fn unit(&self) -> Self::T;
}

/// associative binary operation and an identity element
#[cargo_snippet::snippet("algebra")]
pub trait Monoid: SemiGroup + Unital {
    /// x ^ n = x ∘ ... ∘ x
    /// binary exponentiation
    fn pow(&self, x: Self::T, n: usize) -> Self::T {
        let mut n = n;
        let mut res = self.unit();
        let mut base = x;
        while n > 0 {
            if n & 1 == 1 {
                res = self.operate(&res, &base);
            }
            base = self.operate(&base, &base);
            n = n >> 1;
        }
        res
    }
}

/// ∃e ∈ T, ∀a ∈ T, ∃b,c ∈ T, b ∘ a = a ∘ c = e
#[cargo_snippet::snippet("algebra")]
pub trait Invertible: Magma {
    /// a where a ∘ x = e
    fn inverse(&self, x: &Self::T) -> Self::T;
}

/// associative binary operation and an identity element and inverse elements
#[cargo_snippet::snippet("algebra")]
pub trait Group: Monoid + Invertible {}

/// ∀a,b ∈ T, a ∘ b = b ∘ a
#[cargo_snippet::snippet("algebra")]
pub trait Commutative {}

/// commutative monoid
#[cargo_snippet::snippet("algebra")]
pub trait AbelianMonoid: Monoid + Commutative {}

/// commutative group
#[cargo_snippet::snippet("algebra")]
pub trait AbelianGroup: Group + Commutative {}

/// ∀a ∈ T, a ∘ a = a
#[cargo_snippet::snippet("algebra")]
pub trait Idempotent {}

/// idempotent monoid
#[cargo_snippet::snippet("algebra")]
pub trait IdempotentMonoid: Monoid + Idempotent {}
