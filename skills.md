# skills — meta-signal-persona

Work here when the change concerns the privileged Persona engine-management
contract.

Before editing, read:

- the `ethos` and `datom` skills
- this repo's `ARCHITECTURE.md`
- `../signal-persona/ARCHITECTURE.md`
- `../persona/ARCHITECTURE.md`

Rules:

- Keep the crate contract-only. Do not add runtime code.
- The contract is `ethos/signal.ethos`. Generated Rust is committed and
  freshness-asserted by `build.rs`.
- Import ordinary lifecycle types from `signal-persona`; never redeclare them.
- Add round-trip witnesses and a canonical Datom line for every new head.
