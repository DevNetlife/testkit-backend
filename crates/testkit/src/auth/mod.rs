//! Systematically prove that privileged entry points reject unauthorized
//! callers — the most common Soroban vulnerability class.
//!
//! [`AuthMatrix`] registers each privileged entry point with the addresses
//! allowed to call it, then cross-checks every **known actor** — any address
//! named in at least one entry point's `allowed` list — against every entry
//! point. The definition of a known actor is documented on [`AuthMatrix`] and
//! restated in the failure report so the coverage it implies is never
//! ambiguous.
//!
//! [`AuthMatrix::assert_enforced`] panics on the first run that has any
//! violating cell; [`AuthMatrix::report`] runs the same cells in report-only
//! mode and returns a [`MatrixReport`] of [`CellResult`]s instead. Every cell
//! runs with isolated authorization state, and results are always ordered by
//! entry-point registration, then known-actor first-seen order.
//!
//! # Named actor fixtures
//!
//! For the handful of role shapes that appear in nearly every contract,
//! this module ships pre-built actor sets that remove the address-generation
//! boilerplate:
//!
//! | Fixture | Actors | Use when |
//! |---|---|---|
//! | [`OwnerActors`] | owner + attacker | single privileged owner |
//! | [`AdminUserActors`] | admin + user + outsider | two-tier access control |
//! | [`RbacActors`] | admin + operator + viewer + outsider | three-tier RBAC |
//! | [`PeerActors`] | N equal peers + outsider | multi-party / multisig |

mod fixtures;
mod matrix;

pub use fixtures::{unauthorized_callers, AdminUserActors, OwnerActors, PeerActors, RbacActors};
pub use matrix::{AuthMatrix, CellOutcome, CellResult, MatrixReport};
