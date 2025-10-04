use competitive::prelude::*;
#[doc(no_inline)]
pub use competitive::{algebra::RangeSumRangeAdd, data_structure::LazySegmentTree};

#[verify::aizu_online_judge("DSL_2_G")]
pub fn dsl_2_g(reader: impl Read, mut writer: impl Write) {
    let s = read_all_unchecked(reader);
    let mut scanner = Scanner::new(&s);
    scan!(scanner, n, q);
    let mut seg = LazySegmentTree::<RangeSumRangeAdd<_>>::from_vec(vec![(0, 1); n]);
    for _ in 0..q {
        match scanner.scan::<usize>() {
            0 => {
                scan!(scanner, s, t, x: u64);
                seg.update(s - 1..t, x);
            }
            1 => {
                scan!(scanner, s, t);
                writeln!(writer, "{}", seg.fold(s - 1..t).0).ok();
            }
            _ => unreachable!("unknown query"),
        }
    }
}
