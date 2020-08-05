use crate::scan;
use crate::tools::{read_all, Scanner};
use std::io::{Read, Write};

#[verify_attr::verify("https://judge.yosupo.jp/problem/aplusb")]
pub fn aplusb(reader: &mut impl Read, writer: &mut impl Write) {
    let s = read_all(reader);
    let mut scanner = Scanner::new(&s);
    scan!(scanner, a, b);
    writeln!(writer, "{}", a + b).ok();
}
