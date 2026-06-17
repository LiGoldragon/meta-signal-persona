# INTENT — meta-signal-persona

*The meta policy wire contract for privileged Persona engine-manager commands.
Defines the typed request/reply channel that Persona policy uses to launch
and retire engine contexts, query catalog/engine/component status, and start or
stop supervised components.
Companion to `ARCHITECTURE.md` and `Cargo.toml`. Maintenance: `primary/skills/repo-intent.md`.*

## Repo-scope only

This file carries only the intent that is FOR this meta policy `meta-signal-persona`
contract. Workspace-shape intent stays in the primary workspace `primary/INTENT.md`.
Component daemon intent stays in `persona/INTENT.md`. The ordinary
manager-to-supervised-component lifecycle protocol stays in
`signal-persona/INTENT.md`.

## Why this repo exists

`meta-signal-persona` is the **meta policy side of the Persona triad**.
It carries the requests that can change the engine or component lifecycle.
Component-to-component domain contracts stay in their relation-specific
`signal-persona-*` and `meta-signal-persona-*` crates; the ordinary
manager-to-component lifecycle protocol (`Announce`, readiness, health, `Stop`,
`SpawnEnvelope`) lives in `signal-persona`.

## The channel shape

The meta channel is schema-derived from `schema/lib.schema` and carries:

- **Requests:** `Launch(EngineLaunch)` (create a new engine context),
  `Query(Query)` (read catalog, engine status, or component status),
  `Retire(EngineIdentifier)` (retire an engine context),
  `Start(ComponentStartup)` (order a supervised component to run),
  `Stop(ComponentShutdown)` (order a supervised component to stop).
- **Replies:** the closed generated `Output` enumeration matching those
  operations.

`EngineIdentifier` and shared component status records are imported from
`signal-persona`. The generated root types are `Input`, `InputRoute`, `Output`,
`Frame`, `FrameBody`, short-header constants, and signal-frame encode/decode
helpers. `Operation`, `OperationKind`, `Query`, and `Reply` are crate-level
aliases for the meta policy relation.

## Constraints

- Meta policy mutating authority enters through this crate, not through
  `signal-persona`.
- Request payloads do not carry caller identity, timestamps, or minted engine
  identity — those facts are infrastructure-owned and minted at the daemon.
- Wire enums are closed. No `Unknown` escape hatch.
- This crate carries only schema-derived typed wire vocabulary, explicit NOTA
  text codecs for CLI/tooling projection, and round-trip witnesses — no daemon
  actors, persistence, process spawning, socket paths, or CLI parsing.
- Every operation and reply round-trips through both rkyv frames and NOTA text.

## Non-ownership

This crate does not own:

- daemon actors, persistence, process spawning, socket paths, or CLI parsing;
- component-domain traffic (lives in relation-specific `signal-persona-*` crates);
- the ordinary lifecycle protocol (`Announce`, readiness, health, `Stop`,
  `SpawnEnvelope`) — that lives in `signal-persona`.

## See also

- `ARCHITECTURE.md` — wire shape, invariants, and the engine-management boundary.
- `../persona/INTENT.md` — daemon-side engine-manager intent.
- `../signal-persona/ARCHITECTURE.md` — ordinary lifecycle protocol.
- `primary/skills/contract-repo.md` — contract repo discipline and naming rules.
- `primary/skills/component-triad.md` — repo triad structure and authority tiers.
