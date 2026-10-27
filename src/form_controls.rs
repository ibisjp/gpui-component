// Copyright 2026 ibis inc. Licensed under Apache-2.0.
//! Observe rendered form controls without walking or naming application panels.
use gpui::{App, FocusHandle, Window};
use std::{cell::RefCell, rc::Rc};
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ControlKind {
    Text,
    Multiline,
    Other,
}
#[derive(Clone)]
pub struct FormControl {
    pub kind: ControlKind,
    pub focus: Option<FocusHandle>,
    pub disabled: bool,
    pub open: bool,
    pub composing: Option<Rc<dyn Fn(&mut Window, &mut App) -> bool>>,
}
thread_local! {static SCOPES:RefCell<Vec<Rc<RefCell<Vec<FormControl>>>>>=const{RefCell::new(Vec::new())};}
pub fn record(control: FormControl) {
    SCOPES.with(|stack| {
        if let Some(scope) = stack.borrow().last() {
            scope.borrow_mut().push(control);
        }
    });
}
pub fn other(disabled: bool, open: bool) {
    record(FormControl {
        kind: ControlKind::Other,
        focus: None,
        disabled,
        open,
        composing: None,
    });
}
pub fn collect<T>(render: impl FnOnce() -> T) -> (T, Vec<FormControl>) {
    struct Scope;
    impl Drop for Scope {
        fn drop(&mut self) {
            SCOPES.with(|s| {
                s.borrow_mut().pop();
            });
        }
    }
    let controls = Rc::new(RefCell::new(Vec::new()));
    SCOPES.with(|s| s.borrow_mut().push(controls.clone()));
    let scope = Scope;
    let result = render();
    drop(scope);
    let controls = controls.borrow().clone();
    (result, controls)
}
