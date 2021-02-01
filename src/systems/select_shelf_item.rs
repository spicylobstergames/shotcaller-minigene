use crate::*;

/// Set the selected item on the shelf, ready to buy.
pub fn select_shelf_item_system(
    mouse_events: &Vec<MouseEvent>,
    items: &Components<ShelfItem>,
    selected_item: &mut SelectedItem,
) -> SystemResult {
    for ev in mouse_events.iter() {
        if let MouseEvent::EntitySelected(e) = ev {
            if let Some(i) = items.get(*e) {
                println!("aa");
                selected_item.0 = Some(*i);
            }
        }
    }
    Ok(())
}
