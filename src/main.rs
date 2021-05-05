//! The shotcaller game. A new MOBA!
#![warn(missing_docs)]

#[macro_use]
extern crate serde;

#[macro_use]
extern crate lazy_static;

use minigene::*;
use rand::{seq::SliceRandom, thread_rng, Rng};
use std::collections::hash_map::RandomState;
use std::collections::HashMap;

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
const CREEP_SPAWN_TICKS: u32 = 125;
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
const LEADER_SPAWN_POINT_ME: (i32, i32) = (PLAY_WIDTH as i32 / 2, PLAY_HEIGHT as i32 - 22);
const LEADER_SPAWN_POINT_OTHER: (i32, i32) = (PLAY_WIDTH as i32 / 2, 22);

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
mod render_map;
mod resources;
mod states;
mod systems;
mod utils;
pub use self::components::*;
pub use self::events::*;
pub use self::ids::*;
pub use self::leaders::*;
pub use self::render_map::*;
pub use self::resources::*;
pub use self::states::*;
pub use self::systems::*;
pub use self::utils::*;

// Bridge between bracket-lib and minigene
struct State {
    pub world: World,
    pub dispatcher: Dispatcher,
    pub state_machine: StateMachine,
    #[cfg(not(feature = "wasm"))]
    pub loop_helper: LoopHelper,
}
impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        #[cfg(not(feature = "wasm"))]
        let delta = self.loop_helper.loop_start();
        #[cfg(feature = "wasm")]
        let delta = std::time::Duration::from_secs_f32(1.0 / 20.0);
        self.world.get_mut::<Time>().unwrap().advance_frame(delta);
        mini_frame(
            &mut self.world,
            &mut self.dispatcher,
            ctx,
            &mut self.state_machine,
        );
        #[cfg(not(feature = "wasm"))]
        self.loop_helper.loop_sleep();
        if !self.state_machine.is_running() {
            ctx.quitting = true;
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
        update_collision_resource_system,
        handle_action_points_system,
        creep_spawner_system,
        simple_destination_system,
        ai_pathing_system,
        movement_system,
        toggle_game_speed_system,
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
    let mut spritesheet = SpriteSheet::new("assets/tilemap/colored_tilemap_packed.png");
    for j in 0..10 {
        for i in 0..10 {
            spritesheet = spritesheet.add_sprite(Rect::with_size(i * 8, (9 - j) * 8, 8, 8));
        }
    }
    let (mut world, mut dispatcher, mut context) = mini_init(
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        "Shotcaller",
        Some(spritesheet),
        dispatcher,
        world,
    );

    world.initialize::<Mouse>();
    world.initialize::<Components<Barrack>>();
    world.initialize::<Components<Core>>();
    world.initialize::<Viewshed>();
    world.initialize::<TeamLeaders>();

    *world.get_mut::<Option<CollisionResource>>().unwrap() = Some(CollisionResource::new(
        CollisionMap::new(MAP_SIZE_X, MAP_SIZE_Y),
        Point::new(0, 0),
    ));

    let mut state_machine = StateMachine::new(DefaultState);
    state_machine.start(&mut world, &mut dispatcher, &mut context);
    #[cfg(not(feature = "wasm"))]
    let loop_helper = LoopHelper::builder().build_with_target_rate(TARGET_FPS);

    /*register!(world, MultiSprite, Sprite, Team, Barrack, Tower, Core, Leader,
    Name, SpriteIndex, StatSet<Stats>, EffectorSet<Effectors>,
    SkillSet<Skills>, Inventory<Items, (), ()>, Point, SimpleMovement,
    AiPath, AiDestination, Creep, Player, CollisionMap, CreepSpawner, Collision,
    ProximityAttack, TowerProjectile, GotoStraight, GotoEntity,);*/

    // TODO reenable
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
    // Create cores
    centity!(
        world,
        Point::new(PLAY_WIDTH as i32 / 2, 1),
        Sprite {
            glyph: to_cp437('C'),
            fg: RGBA::named(BLUE),
            bg: RGBA::named(RED),
        },
        SpriteIndex(66),
        Team::Other,
        Core,
        default_stats.clone(),
    );

    centity!(
        world,
        Point::new(PLAY_WIDTH as i32 / 2, PLAY_HEIGHT as i32 - 2),
        Sprite {
            glyph: to_cp437('C'),
            fg: RGBA::named(BLUE),
            bg: RGBA::named(GREEN),
        },
        SpriteIndex(66),
        Team::Me,
        Core,
        default_stats.clone(),
    );

    // Create barracks
    for i in -1..=1 {
        let x = PLAY_WIDTH as i32 / 2 + BARRACK_OFFSET * i as i32;
        let y = BARRACK_HEIGHT_FROM_EDGE;
        centity!(
            world,
            Point::new(x, y),
            Sprite {
                glyph: to_cp437('B'),
                fg: RGBA::named(YELLOW),
                bg: RGBA::named(RED),
            },
            SpriteIndex(69),
            Team::Other,
            Barrack,
            default_stats.clone(),
        );
        // Creep spawners
        centity!(
            world,
            Point::new(x, y + 1),
            CreepSpawner(0, CREEP_SPAWN_TICKS),
            //CreepSpawner(0, 2))
            Team::Other,
        );
    }

    for i in -1..=1 {
        let x = PLAY_WIDTH as i32 / 2 + BARRACK_OFFSET * i;
        let y = PLAY_HEIGHT as i32 - 1 - BARRACK_HEIGHT_FROM_EDGE;
        centity!(
            world,
            Point::new(x, y),
            Sprite {
                glyph: to_cp437('B'),
                fg: RGBA::named(YELLOW),
                bg: RGBA::named(GREEN),
            },
            SpriteIndex(69),
            Team::Me,
            Barrack,
            default_stats.clone(),
            LineOfSight::new(15),
        );
        // Creep spawners
        centity!(
            world,
            Point::new(x, y - 1),
            CreepSpawner(0, CREEP_SPAWN_TICKS),
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
                    PLAY_WIDTH as i32 / 2 + TOWER_OFFSET * i,
                    PLAY_HEIGHT as i32 * j / 6,
                ),
                Sprite {
                    glyph: to_cp437('T'),
                    fg: RGBA::named(GREEN),
                    bg: RGBA::named(RED),
                },
                SpriteIndex(80),
                Team::Other,
                default_stats.clone(),
            );
        }
    }

    for i in -1..=1 {
        for j in 1..=2 {
            centity!(
                world,
                Point::new(
                    PLAY_WIDTH as i32 / 2 + TOWER_OFFSET * i,
                    PLAY_HEIGHT as i32 - 1 - PLAY_HEIGHT as i32 * j / 6,
                ),
                Sprite {
                    glyph: to_cp437('T'),
                    fg: RGBA::named(GREEN),
                    bg: RGBA::named(GREEN),
                },
                SpriteIndex(80),
                Team::Me,
                default_stats.clone(),
                LineOfSight::new(6),
            );
        }
    }

    // Spawn leaders
    let mut rng = thread_rng();
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
    let mut me_number = 0;
    let mut other_number = 0;

    leaders_vec.shuffle(&mut rng);

    for leader in leaders_vec {
        if rng.gen_range(1, 3) == 1 {
            if me_number < 5 {
                team_leaders.me.push(leader);
                me_number += 1;
            } else if other_number < 5 {
                team_leaders.other.push(leader);
                other_number += 1;
            }
        } else {
            if other_number < 5 {
                team_leaders.other.push(leader);
                other_number += 1;
            } else if me_number < 5 {
                team_leaders.me.push(leader);
                me_number += 1;
            }
        }
    }

    *world.get_mut::<TeamLeaders>().unwrap() = team_leaders;

    {
        let mut evs = world.get_mut::<Vec<GameEvent>>();
        let evs = evs.as_mut().unwrap();
        for i in 0..5 {
            evs.push(GameEvent::SpawnLeader(
                Point::new(LEADER_SPAWN_POINT_ME.0, LEADER_SPAWN_POINT_ME.1),
                i,
            ));
            evs.push(GameEvent::SpawnLeader(
                Point::new(LEADER_SPAWN_POINT_OTHER.0, LEADER_SPAWN_POINT_OTHER.1),
                i + 5,
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

    create_map_bg(&mut world);

    let gs = State {
        world,
        dispatcher,
        state_machine,
        #[cfg(not(feature = "wasm"))]
        loop_helper,
    };

    main_loop(context, gs)
}
