//! Code metrics: line counting, complexity, and aggregation.

pub(crate) mod aggregate;
#[allow(dead_code)] // wired into pipeline when dashboard integrates complexity
pub(crate) mod complexity;
pub(crate) mod loc;
