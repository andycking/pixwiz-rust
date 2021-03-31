use crate::model::app_state::AppState;
use crate::model::mod_record::ModRecord;

/// Push a point onto the undo stack.
pub fn push_point(data: &mut AppState, p: druid::Point) {
    let area = druid::Rect::new(p.x, p.y, p.x + 1.0, p.y + 1.0);
    push(data, area);
}

/// Push an area onto the undo stack.
pub fn push(data: &mut AppState, area: druid::Rect) {
    push_inner(data, area);

    // Important: reset the redo stack!
    // This is okay: undo -> undo -> redo -> redo
    // This is not okay: undo -> paint -> redo
    data.redo.clear();
}

fn push_inner(data: &mut AppState, area: druid::Rect) {
    let bytes = data.pixels.read_area(area);
    let record = ModRecord::new(area, bytes);

    data.undo.push(record);
}

/// Pop a record from the undo stack and apply it.
pub fn pop(data: &mut AppState) {
    match data.undo.pop() {
        Some(record) => {
            // Before we undo, record what we just did, so that we can redo it again.
            push_redo(data, record.area);

            data.pixels.write_area(record.area, &*record.bytes);
        }
        _ => {}
    }
}

fn push_redo(data: &mut AppState, area: druid::Rect) {
    let bytes = data.pixels.read_area(area);
    let record = ModRecord::new(area, bytes);

    data.redo.push(record);
}

/// Pop a record from the redo stack and apply it.
pub fn pop_redo(data: &mut AppState) {
    match data.redo.pop() {
        Some(record) => {
            // Before we redo, record what we just did, so that we can undo it again.
            // But call the inner function, so that we don't reset the redo stack!
            push_inner(data, record.area);

            data.pixels.write_area(record.area, &*record.bytes);
        }
        _ => {}
    }
}