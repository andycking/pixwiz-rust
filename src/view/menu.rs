// Copyright 2021 Andy King
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::collections::HashMap;

use druid::widget::prelude::*;

use crate::common::commands;
use crate::common::constants;

pub const COMMON_MENU_FILE_SAVE: &str = "common-menu-file-save";
pub const COMMON_MENU_CUT: &str = "common-menu-cut";
pub const COMMON_MENU_COPY: &str = "common-menu-copy";
pub const COMMON_MENU_UNDO: &str = "common-menu-undo";
pub const COMMON_MENU_REDO: &str = "common-menu-redo";
pub const EDIT_MENU_DESELECT: &str = "edit-menu-deselect";
pub const MENU_VIEW_SHOW_GRID: &str = "menu-view-show-grid";

/// Druid menus are immutable, so if you want to update a menu item at runtime, you have to
/// reconstruct the menu bar from scratch. Use a map to make it easier to tell the menu
/// builder which items to disable (gray out) or select (check mark).
pub struct MenuOpts {
    disabled: HashMap<&'static str, bool>,
    selected: HashMap<&'static str, bool>,
}

impl MenuOpts {
    pub fn disable(&mut self, key: &'static str, value: bool) {
        self.disabled.insert(key, value);
    }

    pub fn disabled_or(&self, key: &'static str, default: bool) -> bool {
        if self.disabled.contains_key(key) {
            self.disabled[key]
        } else {
            default
        }
    }

    pub fn select(&mut self, key: &'static str, value: bool) {
        self.selected.insert(key, value);
    }

    pub fn selected_or(&self, key: &'static str, default: bool) -> bool {
        if self.selected.contains_key(key) {
            self.selected[key]
        } else {
            default
        }
    }
}

impl Default for MenuOpts {
    fn default() -> Self {
        let mut disabled: HashMap<&'static str, bool> = HashMap::new();
        let mut selected: HashMap<&'static str, bool> = HashMap::new();

        // We typically start with an untitled document (no path), so the save menu item
        // is disabled by default. It will get enabled when the user performs a save-as,
        // or opens an existing document.
        disabled.insert(COMMON_MENU_FILE_SAVE, true);

        // Cut/copy are disabled until there's a selection.
        disabled.insert(COMMON_MENU_CUT, true);
        disabled.insert(COMMON_MENU_COPY, true);

        // Undo/redo are disabled until you actually make a change.
        disabled.insert(COMMON_MENU_UNDO, true);
        disabled.insert(COMMON_MENU_REDO, true);

        // Deselect is disabled until there's a selection.
        disabled.insert(EDIT_MENU_DESELECT, true);

        // We show the canvas grid by default.
        selected.insert(MENU_VIEW_SHOW_GRID, true);

        Self { disabled, selected }
    }
}

pub fn menu_bar<T: Data>(menu_opts: &MenuOpts) -> druid::MenuDesc<T> {
    druid::MenuDesc::new(druid::LocalizedString::new(""))
        .append(druid::platform_menus::mac::application::default())
        .append(build_file_menu(menu_opts))
        .append(build_edit_menu(menu_opts))
        .append(build_image_menu())
        .append(build_view_menu(menu_opts))
}

fn build_file_menu<T: Data>(menu_opts: &MenuOpts) -> druid::MenuDesc<T> {
    fn file_dialog_opts() -> druid::FileDialogOptions {
        druid::FileDialogOptions::default().allowed_types(constants::ALLOWED_FILE_TYPES.to_vec())
    }

    fn open_file<T: Data>() -> druid::MenuItem<T> {
        druid::MenuItem::new(
            druid::LocalizedString::new("common-menu-file-open"),
            druid::commands::SHOW_OPEN_PANEL.with(file_dialog_opts()),
        )
        .hotkey(druid::SysMods::Cmd, "o")
    }

    fn save_as<T: Data>() -> druid::MenuItem<T> {
        druid::MenuItem::new(
            druid::LocalizedString::new("common-menu-file-save-as"),
            druid::commands::SHOW_SAVE_PANEL.with(file_dialog_opts()),
        )
        .hotkey(druid::SysMods::CmdShift, "S")
    }

    let save_disabled = menu_opts.disabled_or(COMMON_MENU_FILE_SAVE, false);

    druid::MenuDesc::new(druid::LocalizedString::new("common-menu-file-menu"))
        .append(druid::platform_menus::mac::file::new_file())
        .append(open_file())
        .append_separator()
        .append(druid::platform_menus::mac::file::close())
        .append(druid::platform_menus::mac::file::save().disabled_if(|| save_disabled))
        .append(save_as())
}

