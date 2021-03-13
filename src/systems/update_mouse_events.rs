use crate::*;

/// Spawns mouse events.
pub fn update_mouse_events_system(
    mouse: &Mouse,
    entities: &Entities,
    selectables: &Components<MouseSelectable>,
    creeps: &Components<Creep>,
    leaders: &Components<Leader>,
    clickables: &Components<MouseClickable>,
    hoverables: &Components<MouseHoverable>,
    pos: &Components<Point>,
    mouse_events: &mut Vec<MouseEvent>,
) -> SystemResult {
    mouse_events.clear();
    // Just location of click:
    let mut click_contains_entity = false;

    for (e, _, pos) in join!(&entities && &selectables && &pos) {
        if mouse.pos == (pos.unwrap().x, pos.unwrap().y) && mouse.left_click {
            mouse_events.push(MouseEvent::EntitySelected(e.unwrap()));
            click_contains_entity = true;
        }
    }
    for (e, _, pos) in join!(&entities && &creeps && &pos) {
        if mouse.pos == (pos.unwrap().x, pos.unwrap().y) && mouse.left_click {
            mouse_events.push(MouseEvent::UnitSelected(e.unwrap()));
            click_contains_entity = true;
        }
    }
    for (e, _, pos) in join!(&entities && &leaders && &pos) {
        if mouse.pos == (pos.unwrap().x, pos.unwrap().y) && mouse.left_click {
            mouse_events.push(MouseEvent::UnitSelected(e.unwrap()));
            click_contains_entity = true;
        }
    }
    for (e, _, pos) in join!(&entities && &clickables && &pos) {
        if mouse.pos == (pos.unwrap().x, pos.unwrap().y) && mouse.left_click {
            mouse_events.push(MouseEvent::EntityClicked(e.unwrap()));
            click_contains_entity = true;
        }
    }
    for (e, _, pos) in join!(&entities && &hoverables && &pos) {
        if mouse.pos == (pos.unwrap().x, pos.unwrap().y) {
            mouse_events.push(MouseEvent::EntityHovered(e.unwrap()));
        }
    }

    if mouse.left_click {
        mouse_events.push(MouseEvent::PositionClicked {
            pos: Point::new(mouse.pos.0, mouse.pos.1),
            contains_entity: click_contains_entity,
        });
    }

    Ok(())
}
