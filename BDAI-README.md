# BD AI — remote desktop client for BDS Business Automation

**BD AI** is a rebranded build of [RustDesk](https://github.com/rustdesk/rustdesk), used
internally by BDS Business Automation staff to reach BDS machines.

This repository is a fork of `rustdesk/rustdesk`. Branch `bdai` carries the brand
changes; `master` tracks upstream. The companion fork
[`BDSxAutomation/hbb_common`](https://github.com/BDSxAutomation/hbb_common) (branch `bdai`)
carries the one-line app-name change in the shared library.

## What changed vs upstream

| Area | Change |
|---|---|
| App name | `APP_NAME` default `RustDesk` → `BD-AI` (`libs/hbb_common/src/config.rs`). Names must match `[a-zA-Z0-9-]+`, so the internal name is `BD-AI`; the display name in Windows metadata is "BD AI". Every translated UI string picks this up automatically via `src/lang.rs`. |
| Icons & logo | BDS "Signal" Morse mark on the ink app tile — `res/icon.*`, `res/tray-icon.ico`, `flutter/windows/runner/resources/app_icon.ico`, `flutter/assets/icon.*`, `flutter/assets/logo*.png`. Generated from the canonical mark geometry; regenerate with the script in the BDS estate notes. |
| Theme | Cobalt `#1B3FD6` brand accent, ink/paper surfaces, Hanken Grotesk UI face, JetBrains Mono for the device ID (`flutter/lib/common.dart`, `flutter/pubspec.yaml`). |
| Windows metadata | `flutter/windows/runner/Runner.rc` — company, product, description, copyright. |
| Build | `.github/workflows/bdai-windows.yml` builds the Windows x64 portable EXE and MSI on demand. Upstream's other workflows are removed from this branch. |
| Servers | Unchanged: uses RustDesk's public rendezvous/relay servers until a BDS-hosted `hbbs`/`hbbr` exists. |

## Licence

RustDesk is licensed under the **GNU AGPL-3.0**; so is this fork. Anyone who receives a
BD AI binary is entitled to this source. "RustDesk" is a trademark of Purslane Tech Pte.
Ltd.; the "Powered by RustDesk" attribution in the About screen is intentionally kept.
