//! The shotcaller game. A new MOBA!
#![warn(missing_docs)]

#[macro_use]
extern crate serde;

#[macro_use]
extern crate lazy_static;

use minigene::*;
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use uuid::*;

use nakama_rs::api_client::{ApiClient, Event};
use nakama_rs::matchmaker::{Matchmaker, QueryItemBuilder};

add_wasm_support!();

const PLAY_WIDTH: u32 = 81;
const PLAY_HEIGHT: u32 = 50;
const SCREEN_WIDTH: u32 = 100;
const SCREEN_HEIGHT: u32 = 50;
//const MAP_SIZE_X: u32 = 324;
//const MAP_SIZE_Y: u32 = 200;
const MAP_SIZE_X: u32 = 162;
const MAP_SIZE_Y: u32 = 100;
//const MAP_SIZE_X: u32 = 81;
//const MAP_SIZE_Y: u32 = 50;
const CREEP_SPAWN_TICKS: f32 = 125.0;
const CREEP_ATTACK_RADIUS: f32 = 2.1;
const MELEE_LEADER_ATTACK_RADIUS: f32 = 2.1;
const RANGED_LEADER_ATTACK_RADIUS: f32 = 21.0;
const AOE_RADIUS: f32 = 4.0;
const AOE_DAMAGE: f64 = 100.0;
const SLOW_AOE_RADIUS: f32 = 8.0;
const SLOW_AOE_DAMAGE: f64 = 50.0;
const RETURN_AOE_RADIUS: f32 = 4.0;
const RETURN_AOE_DAMAGE: f64 = 20.0;
const STUN_AOE_RADIUS: f32 = 4.0;
const TOWER_RANGE: f32 = 5.0;
const TOWER_PROJECTILE_EXPLOSION_RADIUS: f32 = 2.1;
const TARGET_FPS: f32 = 20.0;
const ACTION_POINT_MOVE_COST: f64 = 100.0;
//const ACTION_POINT_ATTACK_COST: f64 = 50.0;
const LEADER_SPAWN_POINT_ME: (i32, i32) = (MAP_SIZE_X as i32 / 2, MAP_SIZE_Y as i32 - 22);
const LEADER_SPAWN_POINT_OTHER: (i32, i32) = (MAP_SIZE_X as i32 / 2, 22);

const BARRACK_OFFSET: i32 = 64;
const BARRACK_HEIGHT_FROM_EDGE: i32 = 6;
const TOWER_OFFSET: i32 = 64;

//const MAP: &[u8; 4100] = include_bytes!("../assets/map.txt");
const MAP: &[u8; 16300] = include_bytes!("../assets/map2x.txt");
//const MAP: &[u8; 65000] = include_bytes!("../assets/map4x.txt");

lazy_static! {
    static ref MAP_STRING: Vec<String> = String::from_utf8(MAP.to_vec())
        .unwrap()
        .split("\n")
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
}

mod components;
mod events;
mod ids;
mod leaders;
mod nakama;
mod render_map;
mod resources;
mod states;
mod systems;
mod unit_orders;
mod utils;
pub use self::components::*;
pub use self::events::*;
pub use self::ids::*;
pub use self::leaders::*;
pub use self::render_map::*;
pub use self::resources::*;
pub use self::states::*;
pub use self::systems::*;
pub use self::unit_orders::*;
pub use self::utils::*;
use nakama::*;

pub struct GameData {
    pub world: World,
    pub client_dispatcher: Dispatcher,
    pub host_network: Dispatcher,
    pub client_network: Dispatcher,
}

