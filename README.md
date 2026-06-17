# meta-signal-persona

MetaSignal contract for privileged Persona engine-manager commands: engine
launch, catalog query, retirement, component start, and component stop.

This is the meta policy side of Persona's component triad. Ordinary
manager-to-supervised-component lifecycle traffic stays in `signal-persona`.

The contract is generated from `schema/lib.schema`; refresh the checked-in
artifact with `META_SIGNAL_PERSONA_UPDATE_SCHEMA_ARTIFACTS=1 cargo build
--all-features`.
