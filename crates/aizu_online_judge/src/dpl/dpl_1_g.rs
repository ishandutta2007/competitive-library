#[doc(no_inline)]
pub use competitive::combinatorial_optimization::KnapsackProblemSmallWeight;
use competitive::prelude::*;

#[verify::aizu_online_judge("DPL_1_G")]
pub fn dpl_1_g(reader: impl Read, mut writer: impl Write) {
    let s = read_all_unchecked(reader);
    let mut scanner = Scanner::new(&s);
    scan!(scanner, n, w, vwm: [(i64, usize, usize)]);
    let mut knapsack = KnapsackProblemSmallWeight::new(w);
    knapsack.extend_limitation(vwm.take(n));
    writeln!(writer, "{}", knapsack.solve().unwrap_or_default()).ok();
}
