use minigene::*;
use std::collections::HashMap;
use std::ops::Deref;

add_wasm_support!();

const PLAY_WIDTH: u32 = 81;
const PLAY_HEIGHT: u32 = 50;
const SCREEN_WIDTH: u32 = 100;
const SCREEN_HEIGHT: u32 = 50;
const CREEP_SPAWN_TICKS: u32 = 50;
const CREEP_ATTACK_RADIUS: f32 = 2.1;
const LEADER_ATTACK_RADIUS: f32 = 2.1;
const AOE_RADIUS: f32 = 4.0;
const TOWER_RANGE: f32 = 5.0;
const TOWER_PROJECTILE_EXPLOSION_RADIUS: f32 = 2.1;

const MAP: &[&str] = &[
    "####################################000000000####################################",
    "####################################000000000####################################",
    "####################################000000000####################################",
    "#########################0000000000000000000000000000000#########################",
    "#########################0000000000000000000000000000000#########################",
    "#########################0000000000000000000000000000000#########################",
    "#########################0000000000000000000000000000000#########################",
    "##################000000000000000000000000000000000000000000000##################",
    "##################000000000000000000000000000000000000000000000##################",
    "##################000000000000000000000000000000000000000000000##################",
    "##################00000###############00000###############00000##################",
    "##################00000###############00000###############00000##################",
    "##################00000###############00000###############00000##################",
    "##################00000###############00000###############00000##################",
    "##################00000###############00000###############00000##################",
    "##################00000###############00000###############00000##################",
    "##################00000###############00000###############00000##################",
    "##################00000###############00000###############00000##################",
    "##################00000###############00000###############00000##################",
    "##################00000###############00000###############00000##################",
    "##################00000###############00000###############00000##################",
    "##################00000###############00000###############00000##################",
    "##################00000###############00000###############00000##################",
    "##################00000###############00000###############00000##################",
    "##################00000###############00000###############00000##################",
    "##################00000###############00000###############00000##################",
    "##################00000###############00000###############00000##################",
    "##################00000###############00000###############00000##################",
    "##################00000###############00000###############00000##################",
    "##################00000###############00000###############00000##################",
    "##################00000###############00000###############00000##################",
    "##################00000###############00000###############00000##################",
    "##################00000###############00000###############00000##################",
    "##################00000###############00000###############00000##################",
    "##################00000###############00000###############00000##################",
    "##################00000###############00000###############00000##################",
    "##################00000###############00000###############00000##################",
    "##################00000###############00000###############00000##################",
    "##################00000###############00000###############00000##################",
    "##################00000###############00000###############00000##################",
    "##################000000000000000000000000000000000000000000000##################",
    "##################000000000000000000000000000000000000000000000##################",
    "##################000000000000000000000000000000000000000000000##################",
    "#########################0000000000000000000000000000000#########################",
    "#########################0000000000000000000000000000000#########################",
    "#########################0000000000000000000000000000000#########################",
    "#########################0000000000000000000000000000000#########################",
    "####################################000000000####################################",
    "####################################000000000####################################",
    "####################################000000000####################################",
];

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
pub struct Leader(u32);
#[derive(Component)]
pub struct Name(String);
#[derive(Component)]
pub struct SimpleMovement;
#[derive(Component)]
pub struct Creep;
#[derive(Component)]
pub struct CreepSpawner(u32, u32);
#[derive(Component)]
pub struct Base;
pub struct Spawner<F: Fn(&mut World)> {
    f: F,
}
#[derive(Component)]
pub struct Player;

