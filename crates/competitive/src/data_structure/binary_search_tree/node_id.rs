use super::{BstNode, BstNodeRef, BstRoot, BstSpec, node};
use std::{
    collections::HashSet,
    hash::{Hash, Hasher},
    ptr::NonNull,
    sync::atomic::{self, AtomicU64},
};

pub struct BstNodeId<Spec>
where
    Spec: BstSpec,
{
    node: NonNull<BstNode<Spec::Data, Spec::Parent>>,
    generation: u64,
}

impl<Spec> Clone for BstNodeId<Spec>
where
    Spec: BstSpec,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<Spec> Copy for BstNodeId<Spec> where Spec: BstSpec {}

impl<Spec> PartialEq for BstNodeId<Spec>
where
    Spec: BstSpec,
{
    fn eq(&self, other: &Self) -> bool {
        self.node == other.node && self.generation == other.generation
    }
}

impl<Spec> Eq for BstNodeId<Spec> where Spec: BstSpec {}

impl<Spec> Hash for BstNodeId<Spec>
where
    Spec: BstSpec,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.node.hash(state);
        self.generation.hash(state);
    }
}

impl<Spec> BstNodeId<Spec>
where
    Spec: BstSpec,
{
    pub unsafe fn reborrow<'a>(
        self,
        _root: &'a Option<BstRoot<Spec>>,
    ) -> BstNodeRef<node::marker::Immut<'a>, Spec> {
        unsafe { BstNodeRef::new_unchecked(self.node) }
    }

    pub unsafe fn reborrow_datamut<'a>(
        self,
        _root: &'a mut Option<BstRoot<Spec>>,
    ) -> BstNodeRef<node::marker::DataMut<'a>, Spec> {
        unsafe { BstNodeRef::new_unchecked(self.node) }
    }

    pub unsafe fn reborrow_mut<'a>(
        self,
        _root: &'a mut Option<BstRoot<Spec>>,
    ) -> BstNodeRef<node::marker::Mut<'a>, Spec> {
        unsafe { BstNodeRef::new_unchecked(self.node) }
    }
}

static GENERATION: AtomicU64 = AtomicU64::new(0);

pub struct BstNodeIdManager<Spec>
where
    Spec: BstSpec,
{
    node_ids: HashSet<BstNodeId<Spec>>,
}

impl<Spec> Default for BstNodeIdManager<Spec>
where
    Spec: BstSpec,
{
    fn default() -> Self {
        Self {
            node_ids: Default::default(),
        }
    }
}

impl<Spec> BstNodeIdManager<Spec>
where
    Spec: BstSpec,
{
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_empty(&self) -> bool {
        self.node_ids.is_empty()
    }

    pub fn len(&self) -> usize {
        self.node_ids.len()
    }

    pub fn contains(&self, node_id: &BstNodeId<Spec>) -> bool {
        self.node_ids.contains(node_id)
    }

    pub fn clear(&mut self) {
        self.node_ids.clear();
    }

    pub fn register(&mut self, node: &BstRoot<Spec>) -> BstNodeId<Spec> {
        let node_id = BstNodeId {
            node: node.node,
            generation: GENERATION
                .fetch_update(atomic::Ordering::Relaxed, atomic::Ordering::Relaxed, |x| {
                    x.checked_add(1)
                })
                .expect("Generation counter overflow"),
        };
        self.node_ids.insert(node_id);
        node_id
    }

    pub fn unregister(&mut self, node_id: BstNodeId<Spec>) {
        self.node_ids.remove(&node_id);
    }
}
