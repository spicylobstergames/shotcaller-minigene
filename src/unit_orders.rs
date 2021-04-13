use crate::*;

/// List of orders that can be given to a Unit
pub enum UnitOrder {
    /// Attack move order (A-Move). Go to position, stop to murder any enemies on the way there.
    AMovetoPoint(Point),
    /// Move to a given point.
    MovetoPoint(Point),
    /// Go to or follow an entity.
    MovetoUnit(Entity),
    /// Don't move. Whatever happens, don't move. Even if you are being killed, don't move!
    HoldPosition,
}
