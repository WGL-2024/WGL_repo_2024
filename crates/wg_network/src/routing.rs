use std::collections::Bound;
use std::fmt::{Debug, Formatter};
use std::ops::RangeBounds;

pub type NodeId = u8;

#[derive(Clone)]
pub struct SourceRoutingHeader {
    hop_index: usize, // must be set to 1 initially by the sender
    // Initiator and nodes to which the packet will be forwarded to.
    hops: Vec<NodeId>,
}

/// This prints something like this:
/// 1 -> 2 -> 3 ->(4)-> 5
impl Debug for SourceRoutingHeader {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[ {} ]",
            self.hops
                .iter()
                .enumerate()
                .map(|(i, id)| {
                    if i == self.hop_index {
                        format!("({})", id)
                    } else {
                        format!(" {} ", id)
                    }
                })
                .collect::<Vec<_>>()
                .join("->")
                .trim()
        )
    }
}

impl SourceRoutingHeader {
    pub fn initialize(hops: Vec<NodeId>) -> Self {
        Self { hop_index: 0, hops }
    }
    pub fn with_first_hop(hops: Vec<NodeId>) -> Self {
        Self { hop_index: 1, hops }
    }
    pub fn increase_hop_index(&mut self) {
        self.hop_index += 1;
    }
    pub fn decrease_hop_index(&mut self) {
        self.hop_index -= 1;
    }
    pub fn reset_hop_index(&mut self) {
        self.hop_index = 0;
    }
    pub fn current_hop(&self) -> Option<NodeId> {
        self.hops.get(self.hop_index).cloned()
    }
    pub fn next_hop(&self) -> Option<NodeId> {
        self.hops.get(self.hop_index + 1).cloned()
    }
    pub fn is_last_hop(&self) -> bool {
        self.hop_index == self.hops.len() - 1
    }
    pub fn valid_hop_index(&self) -> bool {
        self.hop_index < self.hops.len()
    }
    pub fn get_hops(&self) -> &Vec<NodeId> {
        &self.hops
    }
    pub fn get_hops_mut(&mut self) -> &mut Vec<NodeId> {
        &mut self.hops
    }
    pub fn get_hop_index(&self) -> usize {
        self.hop_index
    }
    pub fn set_hop_index(&mut self, hop_index: usize) {
        self.hop_index = hop_index;
    }
    pub fn reverse(&mut self) {
        self.hops.reverse();
        self.hop_index = self.hops.len() - self.hop_index - 1;
    }
    pub fn get_reversed(&self) -> SourceRoutingHeader {
        let mut clone = self.clone();
        clone.reverse();
        clone
    }
    pub fn sub_route(&self, range: impl RangeBounds<usize>) -> SourceRoutingHeader {
        let start = match range.start_bound() {
            Bound::Included(&start) => start,
            Bound::Excluded(&start) => start + 1,
            Bound::Unbounded => 0,
        };

        let end = match range.end_bound() {
            Bound::Included(&end) => end + 1,
            Bound::Excluded(&end) => end,
            Bound::Unbounded => self.hops.len(),
        };
        SourceRoutingHeader {
            hop_index: self.hop_index.max(start) - start,
            hops: self.hops[start..end].to_vec(),
        }
    }
}
