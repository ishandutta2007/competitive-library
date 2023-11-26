use super::EdgeListGraph;
use crate::algebra::Group;
use crate::data_structure::{MergingUnionFind, UnionFind};

#[codesnip::entry("minimum_spanning_tree", include("EdgeListGraph", "UnionFind"))]
impl EdgeListGraph {
    pub fn minimum_spanning_tree<T>(&self, weight: impl Fn(&usize) -> T) -> Vec<bool>
    where
        T: Ord,
    {
        let mut idx: Vec<_> = (0..self.edges_size()).collect();
        idx.sort_by_key(weight);
        let mut uf = UnionFind::new(self.vertices_size());
        let mut res = vec![false; self.edges_size()];
        for eid in idx.into_iter() {
            let (u, v) = self[eid];
            res[eid] = uf.unite(u, v);
        }
        res
    }
}

#[codesnip::entry(
    "minimum_spanning_arborescence",
    include("algebra", "EdgeListGraph", "UnionFind")
)]
impl EdgeListGraph {
    /// tarjan
    pub fn minimum_spanning_arborescence<G, F>(
        &self,
        root: usize,
        weight: F,
    ) -> Option<(G::T, Vec<usize>)>
    where
        G: Group,
        G::T: Ord,
        F: Fn(usize) -> G::T,
    {
        use std::{cmp::Reverse, collections::BinaryHeap};
        let mut uf = MergingUnionFind::new_with_merger(
            self.vertices_size(),
            |_| (BinaryHeap::new(), G::unit()),
            |x, y| {
                let ny = G::rinv_operate(&y.1, &x.1);
                x.0.extend(
                    (y.0)
                        .drain()
                        .map(|(Reverse(ref w), i)| (Reverse(G::operate(w, &ny)), i)),
                )
            },
        );
        let mut state = vec![0; self.vertices_size()]; // 0: unprocessed, 1: in process, 2: completed
        state[root] = 2;
        for (id, &(_, to)) in self.edges().enumerate() {
            uf.merge_data_mut(to).0.push((Reverse(weight(id)), id));
        }
        let mut paredge = vec![0; self.edges_size()];
        let mut ord = vec![];
        let mut leaf = vec![self.edges_size(); self.vertices_size()];
        let mut cycle = 0usize;
        let mut acc = G::unit();
        for mut cur in self.vertices() {
            if state[cur] != 0 {
                continue;
            }
            let mut path = vec![];
            let mut ch = vec![];
            while state[cur] != 2 {
                path.push(cur);
                state[cur] = 1;
                let (w, eid) = {
                    let (heap, lazy) = &mut uf.merge_data_mut(cur);
                    match heap.pop() {
                        Some((Reverse(w), eid)) => (G::operate(&w, lazy), eid),
                        None => return None,
                    }
                };
                {
                    let curw = &mut uf.merge_data_mut(cur).1;
                    *curw = G::rinv_operate(curw, &w);
                }
                acc = G::operate(&acc, &w);
                ord.push(eid);
                let (u, v) = self[eid];
                if leaf[v] >= self.edges_size() {
                    leaf[v] = eid;
                }
                while cycle > 0 {
                    paredge[ch.pop().unwrap()] = eid;
                    cycle -= 1;
                }
                ch.push(eid);
                if state[uf.find_root(u)] == 1 {
                    while let Some(t) = path.pop() {
                        state[t] = 2;
                        cycle += 1;
                        if !uf.unite(u, t) {
                            break;
                        }
                    }
                    state[uf.find_root(u)] = 1;
                }
                cur = uf.find_root(u);
            }
            for u in path.into_iter() {
                state[u] = 2;
            }
        }
        let mut tree = vec![root; self.vertices_size()];
        let mut used = vec![false; self.edges_size()];
        for eid in ord.into_iter().rev() {
            if !used[eid] {
                let (u, v) = self[eid];
                tree[v] = u;
                let mut x = leaf[v];
                while x != eid {
                    used[x] = true;
                    x = paredge[x];
                }
            }
        }
        Some((acc, tree))
    }
}
