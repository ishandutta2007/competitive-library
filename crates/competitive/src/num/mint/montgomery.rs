use super::*;

impl<M> MIntBase for M
where
    M: MontgomeryReduction32,
{
    type Inner = u32;
    #[inline]
    fn get_mod() -> Self::Inner {
        <Self as MontgomeryReduction32>::get_mod()
    }
    #[inline]
    fn mod_zero() -> Self::Inner {
        0
    }
    #[inline]
    fn mod_one() -> Self::Inner {
        Self::n1()
    }
    #[inline]
    fn mod_add(x: Self::Inner, y: Self::Inner) -> Self::Inner {
        let z = x + y;
        let m = Self::get_mod();
        if z >= m {
            z - m
        } else {
            z
        }
    }
    #[inline]
    fn mod_sub(x: Self::Inner, y: Self::Inner) -> Self::Inner {
        if x < y {
            x + Self::get_mod() - y
        } else {
            x - y
        }
    }
    #[inline]
    fn mod_mul(x: Self::Inner, y: Self::Inner) -> Self::Inner {
        Self::reduce(x as u64 * y as u64)
    }
    #[inline]
    fn mod_div(x: Self::Inner, y: Self::Inner) -> Self::Inner {
        Self::mod_mul(x, Self::mod_inv(y))
    }
    #[inline]
    fn mod_neg(x: Self::Inner) -> Self::Inner {
        if x == 0 {
            0
        } else {
            Self::get_mod() - x
        }
    }
    fn mod_inv(x: Self::Inner) -> Self::Inner {
        let p = Self::get_mod() as i32;
        let (mut a, mut b) = (x as i32, p);
        let (mut u, mut x) = (1, 0);
        while a != 0 {
            let k = b / a;
            x -= k * u;
            b -= k * a;
            std::mem::swap(&mut x, &mut u);
            std::mem::swap(&mut b, &mut a);
        }
        Self::reduce((if x < 0 { x + p } else { x }) as u64 * Self::n3() as u64)
    }
}
impl<M> MIntConvert<u32> for M
where
    M: MontgomeryReduction32,
{
    #[inline]
    fn from(x: u32) -> Self::Inner {
        Self::reduce(x as u64 * Self::n2() as u64)
    }
    #[inline]
    fn into(x: Self::Inner) -> u32 {
        Self::reduce(x as u64)
    }
    #[inline]
    fn mod_into() -> u32 {
        <Self as MIntBase>::get_mod()
    }
}
impl<M> MIntConvert<u64> for M
where
    M: MontgomeryReduction32,
{
    #[inline]
    fn from(x: u64) -> Self::Inner {
        Self::reduce(x % Self::get_mod() as u64 * Self::n2() as u64)
    }
    #[inline]
    fn into(x: Self::Inner) -> u64 {
        Self::reduce(x as u64) as u64
    }
    #[inline]
    fn mod_into() -> u64 {
        <Self as MIntBase>::get_mod() as u64
    }
}
impl<M> MIntConvert<usize> for M
where
    M: MontgomeryReduction32,
{
    #[inline]
    fn from(x: usize) -> Self::Inner {
        Self::reduce(x as u64 % Self::get_mod() as u64 * Self::n2() as u64)
    }
    #[inline]
    fn into(x: Self::Inner) -> usize {
        Self::reduce(x as u64) as usize
    }
    #[inline]
    fn mod_into() -> usize {
        <Self as MIntBase>::get_mod() as usize
    }
}
impl<M> MIntConvert<i32> for M
where
    M: MontgomeryReduction32,
{
    #[inline]
    fn from(x: i32) -> Self::Inner {
        let x = x % <Self as MIntBase>::get_mod() as i32;
        let x = if x < 0 {
            (x + <Self as MIntBase>::get_mod() as i32) as u64
        } else {
            x as u64
        };
        Self::reduce(x * Self::n2() as u64)
    }
    #[inline]
    fn into(x: Self::Inner) -> i32 {
        Self::reduce(x as u64) as i32
    }
    #[inline]
    fn mod_into() -> i32 {
        <Self as MIntBase>::get_mod() as i32
    }
}
impl<M> MIntConvert<i64> for M
where
    M: MontgomeryReduction32,
{
    #[inline]
    fn from(x: i64) -> Self::Inner {
        let x = x % <Self as MIntBase>::get_mod() as i64;
        let x = if x < 0 {
            (x + <Self as MIntBase>::get_mod() as i64) as u64
        } else {
            x as u64
        };
        Self::reduce(x * Self::n2() as u64)
    }
    #[inline]
    fn into(x: Self::Inner) -> i64 {
        Self::reduce(x as u64) as i64
    }
    #[inline]
    fn mod_into() -> i64 {
        <Self as MIntBase>::get_mod() as i64
    }
}
impl<M> MIntConvert<isize> for M
where
    M: MontgomeryReduction32,
{
    #[inline]
    fn from(x: isize) -> Self::Inner {
        let x = x % <Self as MIntBase>::get_mod() as isize;
        let x = if x < 0 {
            (x + <Self as MIntBase>::get_mod() as isize) as u64
        } else {
            x as u64
        };
        Self::reduce(x * Self::n2() as u64)
    }
    #[inline]
    fn into(x: Self::Inner) -> isize {
        Self::reduce(x as u64) as isize
    }
    #[inline]
    fn mod_into() -> isize {
        <Self as MIntBase>::get_mod() as isize
    }
}
/// m is prime, n = 2^32
pub trait MontgomeryReduction32 {
    /// m
    fn get_mod() -> u32;
    /// (-m)^{-1} mod n
    fn r() -> u32 {
        let m = Self::get_mod();
        let mut r = 0;
        let mut t = 0;
        for i in 0..32 {
            if t % 2 == 0 {
                t += m;
                r += 1 << i;
            }
            t /= 2;
        }
        r
    }
    /// n^1 mod m
    fn n1() -> u32;
    /// n^2 mod m
    fn n2() -> u32;
    /// n^3 mod m
    fn n3() -> u32;
    /// n^{-1}x = (x + (xr mod n)m) / n
    fn reduce(x: u64) -> u32 {
        let m: u32 = Self::get_mod();
        let r = Self::r();
        let mut x = ((x + r.wrapping_mul(x as u32) as u64 * m as u64) >> 32) as u32;
        if x >= m {
            x -= m;
        }
        x
    }
}
macro_rules! define_montgomery_reduction_32 {
    ($([$name:ident, $m:expr, $r:expr, $n1:expr, $n2:expr, $n3:expr, $mint_name:ident $(,)?]),* $(,)?) => {
        $(
            pub enum $name {}
            impl MontgomeryReduction32 for $name {
                #[inline]
                fn get_mod() -> u32 {
                    $m
                }
                #[inline]
                fn r() -> u32 {
                    $r
                }
                #[inline]
                fn n1() -> u32 {
                    $n1
                }
                #[inline]
                fn n2() -> u32 {
                    $n2
                }
                #[inline]
                fn n3() -> u32 {
                    $n3
                }
            }
            pub type $mint_name = MInt<$name>;
        )*
    };
}
define_montgomery_reduction_32!(
    [
        Modulo998244353,
        998_244_353,
        998_244_351,
        301_989_884,
        932_051_910,
        679_058_953,
        MInt998244353,
    ],
    [
        Modulo2113929217,
        2_113_929_217,
        2_113_929_215,
        67_108_862,
        2_111_798_781,
        239_209_529,
        MInt2113929217,
    ],
    [
        Modulo1811939329,
        1_811_939_329,
        1_811_939_327,
        671_088_638,
        959_408_210,
        1_483_943_592,
        MInt1811939329,
    ],
    [
        Modulo2013265921,
        2_013_265_921,
        2_013_265_919,
        268_435_454,
        1_172_168_163,
        317_946_875,
        MInt2013265921,
    ],
);

