# Architecture

Phase 0 contains only the executable and an integration test. The following are intended responsibilities, not implemented modules. Introduce boundaries when real requirements justify them.

- Providers: authorized HTTP/WebSocket retrieval, provider-specific parsing, connection health, freshness and reconnection.
- Event processing: validate and normalize messages, preserve timestamps, handle duplicates, updates, cancellations and ordering. Namespace identifiers by provider; correlate across providers conservatively and never merge ambiguous events automatically.
- Application state: independent events and reports, predictions, tsunami information, watch locations, selected screen, map position and provider health. State remains separate from rendering.
- TUI: responsive Radar, Alert and other screens, input and map rendering, independent of live provider connections.
- Configuration and storage: local settings and necessary history only; TOML and SQLite are candidates, not commitments. No external SERA server is required by design.
- Notifications: optional, explicit and isolated from simulated events.

Handle missing, delayed, malformed, duplicated and out-of-order data, disconnection and reconnection with visible degradation. Restore terminal state on shutdown. Do not silently hide critical failures. Provider fields, access rights, redistribution and regulatory conditions must be verified before integration. Do not derive observation waveforms from monitoring images or fabricate seismic data. See [design](design.md) and [roadmap](roadmap.md).
