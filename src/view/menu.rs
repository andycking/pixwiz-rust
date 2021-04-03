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

use crate::commands;

pub const COMMON_MENU_FILE_SAVE: &'static str = "common-menu-file-save";
pub const COMMON_MENU_CUT: &'static str = "common-menu-cut";
pub const COMMON_MENU_COPY: &'static str = "common-menu-copy";
pub const COMMON_MENU_UNDO: &'static str = "common-menu-undo";
pub const COMMON_MENU_REDO: &'static str = "common-menu-redo";
pub const EDIT_MENU_DESELECT: &'static str = "edit-menu-deselect";
pub const MENU_VIEW_SHOW_GRID: &'static str = "menu-view-show-grid";

/// Druid menus are immutable, so if you want to update a menu item at runtime, you have to
/// reconstruct the menu bar from scratch. Use a map to make it easier to tell the menu
/// builder which items to disable (gray out) or select (check mark).
pub struct MenuOpts {
    pub disabled: HashMap<String, bool>,
    pub selected: HashMap<String, bool>,
}

impl MenuOpts {
    pub fn disable(&mut self, key: String, value: bool) {
        self.disabled.insert(key, value);
    }

    pub fn select(&mut self, key: String, value: bool) {
        self.selected.insert(key, value);
    }
}

impl Default for MenuOpts {
    fn default() -> Self {
        let mut disabled: HashMap<String, bool> = HashMap::new();
        let mut selected: HashMap<String, bool> = HashMap::new();

        // We typically start with an untitled document (no path), so the save menu item
        // is disabled by default. It will get enabled when the user performs a save-as,
        // or opens an existing document.
        disabled.insert(COMMON_MENU_FILE_SAVE.to_string(), true);

        // Cut/copy are disabled until there's a selection.
        disabled.insert(COMMON_MENU_CUT.to_string(), true);
        disabled.insert(COMMON_MENU_COPY.to_string(), true);

        // Undo/redo are disabled until you actually make a change.
        disabled.insert(COMMON_MENU_UNDO.to_string(), true);
        disabled.insert(COMMON_MENU_REDO.to_string(), true);

        // Deselect is disabled until there's a selection.
        disabled.insert(EDIT_MENU_DESELECT.to_string(), true);

        // We show the canvas grid by default.
        selected.insert(MENU_VIEW_SHOW_GRID.to_string(), true);

        Self {
            disabled: disabled,
            selected: selected,
        }
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
    fn open_file<T: Data>() -> druid::MenuItem<T> {
        let file_dialog_options =
            druid::FileDialogOptions::default().allowed_types(vec![druid::FileSpec::PNG]);

        druid::MenuItem::new(
            druid::LocalizedString::new("common-menu-file-open"),
            druid::commands::SHOW_OPEN_PANEL.with(file_dialog_options),
        )
        .hotkey(druid::SysMods::Cmd, "o")
    }

    fn save_as<T: Data>() -> druid::MenuItem<T> {
        let file_dialog_options =
            druid::FileDialogOptions::default().allowed_types(vec![druid::FileSpec::PNG]);

        druid::MenuItem::new(
            druid::LocalizedString::new("common-menu-file-save-as"),
            druid::commands::SHOW_SAVE_PANEL.with(file_dialog_options),
        )
        .hotkey(druid::SysMods::CmdShift, "S")
    }

    let mut save_disabled = false;
    if menu_opts.disabled.contains_key(COMMON_MENU_FILE_SAVE) {
        save_disabled = menu_opts.disabled[COMMON_MENU_FILE_SAVE];
    }

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

    let mut undo_disabled = false;
    if menu_opts.disabled.contains_key(COMMON_MENU_UNDO) {
        undo_disabled = menu_opts.disabled[COMMON_MENU_UNDO];
    }

    let mut redo_disabled = false;
    if menu_opts.disabled.contains_key(COMMON_MENU_REDO) {
        redo_disabled = menu_opts.disabled[COMMON_MENU_REDO];
    }

    let mut cut_disabled = false;
    if menu_opts.disabled.contains_key(COMMON_MENU_CUT) {
        cut_disabled = menu_opts.disabled[COMMON_MENU_CUT];
    }

    let mut copy_disabled = false;
    if menu_opts.disabled.contains_key(COMMON_MENU_COPY) {
        copy_disabled = menu_opts.disabled[COMMON_MENU_COPY];
    }

    let mut deselect = false;
    if menu_opts.disabled.contains_key(EDIT_MENU_DESELECT) {
        deselect = menu_opts.disabled[EDIT_MENU_DESELECT];
    }

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
    }

    let mut grid_selected = true;
    if menu_opts.selected.contains_key(MENU_VIEW_SHOW_GRID) {
        grid_selected = menu_opts.selected[MENU_VIEW_SHOW_GRID];
    }

    druid::MenuDesc::new(druid::LocalizedString::new("menu-view-menu").with_placeholder("View"))
        .append(toggle_grid().selected_if(|| grid_selected))
}