// #[test]
#[allow(dead_code)]
#[codesnip::skip]
fn culculate_montgomery() {
    use crate::math::modinv;
    let m: u64 = 998_244_353;
    println!("m = {}", m);
    let n = 1u64 << 32;
    println!("n = {}", n);
    // n^{-1} mod m
    let ninv = modinv(n, m);
    println!("n^{{-1}} = {}", ninv);
    // r = (-m)^{-1} mod n
    let r = modinv(n - m, n);
    println!("r = {}", r);
    let get_r = || {
        let mut r = 0;
        let mut n = n;
        let mut i = 1;
        let mut t = 0;
        while n > 1 {
            if t % 2 == 0 {
                t += m;
                r += i;
            }
            t /= 2;
            n /= 2;
            i *= 2;
        }
        r
    };
    assert_eq!((get_r() * m + 1) & (n - 1), 0);
    assert_eq!((r * m + 1) & (n - 1), 0);
    assert_eq!(r, get_r());
    let n1 = n % m;
    println!("n^1 = {}", n1);
    let n2 = n1 * n1 % m;
    println!("n^2 = {}", n2);
    let n3 = n1 * n2 % m;
    println!("n^3 = {}", n3);
    // (x + (xr mod n)m) / n
    // r < n
    // xr < xn
    // xr mod n < n
    // (xr mod n)m < mn
    // x + (xr mod n)m < mn + x
    // (x + (xr mod n)m)/n < m + x/n
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::num::montgomery::MInt998244353 as M;
    use crate::tools::Xorshift;

    #[test]
    fn test_mint998244353() {
        let mut rng = Xorshift::new();
        const Q: usize = 1000;
        assert_eq!(0, MInt998244353::zero().inner());
        assert_eq!(1, MInt998244353::one().inner());
        assert_eq!(
            Modulo998244353::reduce(Modulo998244353::n3() as u64),
            Modulo998244353::n2()
        );
        assert_eq!(
            Modulo998244353::reduce(Modulo998244353::n2() as u64),
            Modulo998244353::n1()
        );
        assert_eq!(Modulo998244353::reduce(Modulo998244353::n1() as u64), 1);
        for _ in 0..Q {
            let x = rng.gen(..MInt998244353::get_mod());
            assert_eq!(x, MInt998244353::new(x).inner());
            assert_eq!((-M::new(x)).inner(), (-MInt998244353::new(x)).inner());
            assert_eq!(x, MInt998244353::new(x).inv().inv().inner());
            assert_eq!(M::new(x).inv().inner(), MInt998244353::new(x).inv().inner());
        }

        for _ in 0..Q {
            let x = rng.gen(..MInt998244353::get_mod());
            let y = rng.gen(..MInt998244353::get_mod());
            assert_eq!(
                (M::new(x) + M::new(y)).inner(),
                (MInt998244353::new(x) + MInt998244353::new(y)).inner()
            );
            assert_eq!(
                (M::new(x) - M::new(y)).inner(),
                (MInt998244353::new(x) - MInt998244353::new(y)).inner()
            );
            assert_eq!(
                (M::new(x) * M::new(y)).inner(),
                (MInt998244353::new(x) * MInt998244353::new(y)).inner()
            );
            assert_eq!(
                (M::new(x) / M::new(y)).inner(),
                (MInt998244353::new(x) / MInt998244353::new(y)).inner()
            );
            assert_eq!(
                M::new(x).pow(y as usize).inner(),
                MInt998244353::new(x).pow(y as usize).inner()
            );
        }

        for _ in 0..Q {
            let x = rng.rand64();
            assert_eq!(
                M::from(x as u32).inner(),
                MInt998244353::from(x as u32).inner()
            );
            assert_eq!(M::from(x).inner(), MInt998244353::from(x).inner());
            assert_eq!(
                M::from(x as usize).inner(),
                MInt998244353::from(x as usize).inner()
            );
            assert_eq!(
                M::from(x as i32).inner(),
                MInt998244353::from(x as i32).inner()
            );
            assert_eq!(
                M::from(x as i64).inner(),
                MInt998244353::from(x as i64).inner()
            );
            assert_eq!(
                M::from(x as isize).inner(),
                MInt998244353::from(x as isize).inner()
            );
        }
    }
}
