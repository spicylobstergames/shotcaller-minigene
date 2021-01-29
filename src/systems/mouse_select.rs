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
            // TODO deal with different types os selection e.g. leader, summon, etc.
            input_events.push(MouseEvent::EntitySelected(e.unwrap()));
        }
    }
    Ok(())
}