#[derive(Component, PartialEq, Eq, Copy, Clone, Debug)]
pub enum Team {
    Me,
    Other,
}

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
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub enum SkillEvents {
    AOETrigger,
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub enum Effectors {
    DoubleDamage,
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub enum InputEvent {
    MenuNorth,
    MenuWest,
    MenuEast,
    MenuSouth,
    MenuSelect,
    MenuCancel,
    SpeedToggle,
    ZoomToggle,
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub enum Winner {
    None,
    Me,
    Other,
    Draw,
}

impl Default for Winner {
    fn default() -> Self {
        Winner::None
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum GameEvent {
    GameWon(Team),
}

// TODO: Replace by minigene's Time after its implemented
pub struct GameSpeed(pub u32);

impl Default for GameSpeed {
    fn default() -> Self {
        GameSpeed(1)
    }
}

// non portable
system!(UpdateCollisionResourceSystem, |global_map: WriteExpect<
    'a,
    CollisionResource,
>,
                                        positions: ReadStorage<
    'a,
    Point,
>,
                                        players: ReadStorage<
    'a,
    Player,
>| {
    for j in 0..(PLAY_HEIGHT as usize) {
        MAP[j].char_indices().for_each(|(i, c)| {
            if c == '#' {
                global_map.map.set(i as u32, j as u32);
            } else {
                global_map.map.unset(i as u32, j as u32);
            }
        });
    }
    // TODO fix this
    for (pos, _) in (&positions, &players).join() {
        global_map.position.x = pos.x - 40;
        global_map.position.y = pos.y - 25;
    }
});

system!(
    CreepSpawnerSystem,
    |entities: Entities<'a>,
     positions: WriteStorage<'a, Point>,
     spawners: WriteStorage<'a, CreepSpawner>,
     creeps: WriteStorage<'a, Creep>,
     simple_movements: WriteStorage<'a, SimpleMovement>,
     ai_destinations: WriteStorage<'a, AiDestination>,
     proximity_attacks: WriteStorage<'a, ProximityAttack>,
     stats: WriteStorage<'a, Comp<StatSet<Stats>>>,
     stat_def: ReadExpect<'a, StatDefinitions<Stats>>,
     ai_paths: WriteStorage<'a, AiPath>,
     teams: WriteStorage<'a, Team>,
     sprites: WriteStorage<'a, Sprite>| {
        let mut v = vec![];
        for (pos, mut spawner, team) in (&positions, &mut spawners, &teams).join() {
            if spawner.0 == 0 {
                spawner.0 = spawner.1;
                // spawn
                v.push((pos.clone(), team.clone()));
            }
            spawner.0 -= 1;
        }
        v.into_iter().for_each(|(pos, team)| {
            let creep = entities.create();
            positions.insert(creep, pos.clone()).unwrap();
            creeps.insert(creep, Creep).unwrap();
            simple_movements.insert(creep, SimpleMovement).unwrap();
            ai_paths
                .insert(creep, AiPath::new(NavigationPath::new()))
                .unwrap();
            teams.insert(creep, team).unwrap();
            stats.insert(creep, Comp(stat_def.to_statset())).unwrap();
            proximity_attacks
                .insert(creep, ProximityAttack::new(CREEP_ATTACK_RADIUS))
                .unwrap();
            let bg = if team == Team::Me {
                RGBA::named(GREEN)
            } else {
                RGBA::named(RED)
            };
            sprites
                .insert(
                    creep,
                    Sprite {
                        glyph: to_cp437('c'),
                        fg: RGBA::named(YELLOW),
                        bg,
                    },
                )
                .unwrap();
        });
    }
);

event_reader_res!(ToggleGameSpeedRes, InputEvent);

system!(
    ToggleGameSpeedSystem,
    |events: Read<'a, EventChannel<InputEvent>>,
     res: WriteExpect<'a, ToggleGameSpeedRes>,
     speed: Write<'a, GameSpeed>| {
        for k in events.read(&mut res.0) {
            if k == &InputEvent::SpeedToggle {
                if speed.0 == 1 {
                    speed.0 = 4;
                } else {
                    speed.0 = 1;
                }
            }
        }
    }
);

system!(
    WinConditionSystem,
    |core: ReadStorage<'a, Core>, team: ReadStorage<'a, Team>, winner: Write<'a, Winner>| {
        let mut me = false;
        let mut you = false;
        for (_, t) in (&core, &team).join() {
            match *t {
                Team::Me => me = true,
                Team::Other => you = true,
            }
        }
        match (me, you) {
            (false, false) => *winner = Winner::None,
            (false, true) => *winner = Winner::Other,
            (true, false) => *winner = Winner::Me,
            (true, true) => *winner = Winner::Draw,
        }
    }
);

system!(
    SimpleMovementSystem,
    |entities: Entities<'a>,
     simple_movements: ReadStorage<'a, SimpleMovement>,
     teams: ReadStorage<'a, Team>,
     targets: WriteStorage<'a, AiDestination>,
     stats: ReadStorage<'a, Comp<StatSet<Stats>>>,
     positions: ReadStorage<'a, Point>| {
        for (e, _, team, pos) in (&*entities, &simple_movements, &teams, &positions).join() {
            // find closest in other team
            // TODO: optimize
            let mut vec = (&teams, &positions, &stats)
                .join()
                .filter(|(t, _, _)| **t != *team)
                .map(|(_, p, _)| (dist(pos, p), p.clone()))
                .collect::<Vec<_>>();
            vec.sort_by(|e1, e2| e1.0.partial_cmp(&e2.0).unwrap());
            let closest = vec.into_iter().next().map(|(d, p)| p);
            if let Some(c) = closest {
                targets.insert(e, AiDestination::new(c.clone())).unwrap();
            } else {
                targets.remove(e);
            }
        }
    }
);

system!(
    TowerAiSystem,
    |entities: Entities<'a>,
     stat_def: ReadExpect<'a, StatDefinitions<Stats>>,
     towers: ReadStorage<'a, Tower>,
     teams: WriteStorage<'a, Team>,
     tower_projectiles: WriteStorage<'a, TowerProjectile>,
     sprites: WriteStorage<'a, Sprite>,
     stats: WriteStorage<'a, Comp<StatSet<Stats>>>,
     goto_positions: WriteStorage<'a, GotoStraight>,
     positions: WriteStorage<'a, Point>| {
        let mut v = vec![];
        for (_, team, pos) in (&towers, &teams, &positions).join() {
            // find closest in other team
            // TODO: optimize
            let mut vec = (&teams, &positions)
                .join()
                .filter(|(t, _)| **t != *team)
                .map(|(_, p)| (dist(pos, p), p.clone()))
                .filter(|(d, _)| *d < TOWER_RANGE)
                .collect::<Vec<_>>();
            vec.sort_by(|e1, e2| e1.0.partial_cmp(&e2.0).unwrap());
            let closest = vec.into_iter().next().map(|(d, p)| p);
            if let Some(c) = closest {
                v.push((pos.clone(), *team, c.clone()))
            }
        }
        for (source, team, target) in v.into_iter() {
            let n = entities.create();
            positions.insert(n, source).unwrap();
            tower_projectiles.insert(n, TowerProjectile).unwrap();
            teams.insert(n, team).unwrap();
            stats.insert(n, Comp(stat_def.to_statset())).unwrap();
            sprites
                .insert(
                    n,
                    Sprite {
                        glyph: to_cp437('X'),
                        fg: RGBA::named(RED),
                        bg: RGBA::named(WHITE),
                    },
                )
                .unwrap();
            goto_positions
                .insert(n, GotoStraight::new(target.clone(), 1.0))
                .unwrap();
        }
    }
);

