//! Code metrics: line counting, complexity, and aggregation.

pub(crate) mod aggregate;
#[allow(dead_code)] // many functions are tested internally, wired in incrementally
pub(crate) mod complexity;
#[allow(dead_code)] // wired into pipeline when dashboard integrates deps
pub(crate) mod coupling;
#[allow(dead_code)] // wired into pipeline when dashboard integrates deps
pub(crate) mod dependencies;
pub(crate) mod documentation;
#[allow(dead_code)] // wired into trend/dashboard when git history is displayed
pub(crate) mod git_history;
pub(crate) mod loc;
pub(crate) mod risk;
