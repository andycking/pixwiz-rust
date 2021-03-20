use druid::widget::prelude::*;
use druid::widget::Flex;
use druid::WidgetExt;

mod canvas;
mod palette;
mod theme;
mod tool_button;

use crate::model::commands;
use crate::model::state::AppState;
use crate::model::types::ToolType;
use crate::view::canvas::Canvas;
use crate::view::canvas::CanvasController;
use crate::view::palette::Palette;
use crate::view::tool_button::ToolButton;

pub fn build_ui() -> impl druid::Widget<AppState> {
    Flex::column()
        .cross_axis_alignment(druid::widget::CrossAxisAlignment::End)
        .with_default_spacer()
        .with_child(build_main_pane())
        .with_default_spacer()
        .with_child(build_status_bar())
        .with_default_spacer()
        .background(theme::MAIN_FILL)
}

fn build_tools_row<T: druid::Data>(
    a: impl Widget<T> + 'static,
    b: impl Widget<T> + 'static,
) -> impl Widget<T> {
    Flex::row()
        .with_spacer(8.0)
        .with_child(a)
        .with_spacer(8.0)
        .with_child(b)
        .with_spacer(8.0)
}

fn build_tools() -> impl druid::Widget<AppState> {
    let marquee_bytes = include_bytes!("./assets/marquee.png");
    let lasso_bytes = include_bytes!("./assets/lasso.png");
    let move_bytes = include_bytes!("./assets/move.png");
    let cropper_bytes = include_bytes!("./assets/cropper.png");
    let paint_bytes = include_bytes!("./assets/paint.png");
    let eraser_bytes = include_bytes!("./assets/eraser.png");
    let fill_bytes = include_bytes!("./assets/fill.png");
    let dropper_bytes = include_bytes!("./assets/dropper.png");

    Flex::column()
        .with_child(build_tools_row(
            ToolButton::new(ToolType::Marquee, marquee_bytes),
            ToolButton::new(ToolType::Lasso, lasso_bytes),
        ))
        .with_spacer(8.0)
        .with_child(build_tools_row(
            ToolButton::new(ToolType::Move, move_bytes),
            ToolButton::new(ToolType::Cropper, cropper_bytes),
        ))
        .with_spacer(8.0)
        .with_child(build_tools_row(
            ToolButton::new(ToolType::Paint, paint_bytes),
            ToolButton::new(ToolType::Eraser, eraser_bytes),
        ))
        .with_spacer(8.0)
        .with_child(build_tools_row(
            ToolButton::new(ToolType::Fill, fill_bytes),
            ToolButton::new(ToolType::Dropper, dropper_bytes),
        ))
        .with_spacer(8.0)
}

fn build_color_well() -> impl druid::Widget<AppState> {
    druid::widget::Painter::new(|ctx, data: &AppState, _env| {
        let rect = ctx.size().to_rect();
        let color = match data.tool_type {
            ToolType::Dropper => &data.pos_color,
            _ => &data.brush_color,
        };
        ctx.fill(rect, color);
    })
    .fix_size(88.0, 30.0)
    .border(theme::COLOR_WELL_STROKE, 1.0)
}

fn build_left_pane() -> impl druid::Widget<AppState> {
    Flex::column()
        .cross_axis_alignment(druid::widget::CrossAxisAlignment::End)
        .with_child(build_tools())
        .with_default_spacer()
        .with_child(build_color_well())
}

fn build_canvas() -> impl druid::Widget<AppState> {
    Canvas::new().controller(CanvasController)
}

fn build_palette() -> impl druid::Widget<AppState> {
    Palette::new(include_bytes!("./assets/vga.pal")).background(theme::PALETTE_FILL)
}

fn build_preview() -> impl druid::Widget<AppState> {
    druid::widget::Painter::new(|ctx, data: &AppState, _env| {
        let mut i = 0;
        for y in 0..32 {
            for x in 0..32 {
                let rect = druid::Rect::new(x as f64, y as f64, (x as f64) + 1.0, (y as f64) + 1.0);

                let color = data.pixels.read(i);
                let (_, _, _, a) = color.as_rgba8();
                if a != 255 {
                    ctx.fill(rect, &theme::PREVIEW_FILL);
                };

                ctx.fill(rect, &color);

                i += 1;
            }
        }
    })
    .fix_size(32.0, 32.0)
    .border(theme::PREVIEW_STROKE, 1.0)
}

