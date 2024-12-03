use std::collections::Bound;
use std::fmt::{Debug, Display, Formatter};
use std::ops::RangeBounds;

pub type NodeId = u8;

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "partial_eq", derive(PartialEq))]
pub struct SourceRoutingHeader {
    pub hop_index: usize, // must be set to 1 initially by the sender
    // Initiator and nodes to which the packet will be forwarded to.
    pub hops: Vec<NodeId>,
}

/// This prints something like this:
/// \[ 1 -> 2 -> 3 ->(4)-> 5 ]
impl Display for SourceRoutingHeader {
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
    // INITIALIZATION
    /// Initializes the route with the given hops.
    /// **The hop index is set to 0.**
    pub fn initialize(hops: Vec<NodeId>) -> Self {
        Self { hop_index: 0, hops }
    }
    /// Initializes the route with the given hops.
    /// **The hop index is set to 1.**
    pub fn with_first_hop(hops: Vec<NodeId>) -> Self {
        Self { hop_index: 1, hops }
    }
    pub fn empty_route() -> Self {
        SourceRoutingHeader::default()
    }

    // HOP INDEX MANIPULATION
    /// Increases the hop index by 1.
    pub fn increase_hop_index(&mut self) {
        self.hop_index += 1;
    }
    /// Decreases the hop index by 1.
    pub fn decrease_hop_index(&mut self) {
        self.hop_index -= 1;
    }
    /// Resets the hop index to 0.
    pub fn reset_hop_index(&mut self) {
        self.hop_index = 0;
    }

    // SPECIAL HOPS
    /// Returns the source node of the route if present.
    pub fn source(&self) -> Option<NodeId> {
        self.hops.first().cloned()
    }
    /// Returns the destination node of the route if present.
    pub fn destination(&self) -> Option<NodeId> {
        self.hops.last().cloned()
    }
    /// Returns the current hop of the route if present.
    pub fn current_hop(&self) -> Option<NodeId> {
        self.hops.get(self.hop_index).cloned()
    }
    /// Returns the next hop of the route if present.
    pub fn next_hop(&self) -> Option<NodeId> {
        self.hops.get(self.hop_index + 1).cloned()
    }
    /// Returns the previous hop of the route if present.
    pub fn previous_hop(&self) -> Option<NodeId> {
        if self.is_first_hop() {
            return None;
        }
        self.hops.get(self.hop_index - 1).cloned()
    }

    // CHECKS
    /// Returns true if the route is empty.
    pub fn is_empty(&self) -> bool {
        self.hops.is_empty()
    }
    /// Returns true if the current hop is the source node.
    pub fn is_first_hop(&self) -> bool {
        !self.is_empty() && self.hop_index == 0
    }
    /// Returns true if the current hop is the destination node.
    pub fn is_last_hop(&self) -> bool {
        !self.is_empty() && self.hop_index == self.hops.len() - 1
    }
    /// Returns true if the hop index is valid.
    pub fn valid_hop_index(&self) -> bool {
        !self.is_empty() && self.hop_index < self.hops.len()
    }

    // HOPS MANIPULATION
    /// Appends a hop to the route.
    pub fn append_hop(&mut self, hop: NodeId) {
        self.hops.push(hop);
    }

    // WHOLE ROUTE MANIPULATION
    /// Reverses the route.
    pub fn reverse(&mut self) {
        if self.is_empty() {
            return;
        }
        self.hops.reverse();
        self.hop_index = self.hops.len() - self.hop_index - 1;
    }
    /// Returns the reversed route.
    pub fn get_reversed(&self) -> SourceRoutingHeader {
        let mut clone = self.clone();
        clone.reverse();
        clone
    }
    /// Extracts a sub-route from the route.
    /// If the range is decreasing, it also reverses the sub-route.
    pub fn sub_route(&self, range: impl RangeBounds<usize>) -> Option<SourceRoutingHeader> {
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

        Some(SourceRoutingHeader {
            hop_index: self.hop_index.max(start) - start,
            hops: self.hops.get(start..end)?.to_vec(),
        })
    }

    // OTHERS
    pub fn len(&self) -> usize {
        self.hops.len()
    }
}