fn build_edit_menu<T: Data>(menu_opts: &MenuOpts) -> druid::MenuDesc<T> {
    fn edit_menu_select_all<T: Data>() -> druid::MenuItem<T> {
        druid::MenuItem::new(
            druid::LocalizedString::new("menu-edit-select-all").with_placeholder("Select All"),
            commands::EDIT_SELECT_ALL,
        )
        .hotkey(druid::SysMods::Cmd, "a")
    }
    fn edit_menu_deselect<T: Data>() -> druid::MenuItem<T> {
        druid::MenuItem::new(
            druid::LocalizedString::new("menu-edit-deselect").with_placeholder("Deselect"),
            commands::EDIT_DESELECT,
        )
    }

    let undo_disabled = menu_opts.disabled_or(COMMON_MENU_UNDO, false);
    let redo_disabled = menu_opts.disabled_or(COMMON_MENU_REDO, false);
    let cut_disabled = menu_opts.disabled_or(COMMON_MENU_CUT, false);
    let copy_disabled = menu_opts.disabled_or(COMMON_MENU_COPY, false);
    let deselect = menu_opts.disabled_or(EDIT_MENU_DESELECT, false);

    druid::MenuDesc::new(druid::LocalizedString::new("common-menu-edit-menu"))
        .append(druid::platform_menus::common::undo().disabled_if(|| undo_disabled))
        .append(druid::platform_menus::common::redo().disabled_if(|| redo_disabled))
        .append_separator()
        .append(druid::platform_menus::common::cut().disabled_if(|| cut_disabled))
        .append(druid::platform_menus::common::copy().disabled_if(|| copy_disabled))
        .append(druid::platform_menus::common::paste())
        .append_separator()
        .append(edit_menu_select_all())
        .append(edit_menu_deselect().disabled_if(|| deselect))
}

fn build_image_menu<T: Data>() -> druid::MenuDesc<T> {
    fn fill<T: Data>() -> druid::MenuItem<T> {
        druid::MenuItem::new(
            druid::LocalizedString::new("menu-image-fill").with_placeholder("Fill"),
            commands::IMAGE_FILL.with(false),
        )
        .hotkey(druid::SysMods::AltCmd, "f")
    }

    fn brighten<T: Data>() -> druid::MenuItem<T> {
        druid::MenuItem::new(
            druid::LocalizedString::new("menu-image-brighten").with_placeholder("Brighten"),
            commands::IMAGE_BRIGHTEN,
        )
    }

    fn darken<T: Data>() -> druid::MenuItem<T> {
        druid::MenuItem::new(
            druid::LocalizedString::new("menu-image-darken").with_placeholder("Darken"),
            commands::IMAGE_DARKEN,
        )
    }

    fn black_and_white<T: Data>() -> druid::MenuItem<T> {
        druid::MenuItem::new(
            druid::LocalizedString::new("menu-image-black-and-white")
                .with_placeholder("Black && White"),
            commands::IMAGE_BLACK_AND_WHITE,
        )
    }

    fn desaturate<T: Data>() -> druid::MenuItem<T> {
        druid::MenuItem::new(
            druid::LocalizedString::new("menu-image-desaturate").with_placeholder("Desaturate"),
            commands::IMAGE_DESATURATE,
        )
    }

    fn dither_floyd<T: Data>() -> druid::MenuItem<T> {
        druid::MenuItem::new(
            druid::LocalizedString::new("menu-image-dither-floyd")
                .with_placeholder("Dither - Floyd"),
            commands::IMAGE_DITHER_FLOYD,
        )
    }

    druid::MenuDesc::new(druid::LocalizedString::new("menu-image-menu").with_placeholder("Image"))
        .append(fill())
        .append_separator()
        .append(brighten())
        .append(darken())
        .append_separator()
        .append(black_and_white())
        .append(desaturate())
        .append(dither_floyd())
}

fn build_view_menu<T: Data>(menu_opts: &MenuOpts) -> druid::MenuDesc<T> {
    fn toggle_grid<T: Data>() -> druid::MenuItem<T> {
        druid::MenuItem::new(
            druid::LocalizedString::new(MENU_VIEW_SHOW_GRID).with_placeholder("Show Grid"),
            commands::VIEW_SHOW_GRID,
        )
        .hotkey(druid::SysMods::AltCmd, "'")
    }

    let grid_selected = menu_opts.selected_or(MENU_VIEW_SHOW_GRID, true);

    druid::MenuDesc::new(druid::LocalizedString::new("menu-view-menu").with_placeholder("View"))
        .append(toggle_grid().selected_if(|| grid_selected))
}
