pub use crate::combinatorial_optimization::KnapsackPloblemSmallWeight;
use crate::prelude::*;

#[verify_attr::verify("https://onlinejudge.u-aizu.ac.jp/courses/library/7/DPL/1/DPL_1_C")]
pub fn dpl_1_c(reader: &mut impl Read, writer: &mut impl Write) {
    let s = read_all(reader);
    let mut scanner = Scanner::new(&s);
    scan!(scanner, n, w, vw: [(usize, usize); n]);
    let mut knapsack = KnapsackPloblemSmallWeight::new(w);
    knapsack.extend(vw);
    writeln!(writer, "{}", knapsack.solve()).ok();
}