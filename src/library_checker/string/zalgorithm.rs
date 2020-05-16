pub use crate::string::z_algorithm::Zarray;
pub use crate::tools::scanner::{read_all, Scanner};
use std::io::{self, Read, Write};

#[verify_attr::verify("https://judge.yosupo.jp/problem/zalgorithm")]
pub fn zalgorithm(reader: &mut impl Read, writer: &mut impl Write) -> io::Result<()> {
    let s = read_all(reader);
    let mut scanner = Scanner::new(&s);
    let s = scanner.scan_chars();
    let z = Zarray::new(&s);
    for i in 0..s.len() {
        write!(writer, "{}{}", if i == 0 { "" } else { " " }, z[i])?;
    }
    writeln!(writer, "")?;
    Ok(())
}
