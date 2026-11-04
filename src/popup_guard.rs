// Added by ibis inc. (2026-11-04): a press that closes a popup is not delivered to what is under the pointer.
//! The click that closes a popup (a menu, a dropdown list, a context menu) only closes it: the element under the
//! pointer gets no mouse down, up, click or double click from it.
//!
//! Popups close themselves from `on_mouse_down_out`, a capture-phase listener that does not stop the event, and the
//! popups are painted after (and so capture after) the window's root. So the root cannot stop the press itself; it
//! asks, and a catcher painted after every popup (a `deferred` element of the highest priority) decides:
//!
//! 1. the root calls [`begin_press`] first thing in its capture-phase mouse down;
//! 2. the popups run their `on_mouse_down_out` and close (a [`PopupMenu`](crate::menu::PopupMenu) calls
//!    [`note_outside_dismiss`]; a popover or a select is seen going from open to closed through
//!    `GlobalState::is_in_deferred_context`; the app's own popups call [`note_outside_dismiss`]);
//! 3. the catcher calls [`end_press`]: `true` means this press closed something, and the catcher stops it, and
//!    [`swallow_release`] stops the button's release. A press that is the second of a double click on the same spot
//!    follows its first (the first closed the popup), so it is swallowed too.
//!
//! Both halves are elements to put in the window's root: [`begin`] and [`catcher`]. They listen with
//! `Window::on_mouse_event`, not through a `div`'s listeners: those only fire while the div's hitbox is hovered, and an
//! occluding popup or panel under the pointer hides it.
//!
//! A press inside a popup closes nothing, so it is not touched. Hover-only popups (tooltips, hover cards) and modeless
//! dialogs close by other means and never reach this.
use gpui::{App, DispatchPhase, IntoElement, MouseDownEvent, MouseUpEvent, Styled as _, canvas, deferred};
use std::cell::Cell;

thread_local! {
    static DISMISSED: Cell<u64> = const { Cell::new(0) };
    static BEGUN: Cell<Option<(u64, bool)>> = const { Cell::new(None) };
    static RELEASE: Cell<bool> = const { Cell::new(false) };
    static CHAIN: Cell<bool> = const { Cell::new(false) };
}

/// A popup has just been closed by a press outside of it.
pub fn note_outside_dismiss() {
    DISMISSED.with(|d| d.set(d.get().wrapping_add(1)));
}

/// The root's capture-phase mouse down, before any popup has seen it.
fn begin_press(cx: &App) {
    BEGUN.with(|b| b.set(Some((DISMISSED.with(Cell::get), crate::global_state::GlobalState::is_in_deferred_context(cx)))));
}

/// The catcher's capture-phase mouse down, after every popup has seen it. `true`: the press closed a popup and must go no further.
fn end_press(cx: &App, click_count: usize) -> bool {
    let closed = BEGUN.with(Cell::take).is_some_and(|(count, deferred)| {
        count != DISMISSED.with(Cell::get) || (deferred && !crate::global_state::GlobalState::is_in_deferred_context(cx))
    });
    let second = click_count > 1 && CHAIN.with(Cell::get);
    CHAIN.with(|c| c.set(closed));
    let swallowed = closed || second;
    if swallowed { RELEASE.with(|r| r.set(true)); }
    swallowed
}

/// The catcher's capture-phase mouse up: `true` when the press it ends was swallowed.
fn swallow_release() -> bool {
    RELEASE.with(Cell::take)
}

/// The first half, painted before every popup (a child of the window's root): notes how things stand at each mouse down.
pub fn begin() -> impl IntoElement {
    canvas(|_, _, _| (), |_, _, window, _| {
        window.on_mouse_event(|_: &MouseDownEvent, phase, _, cx| {
            if phase == DispatchPhase::Capture { begin_press(cx); }
        });
    }).absolute().inset_0()
}

/// The second half, painted after every popup (a deferred layer of the highest priority, a child of the window's root): a
/// press that closed a popup, and its release, go no further.
pub fn catcher() -> impl IntoElement {
    deferred(canvas(|_, _, _| (), |_, _, window, _| {
        window.on_mouse_event(|event: &MouseDownEvent, phase, _, cx| {
            if phase == DispatchPhase::Capture && end_press(cx, event.click_count) { cx.stop_propagation(); }
        });
        window.on_mouse_event(|_: &MouseUpEvent, phase, _, cx| {
            if phase == DispatchPhase::Capture && swallow_release() { cx.stop_propagation(); }
        });
    }).absolute().inset_0()).with_priority(usize::MAX)
}
