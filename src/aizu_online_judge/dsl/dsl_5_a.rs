use crate::scan;
use crate::tools::{read_all, Scanner};
use std::io::{Read, Write};

#[verify_attr::verify("https://onlinejudge.u-aizu.ac.jp/courses/library/3/DSL/5/DSL_5_A")]
pub fn dsl_5_a(reader: &mut impl Read, writer: &mut impl Write) {
    let s = read_all(reader);
    let mut scanner = Scanner::new(&s);
    scan!(scanner, n, t, lr: [(usize, usize); n]);
    let mut acc = vec![0; t + 1];
    for (l, r) in lr {
        acc[l] += 1;
        acc[r] -= 1;
    }
    for i in 0..t {
        acc[i + 1] += acc[i];
    }
    writeln!(writer, "{}", acc.into_iter().max().unwrap_or_default()).ok();
}