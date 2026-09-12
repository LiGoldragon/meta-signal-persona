# meta-signal-persona

Meta signal contract for privileged Persona engine-manager commands.

The meta-only wire contract for `persona` — the second leg of the
two-contract pair (`signal-persona` ordinary + `meta-signal-persona` meta).
The ordinary plane carries component lifecycle traffic: readiness, health,
presence, stop. This meta plane carries the owner's privileged operations —
`Launch` and `Retire` of whole engines, `Start` and `Stop` of managed
components inside one, and `Query` for component status, engine status and
the engine catalog.

Ordinary lifecycle types the meta plane quotes — `ComponentName`,
`ComponentDesiredState`, `ComponentStatus`, `EngineIdentifier` — are imported
from `signal-persona` by name, never redeclared here.

The contract is authored in `ethos/signal.ethos`; `ethos-zero` generates
`src/generated/signal.rs`, which is committed, and `build.rs` asserts the
committed generation matches a fresh one. The wire is binary rkyv; the
optional `datom` feature adds Datom text projection. See `ARCHITECTURE.md`.
