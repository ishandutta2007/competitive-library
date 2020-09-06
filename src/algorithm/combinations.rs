/// choose `r` elements from `n` independently
///
/// # Example
///
/// ```
/// # use competitive::algorithm::product;
/// let n = vec![1, 2, 3, 4];
/// let mut p = Vec::new();
/// let mut q = Vec::new();
/// product(n.as_slice(), 2, |v| p.push(v.to_vec()));
/// for x in n.iter().cloned() {
///     for y in n.iter().cloned() {
///         q.push(vec![x, y]);
///     }
/// }
/// assert_eq!(p, q);
/// ```
#[cargo_snippet::snippet]
pub fn product<T: Clone, F>(n: &[T], r: usize, mut f: F)
where
    F: FnMut(&[T]),
{
    fn product_inner<T: Clone, F>(n: &[T], mut r: usize, v: &mut Vec<T>, f: &mut F)
    where
        F: FnMut(&[T]),
    {
        if r == 0 {
            f(v.as_slice());
        } else {
            r -= 1;
            for a in n.iter().cloned() {
                v.push(a);
                product_inner(n, r, v, f);
                v.pop();
            }
        }
    }

    if r <= n.len() {
        let mut v = Vec::with_capacity(r);
        product_inner(n, r, &mut v, &mut f);
    }
}

/// choose distinct `r` elements from `n` in any order
///
/// # Example
///
/// ```
/// # use competitive::algorithm::permutations;
/// let n = vec![1, 2, 3, 4];
/// let mut p = Vec::new();
/// let mut q = Vec::new();
/// permutations(n.as_slice(), 2, |v| p.push(v.to_vec()));
/// for (i, x) in n.iter().cloned().enumerate() {
///     for (j, y) in n.iter().cloned().enumerate() {
///         if i != j {
///             q.push(vec![x, y]);
///         }
///     }
/// }
/// assert_eq!(p, q);
/// ```
#[cargo_snippet::snippet]
pub fn permutations<T: Clone, F>(n: &[T], r: usize, mut f: F)
where
    F: FnMut(&[T]),
{
    use std::collections::BTreeSet;
    fn permutations_inner<T: Clone, F>(
        n: &[T],
        mut r: usize,
        rem: &mut BTreeSet<usize>,
        v: &mut Vec<T>,
        f: &mut F,
    ) where
        F: FnMut(&[T]),
    {
        if r == 0 {
            f(v.as_slice());
        } else {
            r -= 1;
            for i in rem.iter().cloned().collect::<Vec<_>>() {
                v.push(n[i].clone());
                rem.remove(&i);
                permutations_inner(n, r, rem, v, f);
                rem.insert(i);
                v.pop();
            }
        }
    }

    if r <= n.len() {
        let mut v = Vec::with_capacity(r);
        let mut rem: BTreeSet<usize> = (0..n.len()).collect();
        permutations_inner(n, r, &mut rem, &mut v, &mut f);
    }
}

/// choose distinct `r` elements from `n` in sorted order
///
/// # Example
///
/// ```
/// # use competitive::algorithm::combinations;
/// let n = vec![1, 2, 3, 4];
/// let mut p = Vec::new();
/// let mut q = Vec::new();
/// combinations(n.as_slice(), 2, |v| p.push(v.to_vec()));
/// for (i, x) in n.iter().cloned().enumerate() {
///     for y in n[i+1..].iter().cloned() {
///         q.push(vec![x, y]);
///     }
/// }
/// assert_eq!(p, q);
/// ```
#[cargo_snippet::snippet]
pub fn combinations<T: Clone, F>(n: &[T], r: usize, mut f: F)
where
    F: FnMut(&[T]),
{
    fn combinations_inner<T: Clone, F>(n: &[T], mut r: usize, p: usize, v: &mut Vec<T>, f: &mut F)
    where
        F: FnMut(&[T]),
    {
        if r == 0 {
            f(v.as_slice());
        } else {
            r -= 1;
            for i in p..n.len() - r {
                v.push(n[i].clone());
                combinations_inner(n, r, i + 1, v, f);
                v.pop();
            }
        }
    }

    if r <= n.len() {
        let mut v = Vec::with_capacity(r);
        combinations_inner(n, r, 0, &mut v, &mut f);
    }
}

/// choose `r` elements from `n` in sorted order
///
/// # Example
///
/// ```
/// # use competitive::algorithm::combinations_with_replacement;
/// let n = vec![1, 2, 3, 4];
/// let mut p = Vec::new();
/// let mut q = Vec::new();
/// combinations_with_replacement(n.as_slice(), 2, |v| p.push(v.to_vec()));
/// for (i, x) in n.iter().cloned().enumerate() {
///     for y in n[i..].iter().cloned() {
///         q.push(vec![x, y]);
///     }
/// }
/// assert_eq!(p, q);
/// ```
#[cargo_snippet::snippet]
pub fn combinations_with_replacement<T: Clone, F>(n: &[T], r: usize, mut f: F)
where
    F: FnMut(&[T]),
{
    fn combinations_with_replacement_inner<T: Clone, F>(
        n: &[T],
        mut r: usize,
        p: usize,
        v: &mut Vec<T>,
        f: &mut F,
    ) where
        F: FnMut(&[T]),
    {
        if r == 0 {
            f(v.as_slice());
        } else {
            r -= 1;
            for i in p..n.len() {
                v.push(n[i].clone());
                combinations_with_replacement_inner(n, r, i, v, f);
                v.pop();
            }
        }
    }

    if r <= n.len() {
        let mut v = Vec::with_capacity(r);
        combinations_with_replacement_inner(n, r, 0, &mut v, &mut f);
    }
}
