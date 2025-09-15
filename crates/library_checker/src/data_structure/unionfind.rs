#[doc(no_inline)]
pub use competitive::data_structure::UnionFind;
use competitive::prelude::*;

#[verify::library_checker("unionfind")]
pub fn unionfind(reader: impl Read, mut writer: impl Write) {
    let s = read_all_unchecked(reader);
    let mut scanner = Scanner::new(&s);
    scan!(scanner, n, q);
    let mut uf = UnionFind::new(n);
    for _ in 0..q {
        scan!(scanner, ty, u, v);
        if ty == 0 {
            uf.unite(u, v);
        } else {
            writeln!(writer, "{}", uf.same(u, v) as usize).ok();
        }
    }
}
