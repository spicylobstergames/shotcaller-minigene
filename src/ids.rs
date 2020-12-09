use crate::*;

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub enum Stats {
    Health,
    Defense,
    Attack,
    Mana,
    AttackSpeed,
    EnemiesAround,
    AttacksDealt,
    AttacksReceived,
    DamageDealt,
    DamageReceived,
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub enum Skills {
    AOE,
    DoubleDamage,
    DoubleAttackSpeed
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub enum Items {
    TestItem,
    Coffee
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub enum Effectors {
    DoubleDamage,
    DoubleAttackSpeed
}
