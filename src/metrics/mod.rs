//! Code metrics: line counting, complexity, and aggregation.

pub(crate) mod aggregate;
#[allow(dead_code)] // many functions are tested internally, wired in incrementally
pub(crate) mod complexity;
#[allow(dead_code)] // wired into pipeline when dashboard integrates deps
pub(crate) mod coupling;
#[allow(dead_code)] // wired into pipeline when dashboard integrates deps
pub(crate) mod dependencies;
pub(crate) mod documentation;
pub(crate) mod loc;
