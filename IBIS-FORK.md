# ibisOffice local gpui-component fork

This is an independent fork of the published `gpui-component` 0.6.6 crate. Its source is not part of the ibisOffice source repository.

- Upstream repository declared by the crate: https://github.com/longbridge/gpui-kit
- Upstream source revision recorded in `.cargo_vcs_info.json`: `9765ae2c9a5eccfa13891248a445991e6f6a09d8`, directory `crates/component`.
- Distribution: https://crates.io/api/v1/crates/gpui-component/0.6.6/download
- License: Apache-2.0. Original copyright/license notice is retained in `LICENSE-APACHE`; the full Apache 2.0 text is included in `LICENSE-2.0` (https://www.apache.org/licenses/LICENSE-2.0.txt).
- Local baseline was imported unchanged from Cargo's cached published crate. The upstream repository's raw LICENSE URL at the recorded revision returned 404 at the time of this work; its availability was not assumed.
- No remote has been configured or pushed.

## Modifications by ibis inc. (2026-10-26)

- Place cascading menus to the right of the parent when possible, otherwise to its left.
- If neither side fits, highlight the hovered row without opening its submenu. Click, Enter or the appropriate arrow explicitly opens the submenu and clamps it inside the viewport.
- Use the child menu's width and measured row origin, rather than the parent menu's width, for placement.
- Preserve the existing nested deferred paint order, keyboard navigation, focus and dismiss chain.
- Cap menu width to the viewport; retain existing vertical overflow fitting.
- Add pure geometry tests and an activation-state regression test.

The modified source files carry an ibis modification notice. Original author copyrights remain with their respective owners.
