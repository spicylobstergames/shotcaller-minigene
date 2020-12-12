use crate::*;

#[derive(Component)]
pub struct Tower;
#[derive(Component, new)]
pub struct ProximityAttack {
    pub radius: f32,
}
#[derive(Component)]
pub struct TowerProjectile;
#[derive(Component)]
pub struct Core;
#[derive(Component)]
pub struct Barrack;
#[derive(Component)]
pub struct Leader(pub u8);
#[derive(Component)]
pub struct Name(pub String);
#[derive(Component)]
pub struct SimpleMovement;
#[derive(Component)]
pub struct Hero1SimpleMovement;
#[derive(Component)]
pub struct Creep;
#[derive(Component)]
pub struct CreepSpawner(pub u32, pub u32);
#[derive(Component)]
pub struct Base;
pub struct Spawner<F: Fn(&mut World)> {
    pub f: F,
}
#[derive(Component)]
pub struct Player;

#[derive(Component, PartialEq, Eq, Copy, Clone, Debug)]
pub enum Team {
    Me,
    Other,
}