system!(ProximityAttackSystem, |entities: Entities<'a>,
                                proximity_attacks: ReadStorage<
    'a,
    ProximityAttack,
>,
                                stats: WriteStorage<
    'a,
    Comp<StatSet<Stats>>,
>,
                                teams: ReadStorage<'a, Team>,
                                positions: ReadStorage<
    'a,
    Point,
>| {
    let mut v = vec![];
    for (e, proximity, stat, pos, team) in
        (&*entities, &proximity_attacks, &stats, &positions, &teams).join()
    {
        let mut vec = (&*entities, &teams, &positions, &stats)
            .join()
            .filter(|(e, t, _, _)| **t != *team)
            .map(|(e, _, p, _)| (dist(pos, p), e))
            .filter(|(d, _)| *d < proximity.radius)
            .collect::<Vec<_>>();
        vec.sort_by(|e1, e2| e1.0.partial_cmp(&e2.0).unwrap());
        let closest = vec.into_iter().next().map(|(d, p)| p);
        if let Some(target) = closest {
            let damage = stat.0.stats.get(&Stats::Attack).unwrap().value;
            v.push((e.clone(), target.clone(), damage));
        }
    }

    for (attacker, target, dmg) in v.into_iter() {
        increment_attacks_dealt(&mut stats.get_mut(attacker).unwrap().0);
        if damage(&mut stats.get_mut(target).unwrap().0, dmg) {
            entities.delete(target).unwrap();
        }
    }
});

