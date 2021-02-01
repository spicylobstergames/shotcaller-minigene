use crate::*;

/// Buy an item with the earned gold.
pub fn item_purchasing_system(
    mouse_events: &Vec<MouseEvent>,
    buy_buttons: &Components<BuyButton>,
    // items: &Component<ShelfItem>,
    leaders: &Components<Leader>,
    team_leaders: &TeamLeaders,
    selected_leader: &SelectedLeader,
    selected_item: &mut SelectedItem,
    stats: &mut Components<StatSet<Stats>>,
) -> SystemResult {
    for click_ev in mouse_events.iter() {
        if let MouseEvent::EntityClicked(button_entity) = click_ev {
            if buy_buttons.get(*button_entity).is_some() {
                if let Some(shelf_item) = selected_item.0 {
                    for (l, s) in join!(&leaders && &mut stats) {
                        let st = s.unwrap();
                        if l.unwrap().0 == selected_leader.0 && st.stats.get(&Stats::Gold).unwrap().value >= (shelf_item.1 as f64) {
                            println!("comprou o item {:?}", shelf_item);
                            st.stats.get_mut(&Stats::Gold).unwrap().value -= shelf_item.1 as f64;
                            //TODO add item to inventory
                        }// else - not enough gold
                    }
                }
            }
        }
    }
    Ok(())
}
