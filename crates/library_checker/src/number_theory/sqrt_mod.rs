#[doc(no_inline)]
pub use competitive::num::mint_basic::DynMIntU32;
use competitive::prelude::*;

#[verify::library_checker("sqrt_mod")]
pub fn sqrt_mod(reader: impl Read, mut writer: impl Write) {
    let s = read_all_unchecked(reader);
    let mut scanner = Scanner::new(&s);
    scan!(scanner, q, yp: [(u32, u32)]);
    for (y, p) in yp.take(q) {
        DynMIntU32::set_mod(p);
        if let Some(x) = DynMIntU32::from(y).sqrt() {
            writeln!(writer, "{}", x).ok();
        } else {
            writeln!(writer, "-1").ok();
        }
    }
}