system!(TowerProjectileSystem, |projectiles: ReadStorage<
    'a,
    TowerProjectile,
>,
                                entities: Entities<'a>,
                                positions: ReadStorage<
    'a,
    Point,
>,
                                teams: ReadStorage<'a, Team>,
                                gotos: ReadStorage<
    'a,
    GotoStraight,
>,
                                stats: WriteStorage<
    'a,
    Comp<StatSet<Stats>>,
>| {
    for (e, pos, goto, _, team) in (&*entities, &positions, &gotos, &projectiles, &teams).join() {
        let dmg = stats
            .get(e)
            .expect("Add a statset to the projectile.")
            .0
            .stats
            .get(&Stats::Attack)
            .unwrap()
            .value;
        if *pos == goto.target {
            for (e, _, _) in entities_in_radius(
                pos,
                &*entities,
                &positions,
                |e, p| teams.get(e).map(|t| t != team).unwrap_or(false),
                |e, p, d| d <= TOWER_PROJECTILE_EXPLOSION_RADIUS,
            ) {
                // damage around
                if let Some(mut stat) = stats.get_mut(e).as_mut().map(|c| &mut c.0) {
                    if damage(&mut stat, dmg) {
                        entities.delete(e).unwrap();
                    }
                }
            }
        }
    }
});

system!(UpdateEnemiesAroundSystem, |entities: Entities<'a>,
                                    positions: ReadStorage<
    'a,
    Point,
>,
                                    teams: ReadStorage<'a, Team>,
                                    stats: WriteStorage<
    'a,
    Comp<StatSet<Stats>>,
>| {
    for (e, pos, stat, team) in (&*entities, &positions, &mut stats, &teams).join() {
        let c = entities_in_radius(
            pos,
            &*entities,
            &positions,
            |e, _| teams.get(e).map(|t| t != team).unwrap_or(false),
            |_, _, d| d <= AOE_RADIUS,
        )
        .len() as f64;
        stat.0
            .stats
            .get_mut(&Stats::EnemiesAround)
            .expect("Failed to get EnemiesAround stat")
            .value = c;
    }
});

pub fn increment_attacks_dealt(stat_set: &mut StatSet<Stats>) {
    stat_set.stats.get_mut(&Stats::AttacksDealt).unwrap().value += 1.0;
}

pub fn damage(stat_set: &mut StatSet<Stats>, damage: f64) -> bool {
    let mut health_inst = stat_set.stats.get_mut(&Stats::Health).unwrap();
    health_inst.value -= damage;
    health_inst.value <= 0.0
}

pub fn entities_in_radius<
    D: Deref<Target = MaskedStorage<Point>>,
    F1: Fn(Entity, Point) -> bool,
    F2: Fn(Entity, Point, f32) -> bool,
>(
    around: &Point,
    entities: &EntitiesRes,
    positions: &Storage<'_, Point, D>,
    pre_filter: F1,
    post_filter: F2,
) -> Vec<(Entity, Point, f32)> {
    let mut vec = (&*entities, positions)
        .join()
        .filter(|(e, p)| pre_filter(*e, **p))
        .map(|(e, p)| (e, p.clone(), dist(around, p)))
        .filter(|(e, p, d)| post_filter(*e, *p, *d))
        .collect::<Vec<_>>();
    // Sort by distance
    vec.sort_by(|e1, e2| e1.2.partial_cmp(&e2.2).unwrap());
    vec
}

