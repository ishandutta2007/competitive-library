use std::{collections::HashMap, hash::Hash};

/// $P_0 =$ `init`, $P_{i+1} = next(P_i)$
///
/// if |T| is finite then P = f, g, g, ...
#[derive(Debug)]
pub struct RhoPath<T> {
    pub f: Vec<T>,
    pub g: Vec<T>,
}
impl<T> RhoPath<T> {
    /// build rho path
    pub fn build<F>(init: T, next: F) -> Self
    where
        T: Clone + Eq + Hash,
        F: Fn(&T) -> T,
    {
        let mut path = vec![init.clone()];
        let mut visited = HashMap::new();
        visited.insert(init, 0);
        let loop_start = loop {
            let next_val = next(path.last().unwrap());
            if let Some(&idx) = visited.get(&next_val) {
                break idx;
            }
            let cnt = path.len();
            path.push(next_val.clone());
            visited.insert(next_val, cnt);
        };
        let looped = path.split_off(loop_start);
        Self { f: path, g: looped }
    }
    /// rho path that index of rho path
    pub fn build_rho<F>(&self, init: usize, next: F) -> RhoPath<usize>
    where
        F: Fn(&usize) -> usize,
    {
        let (n, m) = (self.f.len(), self.g.len());
        RhoPath::build(init, |x| {
            let y = next(x);
            if y < n { y } else { (y - n) % m + n }
        })
    }
    /// get i-th value of rho path
    pub fn get(&self, index: usize) -> &T {
        if index < self.f.len() {
            &self.f[index]
        } else {
            &self.g[(index - self.f.len()) % self.g.len()]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tools::Xorshift;

    #[test]
    fn test_rho_path() {
        let mut rng = Xorshift::default();
        for _ in 0..100 {
            let n = rng.random(1..100);
            let a: Vec<usize> = rng.random_iter(0..n).take(n).collect();
            let rp = RhoPath::build(0, |&x| a[x]);
            let mut x = 0;
            for i in 0..n * 2 {
                assert_eq!(rp.get(i), &x);
                x = a[x];
            }

            let rp2 = rp.build_rho(0, |&x| x + a[*rp.get(x)]);
            let mut x = 0;
            for i in 0..n * 2 {
                assert_eq!(rp.get(*rp2.get(i)), rp.get(x));
                x += a[*rp.get(x)];
            }
        }
    }
}
