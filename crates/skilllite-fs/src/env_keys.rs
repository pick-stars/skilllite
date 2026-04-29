//! Mirror of `SKILLLITE_*` env-var names that `skilllite-fs` reads.
//!
//! `skilllite-fs` is a leaf crate that `skilllite-core` depends on, so it
//! cannot import `skilllite_core::config::env_keys`. This module mirrors the
//! one variable currently consumed here so call sites still go through a
//! named `pub const` instead of a magic string.
//!
//! Each entry MUST have an identically-named entry in
//! `skilllite_core::config::env_keys::fs`. The
//! `all_skilllite_env_literals_are_registered` consistency test in
//! `skilllite-core` scans this crate too, so any drift between the two
//! string values (or any new bypass added here) fails CI.

/// Override threshold for the fuzzy `apply_replace_*` matchers. See
/// `search_replace::fuzzy_find`.
pub const SKILLLITE_FUZZY_THRESHOLD: &str = "SKILLLITE_FUZZY_THRESHOLD";
