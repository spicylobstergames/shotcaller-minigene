use crate::*;

/// Buy an item with the earned gold.
pub fn item_purchasing_system(
    mouse_events: &Vec<MouseEvent>,
    buy_buttons: &Components<BuyButton>,
    leaders: &Components<Leader>,
    selected_leader: &SelectedLeader,
    item_defs: &ItemDefinitions<Items, (), ()>,
    inventory: &mut Components<Inventory<Items, (), ()>>,
    selected_item: &mut SelectedItem,
    stats: &mut Components<StatSet<Stats>>,
) -> SystemResult {
    for click_ev in mouse_events.iter() {
        if let MouseEvent::EntityClicked(button_entity) = click_ev {
            if buy_buttons.get(*button_entity).is_some() {
                if let Some(shelf_item) = selected_item.0 {
                    for (s, i, l) in join!(&mut stats && &mut inventory && &leaders) {
                        let st = s.unwrap();
                        let inv = i.unwrap();
                        if l.unwrap().0 == selected_leader.0
                            && st.stats.get(&Stats::Gold).unwrap().value >= (shelf_item.1 as f64)
                        {
                            st.stats.get_mut(&Stats::Gold).unwrap().value -= shelf_item.1 as f64;
                            if let Err(e) =
                                inv.insert(ItemInstance::new(shelf_item.0, 1), item_defs)
                            {
                                eprintln!("Item purchasing failed: {:?}", e);
                            }
                        } else {
                            eprintln!("Item purchasing failed: Not enough gold.");
                        }
                    }
                }
            }
        }
    }
    Ok(())
}
