# Product design (planned)

SERA is supplementary to official emergency information. The approved direction combines Design B, Seismic Radar, for normal monitoring and Design C, Emergency Focus, for EEW. SERA has its own identity; do not reuse proprietary branding or assets. No screen is implemented yet.

## Radar

The map occupies most of the terminal. Header: brand, version, navigation, provider health and JST. Left: monitoring, EEW and tsunami status, watch locations and latest earthquake. Map: verified coastline and boundaries, epicenters, intensity regions, authorized observation points and watch locations; estimated P/S waves only with reliable authorized data. Configurable right panels may show history, events and authorized waveforms. Footer: legend, zoom, shortcuts, freshness and source. Plan zoom, pan and automatic epicenter or primary-location tracking.

## Alert

Automatically enter for an applicable EEW warning; relevant forecasts use a configurable threshold. Header shows classification, report number and status. The prominent left panel shows primary watch location, estimated strong-motion arrival countdown, regional and overall maximum predicted intensity. The right map shows epicenter, location, predicted regions and authorized estimated waves. The lower panel shows hypocenter, magnitude, depth, origin time, tsunami status and freshness; the footer includes immediate safety guidance, controls and connection status. Warnings remain visible during manual navigation.

Startup and ordinary reports use Radar; later reports update the corresponding event. Final reports remain marked, cancellations stop only the affected countdown, and disconnected or stale data are explicit. Neither final reports, countdown expiry, missing updates nor loss of connectivity establish safety. Return to Radar only after a defined recovery condition; a 120-second delay is a provisional candidate. Forecast intensity 3 at a relevant registered location is a provisional threshold. Multiple events need independent identifiers, report sequences, predictions and cancellations; priority must be designed and tested.

## Locations and countdown

Users can add, edit, remove and persist multiple locations, choose or change a primary location without restart, search regions and map them to verified JMA forecast areas. Display predictions as regional, not exact building-level values. Keep location local by default; automatic acquisition and transmission require explicit authorization. A location change must not erase active warnings.

Calculate remaining time from the provider's estimated arrival timestamp and current time, never reception time. Update each second, accept revisions including increases, and handle missing, expired, cancelled, stale, unsupported-method and unreliable-clock states. Distinguish PLUM and regional estimates; do not invent estimates or use fixed wave velocity as an authoritative prediction. Provisional colors: 10+ seconds yellow, 5–9 orange, 1–4 red. Expiry means 到達予想時刻経過, not confirmation of arrival or safety. Show safety guidance immediately.

Use accurate terms: 地震情報、緊急地震速報（警報）、緊急地震速報（予報）、最大震度、地域の予測震度、予測最大震度、主要動到達まで（予想）、到達予想時刻経過、予測情報なし、到達時刻情報なし、予測不能、情報更新停止、通信切断、最終報、取消報、監視地点、震源、震央、津波警報、津波注意報、直ちに身の安全を確保してください. Unknown tsunami information is not a safe status.

## Visual system and accessibility

Dark background `#0A1017`, map `#11202E`, accent `#8BC8D5`, text `#E9EFF4`, alert `#C43545`; P-wave blue, S-wave red, healthy connection muted green. These are candidates subject to contrast and terminal tests. Use monospace, straight thin borders, consistent spacing and minimal decoration. No gradients, shadows, glass effects or decorative animation. Provide non-color labels and usable fallbacks for terminals without true color or image protocols.

Target macOS, Linux, Windows Terminal, SSH and tmux, Unicode and Japanese full-width text, resizing, keyboard navigation, optional mouse and terminal restoration. Minimum 80×24, recommended 160×45; simplify small layouts while retaining critical warnings. Geographic data must be accurate and licensed; block, half-block and Braille rendering are candidates. Sixel and Kitty cannot be exclusive requirements.

Provisional bindings: `1` Radar, `2` Alert, `3` Tsunami, `4` Archive, `5` Monitor, `6` Settings, `L` locations, `Shift+L` primary location, `M` map mode, `F` tracking, `Home` center, `+`/`-` zoom, `W/A/S/D` pan, `Ctrl+S` settings, `Tab` focus, `Esc` return, `H` help, `Q` quit. Resolve the case distinction between `L` and `Shift+L` during terminal compatibility testing before implementation; essential alerts cannot depend solely on mouse input. Additional planned screens: Tsunami, Locations, Archive, Monitor, Replay, Settings and Daemon.
