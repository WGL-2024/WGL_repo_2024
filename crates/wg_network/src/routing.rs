use std::fmt::{Debug, Formatter};
use std::ops::Range;

pub type NodeId = u8;

#[derive(Clone)]
pub struct SourceRoutingHeader {
    pub hop_index: usize, // must be set to 1 initially by the sender
    // Initiator and nodes to which the packet will be forwarded to.
    pub hops: Vec<NodeId>,
}

impl Debug for SourceRoutingHeader {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
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
                .join("->").trim()
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
    pub fn current_hop(&self) -> Option<NodeId> {
        self.hops.get(self.hop_index).cloned()
    }
    pub fn next_hop(&self) -> Option<NodeId> {
        self.hops.get(self.hop_index + 1).cloned()
    }
    pub fn is_last_hop(&self) -> bool {
        self.hop_index == self.hops.len() - 1
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
    pub fn sub_route(&self, range: Range<usize>) -> SourceRoutingHeader {
        SourceRoutingHeader {
            hop_index: self.hop_index - range.start,
            hops: self.hops[range].to_vec(),
        }
    }
}
