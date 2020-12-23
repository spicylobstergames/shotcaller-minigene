use crate::*;

pub struct Tower;
#[derive(new)]
pub struct ProximityAttack {
    pub radius: f32,
}
#[derive(new)]
pub struct Hero1ProximityAttack {
    pub radius: f32,
}
pub struct TowerProjectile;
pub struct Core;
pub struct Barrack;
pub struct Leader(pub u8);
pub struct Name(pub String);
pub struct SimpleMovement;
pub struct Hero1SimpleMovement;
pub struct FleeToBase(pub f64);
pub struct IsCaught(pub bool);
pub struct Creep;
pub struct CreepSpawner(pub u32, pub u32);
pub struct Base;
pub struct Spawner<F: Fn(&mut World)> {
    pub f: F,
}
pub struct Player;

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum Team {
    Me,
    Other,
}
