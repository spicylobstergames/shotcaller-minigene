use crate::*;

/// List of orders that can be given to a Unit
pub enum UnitOrder {
    /// Move to a given point.
    Move(Point)
}