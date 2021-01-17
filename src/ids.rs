/// The different stats ids.
#[allow(missing_docs)]
#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub enum Stats {
    Health,
    Defense,
    Attack,
    Mana,
    AttackSpeed,
    EnemiesAround,
    LeadersAround,
    AttacksDealt,
    AttacksReceived,
    DamageDealt,
    DamageReceived,
    ActionPoints,
    ActionPointRefillRate,
    Gold,
    GoldGainMultiplier,
}

impl Default for Stats {
    fn default() -> Self {
        Self::Health
    }
}

/// The different skill ids.
#[allow(missing_docs)]
#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub enum Skills {
    AOE,
    DoubleDamage,
    DoubleAttackSpeed,
    NatureSummon,
    Root,
    AttackSpeedIncrease,
    SlowAOE,
    BearSummon,
    Savagery,
    ReturnAOE,
    BattleHunger,
    GreedyTouch,
    AirCorrosion,
    SpellSteal,
    Telekinesis,
}

/// The different items ids.
#[allow(missing_docs)]
#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub enum Items {
    TestItem,
    Coffee,
}

/// The different effectors ids.
#[allow(missing_docs)]
#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub enum Effectors {
    DoubleDamage,
    DoubleAttackSpeed,
    Root,
    AttackSpeedIncrease,
    HalfMovementSpeed,
    Savagery,
    BattleHungry,
    Enraged,
    HalfDefense,
    DoubleGoldGain,
    Stun,
}

/// The different leader ids.
#[allow(missing_docs)]
#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub enum Leaders {
    Generic1,
    Generic2,
    Generic3,
    TreePersonLeader,
    BearPersonLeader,
    AxePersonLeader,
    Celsus,
    Erno,
}
