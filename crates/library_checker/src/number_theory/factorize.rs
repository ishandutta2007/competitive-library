#[doc(no_inline)]
pub use competitive::math::prime_factors_flatten;
use competitive::prelude::*;

#[verify::library_checker("factorize")]
pub fn factorize(reader: impl Read, mut writer: impl Write) {
    let s = read_all_unchecked(reader);
    let mut scanner = Scanner::new(&s);
    scan!(scanner, q);
    for a in scanner.iter::<u64>().take(q) {
        let x = prime_factors_flatten(a);
        write!(writer, "{}", x.len()).ok();
        for x in x.into_iter() {
            write!(writer, " {}", x).ok();
        }
        writeln!(writer).ok();
    }
}
