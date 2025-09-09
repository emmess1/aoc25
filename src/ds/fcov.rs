//! Functional coverage helper for tests.
//!
//! This module provides a tiny, test-only facility to record which named
//! behaviors have been exercised. It is intentionally simple: tests call
//! `hit("behavior_id")` and a meta-test can assert that all `EXPECTED`
//! behaviors were observed.
//!
//! Notes
//! - This is not a replacement for code coverage; it is a complement that
//!   checks behavioral requirements are touched by tests.
//! - Thread-safe and process-local; good enough for typical Rust test runs.

use std::collections::HashSet;
use std::sync::{Mutex, OnceLock};

// Global registry storing the set of covered behavior identifiers.
static REGISTRY: OnceLock<Mutex<HashSet<&'static str>>> = OnceLock::new();

fn reg() -> &'static Mutex<HashSet<&'static str>> {
    REGISTRY.get_or_init(|| Mutex::new(HashSet::new()))
}

/// Mark a functional behavior as covered (id must be a string literal).
pub fn hit(id: &'static str) {
    let _ = reg().lock().map(|mut s| {
        s.insert(id);
    });
}

/// Return a snapshot of covered ids.
pub fn snapshot() -> HashSet<&'static str> {
    reg().lock().map(|s| s.clone()).unwrap_or_default()
}

/// The list of expected functional behavior ids we aim to cover.
pub const EXPECTED: &[&str] = &[
    // LinkedList
    "ll_empty_new",
    "ll_push_pop_order",
    "ll_clear_idempotent",
    // SimpleHashMap
    "hm_insert_new",
    "hm_update_existing",
    "hm_collision_chain",
    "hm_resize",
    "hm_remove_missing",
    // BstMap
    "bst_insert",
    "bst_get_mut_miss_left",
    "bst_get_mut_miss_right",
    "bst_remove_leaf",
    "bst_remove_one_left",
    "bst_remove_one_right",
    "bst_remove_two_children_immediate_succ",
    "bst_remove_two_children_succ_with_right",
];

/// Return which expected ids are still missing.
pub fn missing() -> Vec<&'static str> {
    let snap = snapshot();
    EXPECTED.iter().copied().filter(|id| !snap.contains(id)).collect()
}

/// True if all expected behaviors were hit.
pub fn all_hit() -> bool { missing().is_empty() }