type PostUpdate = fn(&mut GameData, &Time);
struct State {
    pub engine: Engine<GameData, PostUpdate>,
}
impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        let mut close_requested = false;
        let mut input = INPUT.lock();
        while let Some(ev) = input.pop() {
            match ev {
                /*BEvent::KeyboardInput {key, ..} => {
                    //world.get_mut::<Vec<>>().unwrap().push(key);
                    //println!("kb event");
                }*/
                BEvent::Character { c } => {
                    //println!("Input: {}", c);
                    self.engine
                        .state_data
                        .world
                        .get_mut::<Vec<char>>()
                        .unwrap()
                        .push(c);
                }
                BEvent::CloseRequested => close_requested = true,
                _ => {}
            }
        }
        main_render(ctx, &mut self.engine.state_data);
        self.engine.engine_frame(true);
        if close_requested || !self.engine.state_machine.is_running() {
            ctx.quitting = true;
            return;
        }
    }
}

macro_rules! dispatcher {
    ($dispatch:ident, $($sys:expr),*$(,)?) => {
        $($dispatch = $dispatch.add($sys);)*
    }
}

fn main() -> BError {
    // Load spritesheet
    #[cfg(feature = "wasm")]
    add_embed!(
        "../assets/tilemap/colored_tilemap_packed.png",
        "../assets/tilemap/new.png",
        "../assets/skill_defs.yaml",
        "../assets/effector_defs.yaml",
        "../assets/keymap.yaml",
        "../assets/item_defs.yaml",
        "../assets/stat_defs.yaml",
        "../assets/leader_defs.yaml"
    );
    let mut world = World::default();
    let mut dispatcher = DispatcherBuilder::new();
    dispatcher!(
        dispatcher,
        fog_of_vision_system,
        combine_collision_system,
        input_driver::<InputEvent>,
        update_mouse_events_system,
        order_generation_system,
        unit_selection_system,
        control_group_system,
        update_input_state_system, // should run before all other input systems
        update_collision_resource_system,
        handle_action_points_system,
        creep_spawner_system,
        idle_order_system,
        mmove_order_system,
        amove_order_system,
        holdpos_order_system,
        simple_destination_system,
        ai_pathing_system,
        movement_system,
        toggle_game_speed_system,
        toggle_game_mode_system,
        win_condition_system,
        //leader1_simple_movement_system, // TODO re-enable & rewrite like simple_destination_system
        //leader2_simple_movement_system, // TODO re-enable & rewrite like simple_destination_system
        tower_ai_system,
        proximity_attack_system,
        //leader1_proximity_attack_system, // TODO re-enable
        tower_projectile_system,
        update_enemies_around_system,
        update_leaders_around_system,
        skill_cooldown_system::<Skills>,
        trigger_passive_skill_system::<Stats, Effectors, Skills, Items, (), ()>,
        exec_skill_system::<Stats, Effectors, Skills, Items>,
        apply_effector_system::<Stats, Effectors>,
        remove_outdated_effector_system::<Effectors>,
        nature_summon_system,
        elephant_spawner_system,
        savagery_system,
        battle_hunger_system,
        air_corrosion_system,
        telekinesis_system,
        spell_steal_system,
        dark_presence_system,
        additional_attack_system,
        additional_defense_system,
        aoe_damage_system,
        damage_entity_system,
        return_damage_system,
        back_endurance_system,
        steal_soul_system,
        transfer_gold_system,
        kill_entity_system,
        goto_straight_system,
        select_leader_system,
        select_shelf_item_system,
        item_purchasing_system,
        leader_teleport_system,
        root_system,
        respawn_leader_driver,
        spawn_creep_system,
        spawn_leader_system,
        game_stats_updater_system,
        order_completion_check_system,
        event_retrigger_system::<InputEvent, MoveCameraEvent>,
        move_camera_system,
    );
    // Remove old events at the end of the frame.
    dispatcher = dispatcher.add(
        |ev1: &mut Vec<GameEvent>,
         ev2: &mut Vec<SkillTriggerEvent<Skills>>,
         ev3: &mut Vec<InputEvent>,
         ev4: &mut Vec<char>| {
            ev1.clear();
            ev2.clear();
            ev3.clear();
            ev4.clear();
            Ok(())
        },
    );

    let dispatcher = dispatcher.build(&mut world);
    //let mut spritesheet = SpriteSheet::new("assets/tilemap/colored_tilemap_packed.png");
    let mut spritesheet = SpriteSheet::new("assets/tilemap/new.png");
    let img = vec![
        (0, 0, 56, 40),     // 0, elephant 1
        (0, 40, 56, 40),    // 1, elephant 2
        (0, 80, 56, 40),    // 2, elephant 3
        (0, 120, 40, 32),   // 3, fat man 1
        (0, 152, 40, 32),   // 4, fat man 2
        (0, 184, 40, 32),   // 5, fat man 3
        (0, 216, 32, 32),   // 6, sword man 1
        (0, 248, 32, 32),   // 7, sword man 2
        (0, 280, 32, 32),   // 8, sword man 3
        (0, 312, 32, 32),   // 9, archer 1
        (0, 344, 32, 32),   // 10, archer 2
        (0, 376, 32, 32),   // 11, archer 3
        (0, 408, 40, 32),   // 12, axe 1
        (0, 440, 40, 32),   // 13, axe 2
        (0, 472, 40, 32),   // 14, axe 3
        (0, 504, 32, 32),   // 15, lance 1
        (0, 536, 32, 32),   // 16, lance 2
        (0, 568, 32, 32),   // 17, lance 3
        (0, 600, 24, 24),   // 18, sword small man 1
        (0, 624, 24, 24),   // 19, sword small man 2
        (0, 648, 24, 24),   // 20, sword small man 3
        (0, 672, 32, 32),   // 21, beer
        (88, 864, 8, 8),    // 22, forest
        (0, 704, 96, 120),  // 23, core full
        (96, 746, 32, 32),  // 24, tower1
        (96, 768, 32, 32),  // 25, tower2, 78y
        (128, 768, 54, 56), // 26, barracks
        (127, 723, 33, 37), // 27, tree
        (0, 704, 32, 30), // 28, core 1
        (32, 704, 32, 30), // 29, core 2
        (66, 704, 32, 30), // 30, core 3
        (0, 734, 32, 30), // 31, core 4
        (32, 734, 32, 30), // 32, core 5
        (66, 734, 32, 30), // 33, core 6
        (0, 764, 32, 30), // 34, core 7
        (32, 764, 32, 30), // 35, core 8
        (66, 764, 32, 30), // 36, core 9
        (0, 794, 32, 30), // 37, core 10
        (32, 794, 32, 30), // 38, core 11
        (66, 794, 32, 30), // 39, core 12
    ];
    for v in img {
        spritesheet = spritesheet.add_sprite(Rect::with_size(v.0, 872 - v.2 - v.1, v.2, v.3));
    }
    /*for j in 0..10 {
        for i in 0..10 {
            spritesheet = spritesheet.add_sprite(Rect::with_size(i * 8, (9 - j) * 8, 8, 8));
        }
    }*/

    let mut context = BTermBuilder::new();
    {
        context = context.with_simple_console(SCREEN_WIDTH, SCREEN_HEIGHT, "terminal8x8.png");
    }
    #[cfg(feature = "opengl")]
    {
        context = context.with_sprite_sheet(spritesheet);
        context = context.with_sprite_console(SCREEN_WIDTH, SCREEN_HEIGHT, 0);
    }
    #[cfg(feature = "headless")]
    {
        println!("Running headlessly...");
    }

    let context = context
        .with_font("terminal8x8.png", 8, 8)
        .with_title("Shotcaller")
        .with_vsync(false)
        .with_advanced_input(true)
        .build()
        .expect("Failed to build BTerm context.");

    world.initialize::<Vec<char>>();
    world.initialize::<Components<MultiSprite>>();
    world.initialize::<Components<Sprite>>();
    world.initialize::<Components<Point>>();
    world.initialize::<Camera>();

    world.initialize::<Mouse>();
    world.initialize::<Components<Barrack>>();
    world.initialize::<Components<Core>>();
    world.initialize::<Components<RenderTarget>>();
    world.initialize::<Viewshed>();
    world.initialize::<TeamLeaders>();
    world.initialize::<GameMode>();
    world.initialize::<SelectedUnits>();
    world.initialize::<InputState>();
    world.initialize::<RNG>();
    world.initialize::<Vec<NetworkEvent>>();

    create_map_bg(&mut world);

    *world.get_mut::<Option<CollisionResource>>().unwrap() = Some(CollisionResource::new(
        CollisionMap::new(MAP_SIZE_X, MAP_SIZE_Y),
        Point::new(0, 0),
    ));

    let keymap: HashMap<u8, InputEvent> = load_yaml("assets/keymap.yaml");
    let keymap = keymap.into_iter().map(|(k, v)| (k as char, v)).collect();
    *world.get_mut::<HashMap<char, InputEvent>>().unwrap() = keymap;

    let skill_definitions = load_yaml("assets/skill_defs.yaml");
    *world
        .get_mut::<SkillDefinitions<Stats, Effectors, Skills, Items>>()
        .unwrap() = skill_definitions;

    let effector_defs = load_yaml("assets/effector_defs.yaml");
    *world
        .get_mut::<EffectorDefinitions<Stats, Effectors>>()
        .unwrap() = effector_defs;

    let item_defs = load_yaml("assets/item_defs.yaml");
    world.initialize::<ItemDefinitions<Items, (), ()>>();
    *world.get_mut::<ItemDefinitions<Items, (), ()>>().unwrap() = item_defs;

    let leader_defs = load_yaml("assets/leader_defs.yaml");
    world.initialize::<LeaderDefinitions>();
    *world.get_mut::<LeaderDefinitions>().unwrap() = leader_defs;

    let stat_defs: StatDefinitions<Stats> = load_yaml("assets/stat_defs.yaml");
    let default_stats = stat_defs.to_statset();
    *world.get_mut().unwrap() = stat_defs;

    for (i, s) in MAP_STRING.iter().enumerate() {
        for (j, c) in s.chars().enumerate() {
            if c != '#' && c != '0' {
                let team = if c.is_uppercase() {
                    Team::Me
                } else {
                    Team::Other
                };
                let position = Point::new(i as u32, j as u32);
            }
        }
    }

    let core_color_fg = &[
        RGBA::named(BLUE),
        RGBA::named(BLUE),
        RGBA::named(BLUE),
        RGBA::named(BLUE),
        RGBA::named(BLUE),
        RGBA::named(BLUE),
        RGBA::named(BLUE),
        RGBA::named(BLUE),
        RGBA::named(BLUE),
        RGBA::named(BLUE),
        RGBA::named(BLUE),
        RGBA::named(BLUE),
    ];

    let core_color_bg_other = &[
        RGBA::named(RED),
        RGBA::named(RED),
        RGBA::named(RED),
        RGBA::named(RED),
        RGBA::named(RED),
        RGBA::named(RED),
        RGBA::named(RED),
        RGBA::named(RED),
        RGBA::named(RED),
        RGBA::named(RED),
        RGBA::named(RED),
        RGBA::named(RED),
    ];

    let core_color_bg_me = &[
        RGBA::named(GREEN),
        RGBA::named(GREEN),
        RGBA::named(GREEN),
        RGBA::named(GREEN),
        RGBA::named(GREEN),
        RGBA::named(GREEN),
        RGBA::named(GREEN),
        RGBA::named(GREEN),
        RGBA::named(GREEN),
        RGBA::named(GREEN),
        RGBA::named(GREEN),
        RGBA::named(GREEN),
    ];

    // Create cores
    centity!(
        world,
        Point::new(MAP_SIZE_X as i32 / 2, 1),
        MultiSprite {
            ascii: "CCCCCCCCCCCC",
            width: 3,
            height: 4,
            fg: core_color_fg.clone(),
            bg: core_color_bg_other.clone(),
            sprite_indices: vec![28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39],
        },
        Team::Other,
        Core,
        default_stats.clone(),
    );

    centity!(
        world,
        Point::new(MAP_SIZE_X as i32 / 2, MAP_SIZE_Y as i32 - 2),
        MultiSprite {
            ascii: "CCCCCCCCCCCC",
            width: 3,
            height: 4,
            fg: core_color_fg.clone(),
            bg: core_color_bg_me.clone(),
            sprite_indices: vec![28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39],
        },
        Team::Me,
        Core,
        default_stats.clone(),
    );

    // Create barracks
    for i in -1..=1 {
        let x = MAP_SIZE_X as i32 / 2 + BARRACK_OFFSET * i as i32;
        let y = BARRACK_HEIGHT_FROM_EDGE;
        centity!(
            world,
            Point::new(x, y),
            Sprite {
                glyph: to_cp437('B'),
                fg: RGBA::named(YELLOW),
                bg: RGBA::named(RED),
            },
            SpriteIndex(TileMapping::Barrack.into()),
            Team::Other,
            Barrack,
            default_stats.clone(),
        );
        // Creep spawners
        centity!(
            world,
            Point::new(x, y + 1),
            CreepSpawner(0.0, CREEP_SPAWN_TICKS),
            //CreepSpawner(0, 2))
            Team::Other,
        );
    }

    for i in -1..=1 {
        let x = MAP_SIZE_X as i32 / 2 + BARRACK_OFFSET * i;
        let y = MAP_SIZE_Y as i32 - 1 - BARRACK_HEIGHT_FROM_EDGE;
        centity!(
            world,
            Point::new(x, y),
            Sprite {
                glyph: to_cp437('B'),
                fg: RGBA::named(YELLOW),
                bg: RGBA::named(GREEN),
            },
            SpriteIndex(TileMapping::Barrack.into()),
            Team::Me,
            Barrack,
            default_stats.clone(),
            LineOfSight::new(15),
        );
        // Creep spawners
        centity!(
            world,
            Point::new(x, y - 1),
            CreepSpawner(0.0, CREEP_SPAWN_TICKS),
            Team::Me,
            LineOfSight::new(15),
        );
    }

    // Create towers
    for i in -1..=1 {
        for j in 1..=2 {
            centity!(
                world,
                Point::new(
                    MAP_SIZE_X as i32 / 2 + TOWER_OFFSET * i,
                    MAP_SIZE_Y as i32 * j / 6,
                ),
                Sprite {
                    glyph: to_cp437('T'),
                    fg: RGBA::named(GREEN),
                    bg: RGBA::named(RED),
                },
                SpriteIndex(TileMapping::Tower1.into()),
                Team::Other,
                default_stats.clone(),
            );
            centity!(
                world,
                Point::new(
                    MAP_SIZE_X as i32 / 2 + TOWER_OFFSET * i,
                    MAP_SIZE_Y as i32 * j / 6 + 1,
                ),
                Sprite {
                    glyph: to_cp437('T'),
                    fg: RGBA::named(GREEN),
                    bg: RGBA::named(RED),
                },
                SpriteIndex(TileMapping::Tower2.into()),
            );
        }
    }

    for i in -1..=1 {
        for j in 1..=2 {
            centity!(
                world,
                Point::new(
                    MAP_SIZE_X as i32 / 2 + TOWER_OFFSET * i,
                    MAP_SIZE_Y as i32 - 1 - PLAY_HEIGHT as i32 * j / 6,
                ),
                Sprite {
                    glyph: to_cp437('T'),
                    fg: RGBA::named(GREEN),
                    bg: RGBA::named(GREEN),
                },
                SpriteIndex(TileMapping::Tower1.into()),
                Team::Me,
                default_stats.clone(),
                LineOfSight::new(6),
            );
            centity!(
                world,
                Point::new(
                    MAP_SIZE_X as i32 / 2 + TOWER_OFFSET * i,
                    MAP_SIZE_Y as i32 - PLAY_HEIGHT as i32 * j / 6,
                ),
                Sprite {
                    glyph: to_cp437('T'),
                    fg: RGBA::named(GREEN),
                    bg: RGBA::named(RED),
                },
                SpriteIndex(TileMapping::Tower2.into()),
            );
        }
    }

    let mut nakama = nakama::NakamaApi::new();
    nakama.connect();
    nakama.get_match();
    std::thread::sleep_ms(500);
    *world.get_mut::<_>().unwrap() = nakama::receive_events(&mut nakama);
    let mut sys = nakama::network_player_manager_system.system();
    sys.initialize(&mut world);
    sys.run(&mut world).unwrap();
    world.get_mut::<Vec<NetworkEvent>>().unwrap().clear();

    let team_leaders = if nakama.is_host(&world.get::<_>().unwrap()) {
        // generate and send leaders.
        let mut rng = world.get_mut::<RNG>().unwrap();
        let mut leaders_vec = vec![
            Leaders::Generic1,
            Leaders::Generic2,
            Leaders::TreePersonLeader,
            Leaders::Raja,
            Leaders::AxePersonLeader,
            Leaders::CentaurPersonLeader,
            Leaders::Celsus,
            Leaders::Erno,
            Leaders::SoulsCollector,
            Leaders::BristlebackPersonLeader,
        ];

        let mut team_leaders = TeamLeaders::new(vec![], vec![]);

        for i in 0..10 {
            let select = rng.rng.rand_range(0..leaders_vec.len() as u32) as usize;
            let leader = leaders_vec.swap_remove(select);
            if i < 5 {
                team_leaders.me.push(leader);
            } else {
                team_leaders.other.push(leader);
            }
        }
        println!("Sending leaders");
        nakama.send_event(NetworkEvent::Leaders(team_leaders.clone()));
        team_leaders
    } else {
        println!("Waiting for leaders.");
        // receive leaders.
        let mut leaders = None;
        while leaders.is_none() {
            for ev in nakama::receive_events(&mut nakama) {
                match ev {
                    NetworkEvent::Leaders(l) => leaders = Some(l),
                    _ => {}
                }
            }
        }
        println!("Received leaders!");
        let mut leaders = leaders.unwrap();
        std::mem::swap(&mut leaders.me, &mut leaders.other);
        leaders
    };

    *world.get_mut::<TeamLeaders>().unwrap() = team_leaders;

    {
        let mut evs = world.get_mut::<Vec<GameEvent>>();
        let evs = evs.as_mut().unwrap();
        for i in 0..5 {
            evs.push(GameEvent::SpawnLeader(
                Point::new(LEADER_SPAWN_POINT_ME.0, LEADER_SPAWN_POINT_ME.1),
                i,
                None,
            ));
            evs.push(GameEvent::SpawnLeader(
                Point::new(LEADER_SPAWN_POINT_OTHER.0, LEADER_SPAWN_POINT_OTHER.1),
                i + 5,
                None,
            ));
        }
    }

    let mut input_to_move_camera = HashMap::<_, _, RandomState>::default();
    input_to_move_camera.insert(
        InputEvent::CameraNorth,
        MoveCameraEvent {
            direction: Direction::North,
            distance: 1,
        },
    );
    input_to_move_camera.insert(
        InputEvent::CameraSouth,
        MoveCameraEvent {
            direction: Direction::South,
            distance: 1,
        },
    );
    input_to_move_camera.insert(
        InputEvent::CameraEast,
        MoveCameraEvent {
            direction: Direction::East,
            distance: 1,
        },
    );
    input_to_move_camera.insert(
        InputEvent::CameraWest,
        MoveCameraEvent {
            direction: Direction::West,
            distance: 1,
        },
    );
    *world.get_mut::<_>().unwrap() = input_to_move_camera;

    world.get_mut::<Camera>().unwrap().size.x = PLAY_WIDTH as i32;

    let host_network = DispatcherBuilder::new().build(&mut world);
    let client_network = DispatcherBuilder::new().build(&mut world);

    *world.get_mut::<NakamaApi>().unwrap() = nakama;

    let gd = GameData {
        world,
        client_dispatcher: dispatcher,
        host_network,
        client_network,
    };

    let post_update: PostUpdate = |_, _| {};
    let engine = Engine::new(DefaultState, gd, post_update, 60.0);

    let gs = State { engine };

    main_loop(context, gs)
}
