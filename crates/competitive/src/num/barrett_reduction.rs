#[derive(Debug, Clone, Copy)]
pub struct BarrettReduction<T> {
    m: T,
    im: T,
}
macro_rules! impl_barrett {
    ($basety:ty, |$a:ident, $im:ident| $quotient:expr) => {
        impl BarrettReduction<$basety> {
            pub const fn new(m: $basety) -> Self {
                Self { m, im: !0 / m }
            }
            pub const fn get_mod(&self) -> $basety {
                self.m
            }
            pub const fn div_rem(&self, $a: $basety) -> ($basety, $basety) {
                if self.m == 1 {
                    return ($a, 0);
                }
                let $im = self.im;
                let mut q = $quotient;
                let mut r = $a - q * self.m;
                if self.m <= r {
                    r -= self.m;
                    q += 1;
                }
                (q, r)
            }
            pub const fn div(&self, a: $basety) -> $basety {
                self.div_rem(a).0
            }
            pub const fn rem(&self, a: $basety) -> $basety {
                self.div_rem(a).1
            }
        }
    };
}
impl_barrett!(u32, |a, im| ((a as u64 * im as u64) >> 32) as u32);
impl_barrett!(u64, |a, im| ((a as u128 * im as u128) >> 64) as u64);
impl_barrett!(u128, |a, im| {
    const MASK64: u128 = 0xffff_ffff_ffff_ffff;
    let au = a >> 64;
    let ad = a & MASK64;
    let imu = im >> 64;
    let imd = im & MASK64;
    let mut res = au * imu;
    let x = (ad * imd) >> 64;
    let (x, c) = x.overflowing_add(au * imd);
    res += c as u128;
    let (x, c) = x.overflowing_add(ad * imu);
    res += c as u128;
    res + (x >> 64)
});

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tools::Xorshift;

    macro_rules! test_barrett {
        ($test_name:ident, $ty:ty, |$rng:ident| $res:expr) => {
            #[test]
            fn $test_name() {
                let mut $rng = Xorshift::default();
                const Q: usize = 10_000;
                for _ in 0..Q {
                    let (a, b): ($ty, $ty) = $res;
                    let barrett = BarrettReduction::<$ty>::new(b);
                    assert_eq!(a / b, barrett.div(a));
                    assert_eq!(a % b, barrett.rem(a));
                }
            }
        };
    }
    test_barrett!(test_barrett_u32_small, u32, |rng| (
        rng.random(..=100),
        rng.random(1..=100)
    ));
    test_barrett!(test_barrett_u64_small, u64, |rng| (
        rng.random(..=100),
        rng.random(1..=100)
    ));
    test_barrett!(test_barrett_u128_small, u128, |rng| {
        (
            rng.random(..=100u64) as u128 * rng.random(..=100u64) as u128,
            rng.random(1..=100u64) as u128 * rng.random(1..=100u64) as u128,
        )
    });

    test_barrett!(test_barrett_u32_large, u32, |rng| (
        rng.random(..=!0),
        rng.random(1..=!0)
    ));
    test_barrett!(test_barrett_u64_large, u64, |rng| (
        rng.random(..=!0),
        rng.random(1..=!0)
    ));
    test_barrett!(test_barrett_u128_large, u128, |rng| {
        (
            rng.random(..=!0u64) as u128 * rng.random(..=!0u64) as u128,
            rng.random(1..=!0u64) as u128 * rng.random(1..=!0u64) as u128,
        )
    });

    test_barrett!(test_barrett_u32_max, u32, |rng| (
        rng.random(!0 - 100..=!0),
        rng.random(!0 - 100..=!0)
    ));
    test_barrett!(test_barrett_u64_max, u64, |rng| (
        rng.random(!0 - 100..=!0),
        rng.random(!0 - 100..=!0)
    ));
    test_barrett!(test_barrett_u128_max, u128, |rng| {
        (
            rng.random(!0 - 100..=!0u64) as u128 * rng.random(!0 - 100..=!0u64) as u128,
            rng.random(!0 - 100..=!0u64) as u128 * rng.random(!0 - 100..=!0u64) as u128,
        )
    });

    test_barrett!(test_barrett_u128_mul, u128, |rng| {
        (
            rng.random(0u64..) as u128 * rng.random(0u64..) as u128,
            rng.random(0u64..) as u128,
        )
    });
}
