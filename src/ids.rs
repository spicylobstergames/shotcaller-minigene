/// The different stats ids.
#[allow(missing_docs)]
#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub enum Stats {
    Health,
    Defense,
    Attack,
    AggroRange,
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
    Souls,
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
    ElephantSummon,
    Savagery,
    ReturnAOE,
    BattleHunger,
    GreedyTouch,
    AirCorrosion,
    SpellSteal,
    Telekinesis,
    StealSoul,
    DarkPresence,
    ReturnDamage,
    BattleCall,
    ThornVolley,
    BackEndurance,
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
    AdditionalAttack,
    AdditionalDefense,
}

/// The different leader ids.
#[allow(missing_docs)]
#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub enum Leaders {
    Generic1,
    Generic2,
    TreePersonLeader,
    Raja,
    AxePersonLeader,
    CentaurPersonLeader,
    Celsus,
    Erno,
    SoulsCollector,
    BristlebackPersonLeader,
}

#[allow(missing_docs)]
#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub enum TileMapping {
    Elephant1,
    Elephant2,
    Elephant3,
    FatMan1,
    FatMan2,
    FatMan3,
    SwordMan1,
    SwordMan2,
    SwordMan3,
    Archer1,
    Archer2,
    Archer3,
    Axe1,
    Axe2,
    Axe3,
    Lance1,
    Lance2,
    Lance3,
    SwordSmallMan1,
    SwordSmallMan2,
    SwordSmallMan3,
    Beer,
    Forest,
    Core,
    Barrack,
    Tower1,
    Tower2,
    Creep,
    Tree1,
    Fireball,
}

impl Into<usize> for TileMapping {
    fn into(self) -> usize {
        match self {
            TileMapping::Elephant1 => 0,
            TileMapping::Elephant2 => 1,
            TileMapping::Elephant3 => 2,
            TileMapping::FatMan1 => 3,
            TileMapping::FatMan2 => 4,
            TileMapping::FatMan3 => 5,
            TileMapping::SwordMan1 => 6,
            TileMapping::SwordMan2 => 7,
            TileMapping::SwordMan3 => 8,
            TileMapping::Archer1 => 9,
            TileMapping::Archer2 => 10,
            TileMapping::Archer3 => 11,
            TileMapping::Axe1 => 12,
            TileMapping::Axe2 => 13,
            TileMapping::Axe3 => 14,
            TileMapping::Lance1 => 15,
            TileMapping::Lance2 => 16,
            TileMapping::Lance3 => 17,
            TileMapping::SwordSmallMan1 => 18,
            TileMapping::SwordSmallMan2 => 19,
            TileMapping::SwordSmallMan3 => 20,
            TileMapping::Beer => 21,
            TileMapping::Forest => 22,
            TileMapping::Core => 23,
            TileMapping::Barrack => 26,
            TileMapping::Tower1 => 24,
            TileMapping::Tower2 => 25,
            // TODO
            TileMapping::Creep => 19,
            TileMapping::Tree1 => 12,
            TileMapping::Fireball => 0,
        }
    }
}