fn render<'a>(ctx: &mut BTerm) {
    ctx.cls();
    let mut i = 0;
    for s in MAP {
        ctx.print(0, i, s);
        i = i + 1;
    }
}

struct State {
    pub world: World,
    pub dispatcher: Box<dyn UnifiedDispatcher + 'static>,
}
impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        // Input
        let input = INPUT.lock();
        for key in input.key_pressed_set().iter() {
            self.world
                .fetch_mut::<EventChannel<VirtualKeyCode>>()
                .single_write(key.clone());
        }
        self.dispatcher.run_now(&mut self.world);
        render(ctx);
        render_sprites(
            ctx,
            &self.world.read_resource(),
            self.world.read_storage(),
            self.world.read_storage(),
            self.world.read_storage(),
        );
        self.world.maintain();

        #[cfg(not(target_arch = "wasm32"))]
        std::thread::sleep(std::time::Duration::from_millis(
            (50 / self.world.fetch::<GameSpeed>().0) as u64,
        ));
    }
}

fn main() -> BError {
    let mut world = World::new();
    let mut builder = dispatcher!(
        world,
        (CombineCollisionSystem, "combine_collision", &[]),
        (InputDriver::<InputEvent>, "input_driver", &[]),
        (
            UpdateCollisionResourceSystem,
            "update_collision_res",
            &["combine_collision"],
        ),
        (CreepSpawnerSystem, "creep_spawner", &[]),
        (AiPathingSystem, "ai_pathing", &["update_collision_res"]),
        (AiMovementSystem, "ai_movement", &["ai_pathing"]),
        (ToggleGameSpeedSystem, "toggle_speed", &["input_driver"]),
        (WinConditionSystem, "win_cond", &[]),
        (SimpleMovementSystem, "simple_movement", &[]),
        (TowerAiSystem, "tower_ai", &[]),
        (ProximityAttackSystem, "proximity_attack", &[]),
        (TowerProjectileSystem, "tower_projectile", &[]),
        (UpdateEnemiesAroundSystem, "update_enemies_around", &[]),
        (GotoStraightSystem, "goto_straight", &[])
    );
    let (mut world, mut dispatcher, mut context) =
        mini_init(SCREEN_WIDTH, SCREEN_HEIGHT, "Shotcaller", builder, world);

    world.register::<MultiSprite>();
    world.register::<Sprite>();
    world.register::<Team>();
    world.register::<Barrack>();
    world.register::<Tower>();
    world.register::<Core>();
    world.register::<Leader>();
    world.register::<Name>();
    world.register::<Comp<StatSet<Stats>>>();

    // WASM REGISTER
    world.register::<Point>();
    world.register::<SimpleMovement>();
    world.register::<AiPath>();
    world.register::<AiDestination>();
    world.register::<Creep>();
    world.register::<Player>();
    world.register::<CollisionMap>();
    world.register::<CreepSpawner>();
    world.register::<Collision>();
    world.insert(GameSpeed::default());

    let mut input_channel = EventChannel::<VirtualKeyCode>::new();
    let reader = input_channel.register_reader();
    world.insert(input_channel);
    world.insert(InputDriverRes(reader));

    let mut keymap = HashMap::new();
    keymap.insert(VirtualKeyCode::J, InputEvent::MenuSouth);
    keymap.insert(VirtualKeyCode::K, InputEvent::MenuNorth);
    keymap.insert(VirtualKeyCode::H, InputEvent::MenuWest);
    keymap.insert(VirtualKeyCode::L, InputEvent::MenuEast);
    keymap.insert(VirtualKeyCode::Return, InputEvent::MenuSelect);
    keymap.insert(VirtualKeyCode::Q, InputEvent::MenuCancel);
    keymap.insert(VirtualKeyCode::S, InputEvent::SpeedToggle);
    world.insert(keymap);

    let mut input_channel = EventChannel::<InputEvent>::new();
    let reader = input_channel.register_reader();
    world.insert(input_channel);
    world.insert(ToggleGameSpeedRes(reader));

    world.insert(Camera::new(
        Point::new(0, 0),
        Point::new(PLAY_WIDTH, PLAY_HEIGHT),
    ));

    let stat_defs = StatDefinitions::from(vec![
        StatDefinition::new(
            Stats::Health,
            String::from("health"),
            String::from("HP"),
            100.0,
        ),
        StatDefinition::new(
            Stats::Defense,
            String::from("defense"),
            String::from("Defense"),
            0.0,
        ),
        StatDefinition::new(
            Stats::Attack,
            String::from("attack"),
            String::from("Attack"),
            10.0,
        ),
        StatDefinition::new(
            Stats::EnemiesAround,
            String::from("enemies_around"),
            String::from("Enemies Around"),
            0.0,
        ),
        StatDefinition::new(
            Stats::AttacksDealt,
            String::from("attacks_dealt"),
            String::from("Attacks Dealt"),
            0.0,
        ),
        StatDefinition::new(
            Stats::AttackSpeed,
            String::from("attack_speed"),
            String::from("Attack Speed"),
            10.0,
        ),
        StatDefinition::new(Stats::Mana, String::from("mana"), String::from("MP"), 100.0),
    ]);
    let default_stats = stat_defs.to_statset();

    let skill_definitions = SkillDefinitions::<_, _, _, (), _>::from(vec![
        SkillDefinition::new(
            Skills::AOE,
            String::from("AOE"),
            String::from("aoe"),
            String::from("Does 100 damage to all enemy entities around. Actives only if 3 or more enemy entities are present. Cooldown of 12s."),
            12.0,
            true,
            vec![
                // enemies around >= 3
                StatCondition::new(
                    Stats::EnemiesAround,
                    StatConditionType::MinValue(3.0),
                ),
            ],
            vec![],
            vec![],
            vec![
                SkillEvents::AOETrigger,
            ],
        ),
        SkillDefinition::new(
            Skills::DoubleDamage,
            String::from("Double Damage"),
            String::from("double_damage"),
            String::from("Each 3 attacks, deal double damage."),
            0.0,
            true,
            vec![
                StatCondition::new(
                    Stats::AttacksDealt,
                    StatConditionType::Custom(std::sync::Arc::new(Box::new(|v| v as i32 % 3 == 0))),
                ),
            ],
            vec![],
            vec![
                Effectors::DoubleDamage,
            ],
            vec![],
        ),
    ]);

    // player
    // TODO remove
    //world
    //    .create_entity()
    //    .with(Point::new(0, 0))
    //    .with(MultiSprite::new(MultiTileSprite::from_string("@@", 1, 2)))
    //    .with(Comp(stat_defs.to_statset()))
    //    //.with(Player)
    //    .build();

    world.insert(stat_defs);
    world.insert(CollisionResource::new(
        CollisionMap::new(PLAY_WIDTH, PLAY_HEIGHT),
        Point::new(0, 0),
    ));

    // Create cores
    world
        .create_entity()
        .with(Point::new(PLAY_WIDTH as i32 / 2, 1))
        .with(Sprite {
            glyph: to_cp437('C'),
            fg: RGBA::named(BLUE),
            bg: RGBA::named(RED),
        })
        .with(Team::Other)
        .with(Core)
        .with(Comp(default_stats.clone()))
        .build();

    world
        .create_entity()
        .with(Point::new(PLAY_WIDTH as i32 / 2, PLAY_HEIGHT as i32 - 2))
        .with(Sprite {
            glyph: to_cp437('C'),
            fg: RGBA::named(BLUE),
            bg: RGBA::named(GREEN),
        })
        .with(Team::Me)
        .with(Core)
        .with(Comp(default_stats.clone()))
        .build();

    // Create barracks
    for i in -1..=1 {
        let x = PLAY_WIDTH as i32 / 2 + PLAY_WIDTH as i32 / 7 * i as i32;
        let y = PLAY_HEIGHT as i32 / 8;
        world
            .create_entity()
            .with(Point::new(x, y))
            .with(Sprite {
                glyph: to_cp437('B'),
                fg: RGBA::named(YELLOW),
                bg: RGBA::named(RED),
            })
            .with(Team::Other)
            .with(Barrack)
            .with(Comp(default_stats.clone()))
            .build();
        // Creep spawners
        world
            .create_entity()
            .with(Point::new(x, y + 1))
            .with(CreepSpawner(0, CREEP_SPAWN_TICKS))
            .with(Team::Other)
            .build();
    }

    for i in -1..=1 {
        let x = PLAY_WIDTH as i32 / 2 + PLAY_WIDTH as i32 / 7 * i;
        let y = PLAY_HEIGHT as i32 - 1 - PLAY_HEIGHT as i32 / 8;
        world
            .create_entity()
            .with(Point::new(x, y))
            .with(Sprite {
                glyph: to_cp437('B'),
                fg: RGBA::named(YELLOW),
                bg: RGBA::named(GREEN),
            })
            .with(Team::Me)
            .with(Barrack)
            .with(Comp(default_stats.clone()))
            .build();
        // Creep spawners
        world
            .create_entity()
            .with(Point::new(x, y - 1))
            // TODO put back to normal
            .with(CreepSpawner(0, CREEP_SPAWN_TICKS - 5))
            .with(Team::Me)
            .build();
    }

    // Create towers
    for i in -1..=1 {
        for j in 1..=2 {
            world
                .create_entity()
                .with(Point::new(
                    PLAY_WIDTH as i32 / 2 + PLAY_WIDTH as i32 / 4 * i,
                    PLAY_HEIGHT as i32 * j / 6,
                ))
                .with(Sprite {
                    glyph: to_cp437('T'),
                    fg: RGBA::named(GREEN),
                    bg: RGBA::named(RED),
                })
                .with(Team::Other)
                .with(Comp(default_stats.clone()))
                .build();
        }
    }

    for i in -1..=1 {
        for j in 1..=2 {
            world
                .create_entity()
                .with(Point::new(
                    PLAY_WIDTH as i32 / 2 + PLAY_WIDTH as i32 / 4 * i,
                    PLAY_HEIGHT as i32 - 1 - PLAY_HEIGHT as i32 * j / 6,
                ))
                .with(Sprite {
                    glyph: to_cp437('T'),
                    fg: RGBA::named(GREEN),
                    bg: RGBA::named(GREEN),
                })
                .with(Team::Me)
                .with(Comp(default_stats.clone()))
                .build();
        }
    }

    // Create generic hero 1
    world
        .create_entity()
        .with(Point::new(PLAY_WIDTH as i32 / 2, PLAY_HEIGHT as i32 - 11))
        .with(Sprite {
            glyph: to_cp437('L'),
            fg: RGBA::named(YELLOW),
            bg: RGBA::named(GREEN),
        })
        .with(Team::Me)
        .with(SimpleMovement)
        .with(AiPath::new(NavigationPath::new()))
        .with(Leader(0))
        .with(ProximityAttack::new(LEADER_ATTACK_RADIUS))
        .with(Name("Generic Leader 1".to_string()))
        .with(Comp(default_stats.clone()))
        .build();

    //for i in 10..30 {
    //    world.create_entity()
    //        .with(Point::new(i, 49))
    //        .with(CreepSpawner(i))
    //        .build();
    //    world.create_entity()
    //        .with(Point::new(i, 1))
    //        .with(CreepSpawner(i))
    //        .build();
    //}

    let gs = State { world, dispatcher };

    main_loop(context, gs)
}
