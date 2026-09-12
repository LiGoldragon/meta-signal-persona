# Meta Signal Persona — Agent Instructions

## Purpose

`meta-signal-persona` is the meta policy contract for the Persona engine
manager. It carries the owner's privileged engine-management operations.
Ordinary component lifecycle traffic stays in `signal-persona`; runtime
behavior stays in `persona`.

## Local Rules

- Keep this crate contract-only: no actors, sockets, redb, or daemon loops.
- Change the contract in `ethos/signal.ethos`, then regenerate:
  `ethos-zero 'Generate.{ <abs>/ethos/signal.ethos <abs>/src/generated }'`.
  Commit the generated Rust. Never hand-edit `src/generated/`.
- Import ordinary lifecycle types from `signal-persona` by name. A second
  declaration of `ComponentName`, `ComponentStatus`, `ComponentDesiredState`
  or `EngineIdentifier` here would be a wire fork, not a mirror.
- Pin every LiGoldragon dependency by immutable `rev`. Never by branch.
- `signal-persona` declares `links = "signal-persona"`, so cargo admits
  exactly one revision of it per dependency graph. Every contract in the
  estate must share the same pin.
- Add a round-trip witness in `tests/contract.rs` and a line in
  `examples/canonical.datom` for every new head.

## Protos estate status

Stack: correct-new destination
Status: active component contract, on the Datom stack
