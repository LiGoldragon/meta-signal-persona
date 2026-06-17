# meta-signal-persona — Architecture

`meta-signal-persona` is the meta policy Signal contract for privileged
Persona engine-manager commands.

## Boundary

This crate is the meta policy side of the Persona triad. It carries requests that
can change the engine or component lifecycle:

| Operation | Meaning |
|---|---|
| `Launch(EngineLaunch)` | create a new engine context |
| `Query(Query)` | read catalog, engine status, or component status |
| `Retire(EngineIdentifier)` | retire an engine context |
| `Start(ComponentStartup)` | order a supervised component to run |
| `Stop(ComponentShutdown)` | order a supervised component to stop |

The ordinary manager-to-supervised-component lifecycle protocol lives in
`signal-persona`. That crate carries `Announce`, readiness, health, `Stop`,
and `SpawnEnvelope`.

## Non-Goals

This crate does not own daemon actors, persistence, process spawning, socket
paths, CLI parsing, or component-domain traffic. Component-to-component domain
contracts stay in their relation-specific `signal-persona-*` and
`meta-signal-persona-*` crates.

## Wire Shape

`schema/lib.schema` declares the meta policy operation and reply roots.
`schema-rust-next` emits the `Input` / `InputRoute` / `Output` roots, route
witnesses, short-header constants, frame aliases, and rkyv/NOTA codecs into
`src/schema/lib.rs`. The crate root re-exports that generated surface and keeps
`Operation`, `OperationKind`, `Query`, and `Reply` aliases for the meta policy
relation.

## Invariants

- Meta policy mutating authority enters through this crate, not through
  `signal-persona`.
- Request payloads do not carry caller identity, timestamps, or minted engine
  identity. Those facts are infrastructure-owned.
- Wire enums are closed. There is no `Unknown` escape hatch.
- Round-trip tests cover frame encoding, generated short headers, and NOTA text
  encoding for the meta surface. The crate-local `nota-text` feature maps to
  `signal-frame/nota-text` and `signal-persona/nota-text` so generated policy
  records and imported ordinary Persona records carry text codecs when the text
  witnesses build.

## Emission

`build.rs` runs `schema-rust-next`'s wire-contract driver and imports
`signal-persona`'s schema metadata so shared lifecycle records are defined
once. Regenerate with `META_SIGNAL_PERSONA_UPDATE_SCHEMA_ARTIFACTS=1 cargo
build --all-features` after schema edits. The ordinary `signal-persona`
contract owns shared component names, status records, and engine identifiers;
this meta contract imports them instead of redefining them.

## See Also

- `/git/github.com/LiGoldragon/signal-persona/ARCHITECTURE.md`
