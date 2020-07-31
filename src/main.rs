use minigene::*;
use std::collections::HashMap;

add_wasm_support!();

const PLAY_WIDTH: u32 = 81;
const PLAY_HEIGHT: u32 = 50;
const SCREEN_WIDTH: u32 = 100;
const SCREEN_HEIGHT: u32 = 50;
const CREEP_SPAWN_TICKS: u32 = 50;

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
#[derive(Component)]
pub struct Core;
#[derive(Component)]
pub struct Barrack;
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
    None, Me, Other, Draw,
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
     ai_destinations: WriteStorage<'a, AiDestination>,
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
            ai_paths
                .insert(creep, AiPath::new(NavigationPath::new()))
                .unwrap();
            teams.insert(creep, team).unwrap();
            sprites
                .insert(
                    creep,
                    Sprite {
                        glyph: to_cp437('c'),
                        fg: RGBA::named(YELLOW),
                        bg: RGBA::named(BLACK),
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
            (false, true) =>  *winner = Winner::Other,
            (true, false) =>  *winner = Winner::Me,
            (true, true) =>   *winner = Winner::Draw,
        }
});

system!(
    CreepAi,
    |entities: Entities<'a>, creeps: ReadStorage<'a, Creep>, teams: ReadStorage<'a, Team>, targets: WriteStorage<'a, AiDestination>, positions: ReadStorage<'a, Point>| {
        for (e, _, team, pos) in (&*entities, &creeps, &teams, &positions).join() {
            // find closest in other team
            // TODO: optimize
            let mut vec = (&teams, &positions).join().filter(|(t, _)| **t != *team).map(|(_, p)| (dist(pos, p), p.clone())).collect::<Vec<_>>();
            vec.sort_by(|e1, e2| e1.0.partial_cmp(&e2.0).unwrap());
            let closest = vec.into_iter().next().map(|(d, p)| p);
            if let Some(c) = closest {
                targets.insert(e, AiDestination::new(c.clone())).unwrap();
            } else {
                targets.remove(e);
            }
        }
});

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
        //self.world.insert(ctx.key.clone());
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
        (WinConditionSystem, "win_cond", &[])
    );
    let (mut world, mut dispatcher, mut context) =
        mini_init(SCREEN_WIDTH, SCREEN_HEIGHT, "Shotcaller", builder, world);

    world.register::<MultiSprite>();
    world.register::<Sprite>();
    world.register::<Team>();
    world.register::<Barrack>();
    world.register::<Tower>();
    world.register::<Core>();
    world.register::<Comp<StatSet<Stats>>>();

    // WASM REGISTER
    world.register::<Point>();
    world.register::<AiPath>();
    world.register::<AiDestination>();
    world.register::<Creep>();
    world.register::<Player>();
    world.register::<CollisionMap>();
    world.register::<CreepSpawner>();
    world.register::<Tower>();
    world.register::<Barrack>();
    world.register::<Core>();
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
            Stats::AttackSpeed,
            String::from("attack_speed"),
            String::from("Attack Speed"),
            10.0,
        ),
        StatDefinition::new(Stats::Mana, String::from("mana"), String::from("MP"), 100.0),
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
        .build();

    world
        .create_entity()
        .with(Point::new(PLAY_WIDTH as i32 / 2, PLAY_HEIGHT as i32 - 2))
        .with(Sprite {
            glyph: to_cp437('C'),
            fg: RGBA::named(BLUE),
            bg: RGBA::named(RED),
        })
        .with(Team::Me)
        .with(Core)
        .build();

    // Create barracks
    for i in -1..=1 {
        let x = PLAY_WIDTH as i32 / 2 + PLAY_WIDTH as i32 / 8 * i as i32;
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
        let x = PLAY_WIDTH as i32 / 2 + PLAY_WIDTH as i32 / 8 * i;
        let y = PLAY_HEIGHT as i32 - 1 - PLAY_HEIGHT as i32 / 8;
        world
            .create_entity()
            .with(Point::new(
                PLAY_WIDTH as i32 / 2 + PLAY_WIDTH as i32 / 8 * i,
                PLAY_HEIGHT as i32 - 1 - PLAY_HEIGHT as i32 / 8,
            ))
            .with(Sprite {
                glyph: to_cp437('B'),
                fg: RGBA::named(YELLOW),
                bg: RGBA::named(RED),
            })
            .with(Team::Me)
            .with(Barrack)
            .build();
        // Creep spawners
        world
            .create_entity()
            .with(Point::new(x, y - 1))
            .with(CreepSpawner(0, CREEP_SPAWN_TICKS))
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
                .with(Team::Me)
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
                    bg: RGBA::named(RED),
                })
                .with(Team::Other)
                .build();
        }
    }

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
