use crate::*;

pub fn mouse_select_system(
    mouse: &Mouse,
    entities: &Entities,
    selectables: &Components<MouseSelectable>,
    pos: &Components<Point>,
    input_events: &mut Vec<MouseEvent>,
) -> SystemResult {
    for (e, _, pos) in join!(&entities && &selectables && &pos) {
        if mouse.pos == (pos.unwrap().x, pos.unwrap().y) {
            input_events.push(MouseEvent::ItemSelected(e.unwrap()));
        }
    }
    Ok(())
}
