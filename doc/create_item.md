As of 0.4.0
# Creating An Item

- Add an item ID in the `Items` enum.
- Create a new `ItemDefinition` for your item in the `assets/item_defs.yaml` file.

An `ItemDefinition` is composed of the following:
- The newly defined item ID
- The "slot" of this item. This is used to limit which item can go into which inventory space. This is not currently used in Shotcaller so we assign `()`.
- The displayed name of the item.
- The friendly name of the item. This is a name that is easier to work with when debugging.
- The displayed description of the item.
- The maximum stack of the item. For a MOBA, Some(1) would be used, as it limits the inventory to contain only one item per slot.
- The maximum durability. Usually None for a MOBA, as items don't break.
- Custom user data, if needed, usually `()`.

## Applying an effect when the item is present
Finally, if you want to apply an effect when an item is in the inventory of an entity, here is how to do it:
- Create a passive skill which uses this item as a condition for activating.
- Create an effector definition in the `assets/effector_defs.yaml` file which modifies the desired stat(s).
- Add the effector to this skill's definition.
