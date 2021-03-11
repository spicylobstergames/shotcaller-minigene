use crate::*;

/// List of orders that can be given to a Unit
pub enum UnitOrder {
    /// Move to a given point.
    MovetoPoint(Point),
    /// Go to or follow an entity.
    MovetoUnit(Entity),
    /// Don't move. Whatever happens, don't move. Even if you are being killed, don't move!
    HoldPosition,
}
