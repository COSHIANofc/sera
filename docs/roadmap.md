# Roadmap

Phase 0 was completed and committed locally; hosted CI remains unverified. Later milestones are planned and need separate authorization. CDL is published and applied; project rights-holder identification and third-party review remain outstanding.

| Milestone | Status | Scope and completion gate |
|---|---|---|
| Phase 0 | Completed locally | Minimal executable and test, toolchain, CI, dependency policy, architecture, design and roadmap. Hosted platform checks remain unverified. |
| v0.0.1 UI prototype | Planned | Ratatui/Crossterm Radar and Alert, labeled simulation, manual and simulated automatic transitions, countdown and location UI prototypes, resize, Japanese text, snapshots and terminal restoration. Both layouts and critical alerts must remain readable; no live API without separate authorization. |
| v0.1.0 Radar | Planned | Authorized earthquake provider, map, epicenters, observed intensity, history, locations, persistence, connection and freshness, reconnection. Verify data rights and geography; mandatory tests remain offline. |
| v0.2.0 Alert | Planned | Validated EEW, classification, screen transitions, regional/maximum predictions, provider arrival estimates, report updates, cancellation, final and stale states, multiple events and locations; controllable clock tests. Never fabricate estimates or interpret expiry as safety. |
| v0.3.0 Visualization | Planned | Verified boundaries and JMA mappings, zoom/pan/tracking, authorized P/S estimates and observation points, performance and map modes. Defer waves if permissions or reliable data are unavailable. |
| v0.4.0 Tsunami | Planned | Major warnings, warnings, advisories, coastal regions, arrival, height, authorized observations, updates and lifting, concurrent alerts; unknown is not safe. |
| v0.5.0 Advanced monitoring | Planned | Authorized strong motion and waveforms, replay, simulation, audio/OS notifications, views, headless operation, JSON and optional webhooks. Isolate simulation and document output. |
| v0.6.0 Reliability | Planned | Measure CPU/memory/rendering, long-running stability, recovery, failover and conservative correlation, SSH/tmux, terminal restoration and security audit. Set targets from measurements. |
| v0.9.0 Release preparation | Planned | Integration, platform and terminal testing, security, dependency and data rights, CDL and regulatory review, accessibility, user and installation documentation, artifact and corresponding-source verification. Use actual SemVer prerelease identifiers such as `-rc.1`. |
| v1.0.0 Stable | Planned | Only with explicit authorization: cross-platform artifacts, release notes, checksums, source, licenses, notices and installation guidance; consider Homebrew, AUR, WinGet and Scoop later. |

Continuous delivery, package manifests and website are deferred. Candidate libraries include Ratatui, Crossterm, Tokio, Reqwest, Tokio-Tungstenite, Serde, quick-xml, Geo/GeoJSON, TOML, SQLite, tracing and Clap; add only when justified. Evaluate cargo-audit, cargo-deny, cargo-llvm-cov and Criterion when useful; do not invent license allowlists or suppress advisories.

## Testing strategy

Future tests cover units, integration, provider parsing and normalization, state transitions, countdown, persistence, mapping, rendering and snapshots, platform compatibility, performance, long-running operation, terminal restoration and release artifacts. Use authorized fixtures, mock providers, controlled clocks and labeled synthetic data; required CI must not depend on live APIs. Keep live connectivity tests separate.

Cover ordinary events, forecasts/warnings, updates, cancellation/final, revised/missing/expired arrivals, disconnect/reconnect, duplicate/out-of-order/delayed/malformed reports, simultaneous events, location changes and persistence, missing region mappings, unreliable clocks, resizing, Japanese width, normal/abnormal exit and restoration. Future Ratatui TestBackend snapshots at 80×24, 120×36, 160×45 and 200×60 should check both screens for truncated critical content, units, labels, states and layout; real terminal tests remain necessary. Mark all synthetic data DEMO, SIMULATION, TEST, 訓練 or 模擬データ and never send simulated alerts to unsuspecting users or external services.

Provider candidates include P2P Earthquake Information, Wolfx, JMA XML and GIS, NIED and DMDATA.JP; verify current specifications, availability, use and redistribution terms before use. NERV is a functional reference only; do not access private APIs. Independent earthquake motion forecasts require regulatory review and any necessary authorization before implementation or public distribution.

CI expands with snapshots in v0.0.1, provider/location tests in v0.1, EEW clock tests in v0.2, map tests in v0.3, concurrent tsunami tests in v0.4, observation/replay tests in v0.5, reliability/security tests in v0.6, release candidates in v0.9 and authorized delivery in v1.0.
