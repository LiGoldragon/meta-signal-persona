# INTENT — owner-signal-persona

*The owner-only wire contract for privileged Persona engine-manager commands.
Defines the typed request/reply channel that the Persona owner uses to launch
and retire engine contexts, query catalog/engine/component status, and start or
stop supervised components.
Companion to `ARCHITECTURE.md` and `Cargo.toml`. Maintenance: `primary/skills/repo-intent.md`.*

## Repo-scope only

This file carries only the intent that is FOR this owner-only `owner-signal-persona`
contract. Workspace-shape intent stays in the primary workspace `primary/INTENT.md`.
Component daemon intent stays in `persona/INTENT.md`. The ordinary
manager-to-supervised-component lifecycle protocol stays in
`signal-engine-management/INTENT.md`.

## Why this repo exists

`owner-signal-persona` is the **owner-only policy side of the Persona triad**.
It carries the requests that can change the engine or component lifecycle.
Component-to-component domain contracts stay in their relation-specific
`signal-persona-*` and `owner-signal-persona-*` crates; the ordinary
manager-to-component lifecycle protocol (`Announce`, readiness, health, `Stop`,
`SpawnEnvelope`) lives in `signal-engine-management`.

## The channel shape

The owner channel (`signal_channel! { channel Owner { ... } }`) carries:

- **Requests:** `Launch(EngineLaunch)` (create a new engine context),
  `Query(Query)` (read catalog, engine status, or component status),
  `Retire(EngineIdentifier)` (retire an engine context),
  `Start(ComponentStartup)` (order a supervised component to run),
  `Stop(ComponentShutdown)` (order a supervised component to stop).
- **Replies:** the closed `Reply` enumeration matching those operations.
- **Observations:** the observer stream types emitted by `signal_channel!`.

`EngineIdentifier` is imported from `signal-persona-origin`. The generated root
types are `Operation`, `OperationKind`, `Reply`, `Frame`, `FrameBody`,
`RequestBuilder`, and the observer stream types.

## Constraints

- Owner-only mutating authority enters through this crate, not through
  `signal-engine-management`.
- Request payloads do not carry caller identity, timestamps, or minted engine
  identity — those facts are infrastructure-owned and minted at the daemon.
- Wire enums are closed. No `Unknown` escape hatch.
- This crate carries only typed wire vocabulary, NOTA codecs, and round-trip
  witnesses — no daemon actors, persistence, process spawning, socket paths, or
  CLI parsing.
- Every operation and reply round-trips through both rkyv frames and NOTA text.

## Non-ownership

This crate does not own:

- daemon actors, persistence, process spawning, socket paths, or CLI parsing;
- component-domain traffic (lives in relation-specific `signal-persona-*` crates);
- the ordinary lifecycle protocol (`Announce`, readiness, health, `Stop`,
  `SpawnEnvelope`) — that lives in `signal-engine-management`.

## See also

- `ARCHITECTURE.md` — wire shape, invariants, and the engine-management boundary.
- `../persona/INTENT.md` — daemon-side engine-manager intent.
- `../signal-engine-management/ARCHITECTURE.md` — ordinary lifecycle protocol.
- `../signal-persona-origin/ARCHITECTURE.md` — shared origin/identity vocabulary.
- `primary/skills/contract-repo.md` — contract repo discipline and naming rules.
- `primary/skills/component-triad.md` — repo triad structure and authority tiers.
