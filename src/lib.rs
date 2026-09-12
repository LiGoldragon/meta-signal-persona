//! Meta signal contract — privileged Persona engine-manager commands.
//!
//! Ordinary component lifecycle traffic — readiness, health, presence, stop —
//! lives in `signal-persona`. This crate carries the meta plane: the
//! privileged owner operations that launch and retire whole Persona engines,
//! start and stop managed components inside one, and read the engine catalog
//! and status.
//!
//! Ordinary lifecycle types the meta plane quotes — `ComponentName`,
//! `ComponentDesiredState`, `ComponentStatus`, `EngineIdentifier` — are
//! imported from `signal-persona` by name, never redeclared here.

pub mod generated;
pub use generated::signal::*;

pub const ETHOS: &str = include_str!("../ethos/signal.ethos");

/// The portable rkyv Signal frame is one shared type across the estate.
/// A second copy here would be a different Rust type, forking the wire.
pub use signal::{ByteViewable, Restorable, Signal, Signalizable};
