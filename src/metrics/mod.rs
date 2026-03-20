//! Code metrics: line counting, complexity, and aggregation.

pub(crate) mod aggregate;
#[allow(dead_code)] // many functions are tested internally, wired in incrementally
pub(crate) mod complexity;
pub(crate) mod loc;