fn build_right_pane() -> impl druid::Widget<AppState> {
    Flex::column()
        .with_child(build_preview())
        .with_default_spacer()
}

fn build_center_pane() -> impl druid::Widget<AppState> {
    Flex::column()
        .with_child(build_canvas())
        .with_default_spacer()
        .with_child(build_palette())
        .with_default_spacer()
}

fn build_main_pane() -> impl druid::Widget<AppState> {
    Flex::row()
        .cross_axis_alignment(druid::widget::CrossAxisAlignment::Start)
        .with_default_spacer()
        .with_child(build_left_pane())
        .with_default_spacer()
        .with_child(build_center_pane())
        .with_default_spacer()
        .with_child(build_right_pane())
        .with_default_spacer()
}

fn build_status_label() -> impl druid::Widget<AppState> {
    druid::widget::Label::new(|data: &AppState, _env: &_| {
        let (r, g, b, a) = data.pos_color.as_rgba8();
        let selection = data.selection.unwrap_or(druid::Rect::ZERO);
        format!(
            "{:>10}  r:{:3} g:{:3} b:{:3} a:{:3}  {:02}:{:02}-{:02}:{:02}  {:02}:{:02}",
            data.tool_type.to_string().to_lowercase(),
            r,
            g,
            b,
            a,
            selection.x0,
            selection.y0,
            selection.x1,
            selection.y1,
            data.current_pos.x,
            data.current_pos.y
        )
    })
    .with_font(druid::FontDescriptor::new(druid::FontFamily::MONOSPACE))
    .with_text_color(theme::STATUS_BAR_STROKE)
    .padding(3.0)
}

fn build_status_bar() -> impl druid::Widget<AppState> {
    Flex::row()
        .main_axis_alignment(druid::widget::MainAxisAlignment::End)
        .must_fill_main_axis(true)
        .with_child(build_status_label())
        .with_default_spacer()
        .background(theme::STATUS_BAR_FILL)
}

pub fn build_menu_bar<T: Data>(disable_save: bool) -> druid::MenuDesc<T> {
    druid::MenuDesc::new(druid::LocalizedString::new(""))
        .append(druid::platform_menus::mac::application::default())
        .append(build_file_menu(disable_save))
        .append(build_edit_menu())
        .append(build_image_menu())
        .append(build_view_menu())
}

fn build_file_menu<T: Data>(disable_save: bool) -> druid::MenuDesc<T> {
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

    druid::MenuDesc::new(druid::LocalizedString::new("common-menu-file-menu"))
        .append(druid::platform_menus::mac::file::new_file())
        .append(open_file())
        .append_separator()
        .append(druid::platform_menus::mac::file::close())
        .append(druid::platform_menus::mac::file::save().disabled_if(|| disable_save))
        .append(save_as())
}

fn build_edit_menu<T: Data>() -> druid::MenuDesc<T> {
    druid::MenuDesc::new(druid::LocalizedString::new("common-menu-edit-menu"))
        .append(druid::platform_menus::common::undo())
        .append(druid::platform_menus::common::redo())
        .append_separator()
        .append(druid::platform_menus::common::cut())
        .append(druid::platform_menus::common::copy())
        .append(druid::platform_menus::common::paste())
}

fn build_image_menu<T: Data>() -> druid::MenuDesc<T> {
    fn convert_to_grayscale<T: Data>() -> druid::MenuItem<T> {
        druid::MenuItem::new(
            druid::LocalizedString::new("menu-image-grayscale")
                .with_placeholder("Convert to Grayscale"),
            commands::CONVERT_TO_GRAYSCALE,
        )
    }

    druid::MenuDesc::new(druid::LocalizedString::new("menu-image-menu").with_placeholder("Image"))
        .append(convert_to_grayscale())
}

fn build_view_menu<T: Data>() -> druid::MenuDesc<T> {
    fn toggle_grid<T: Data>() -> druid::MenuItem<T> {
        druid::MenuItem::new(
            druid::LocalizedString::new("menu-view-show-grid").with_placeholder("Show Grid"),
            commands::SHOW_GRID,
        )
    }

    druid::MenuDesc::new(druid::LocalizedString::new("menu-view-menu").with_placeholder("View"))
        .append(toggle_grid().selected())
}
