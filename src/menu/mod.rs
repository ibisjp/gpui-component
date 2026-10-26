// Modified by ibis inc. (2026-10-26): parent-avoiding submenu placement.
use gpui::App;

mod app_menu_bar;
mod context_menu;
mod dropdown_menu;
mod menu_item;
mod popup_menu;
mod submenu_placement;

pub use app_menu_bar::AppMenuBar;
pub use context_menu::{ContextMenu, ContextMenuExt, ContextMenuState};
pub use dropdown_menu::DropdownMenu;
pub use popup_menu::{PopupMenu, PopupMenuItem};

pub(crate) fn init(cx: &mut App) {
    app_menu_bar::init(cx);
    popup_menu::init(cx);
}
