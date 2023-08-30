//! The module implements a datastructure that aggregates a "forest" into less
//! nodes. For any node one can ask for a single aggregated version of all
//! children on that node. Changes the the forest will propagate up the
//! aggregation tree to keep it up to date. So asking of an aggregated
//! information is cheap and one can even wait for aggregated info to change.
//!
//! The aggregation will try to reuse aggregated nodes on every level to reduce
//! memory and cpu usage of propagating changes. The tree structure is designed
//! for multi-thread usage.
//!
//! The aggregation tree is build out of two halfs. The top tree and the bottom
//! tree. One node of the bottom tree can aggregate items of connectivity
//! 2^height. It will do that by having bottom trees of height - 1 as children.
//! One node of the top tree can aggregate items of any connectivity. It will do
//! that by having a bottom tree of height = depth as a child and top trees of
//! depth + 1 as children. So it's basically a linked list of bottom trees of
//! increasing height. Any top or bottom node can be shared between multiple
//! parents.
//!
//! Notations:
//! - parent/child: Relationship in the original forest resp. the aggregated
//!   version of the relationships.
//! - upper: Relationship to a aggregated node in a higher level (more
//!   aggregated). Since all communication is strictly upwards there is no down
//!   relationship for that.

mod bottom_tree;
mod inner_refs;
mod leaf;
#[cfg(test)]
mod tests;
mod top_tree;

use std::hash::Hash;

use self::leaf::top_tree;
pub use self::{leaf::AggregationTreeLeaf, top_tree::AggregationInfoGuard};

pub trait AggregationContext {
    type ItemLock: AggregationItemLock<Context = Self>;
    type Info;
    type ItemChange;
    type ItemRef: Eq + Hash + Clone;

    fn new_info() -> Self::Info;

    fn item(&self, reference: Self::ItemRef) -> Self::ItemLock;

    fn apply_change(
        &self,
        info: &mut Self::Info,
        change: &Self::ItemChange,
    ) -> Option<Self::ItemChange>;

    fn info_to_add_change(&self, info: &Self::Info) -> Option<Self::ItemChange>;
    fn info_to_remove_change(&self, info: &Self::Info) -> Option<Self::ItemChange>;
}

pub trait AggregationItemLock {
    type Context: AggregationContext;
    type ChildrenIter<'a>: Iterator<Item = <Self::Context as AggregationContext>::ItemRef> + 'a
    where
        Self: 'a;
    fn leaf(&mut self) -> &mut AggregationTreeLeaf<Self::Context>;
    fn children(&self) -> Self::ChildrenIter<'_>;
    fn is_blue(&self) -> bool;
    fn get_remove_change(&self) -> Option<<Self::Context as AggregationContext>::ItemChange>;
    fn get_add_change(&self) -> Option<<Self::Context as AggregationContext>::ItemChange>;
}

pub fn aggregation_info<T: AggregationContext>(
    context: &T,
    reference: T::ItemRef,
) -> AggregationInfoGuard<T> {
    let mut item = context.item(reference);
    let top_tree = top_tree(context, &mut item, 0);
    top_tree.info()
}