//! SigmaOS-awk — Text processing language model.
//!
//! Models awk pattern-action language: field splitting,
//! pattern matching, actions, record processing. Deterministic and std-only.

pub mod record;
pub mod pattern;
pub mod action;
pub mod program;

pub use record::{Record, Field};
pub use pattern::{AwkPattern, MatchCondition};
pub use action::{AwkAction, PrintAction};
pub use program::{AwkProgram, Rule};
