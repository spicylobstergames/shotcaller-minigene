//! The shotcaller game. A new MOBA!
#![warn(missing_docs)]

#[macro_use]
extern crate serde;

use minigene::*;
use std::collections::HashMap;

add_wasm_support!();

const PLAY_WIDTH: u32 = 81;
const PLAY_HEIGHT: u32 = 50;
const SCREEN_WIDTH: u32 = 100;
const SCREEN_HEIGHT: u32 = 50;
const CREEP_SPAWN_TICKS: u32 = 50;
const CREEP_ATTACK_RADIUS: f32 = 2.1;
//const LEADER_ATTACK_RADIUS: f32 = 2.1;
const AOE_RADIUS: f32 = 4.0;
const AOE_DAMAGE: f64 = 100.0;
const TOWER_RANGE: f32 = 5.0;
const TOWER_PROJECTILE_EXPLOSION_RADIUS: f32 = 2.1;
const TARGET_FPS: f32 = 20.0;
const ACTION_POINT_MOVE_COST: f64 = 100.0;
//const ACTION_POINT_ATTACK_COST: f64 = 50.0;

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

mod components;
mod events;
mod heroes;
mod ids;
mod render_map;
mod resources;
mod states;
mod systems;
mod utils;
pub use self::components::*;
pub use self::events::*;
pub use self::heroes::*;
pub use self::ids::*;
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
        if self.world.get::<QuitGame>().unwrap().0 {
            ctx.quitting = true;
        }
        if self.state_machine.is_running() && !ctx.quitting {
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
        "../assets/hero_defs.yaml"
    );
    let mut world = World::default();
    let mut dispatcher = DispatcherBuilder::new();
    dispatcher!(
        dispatcher,
        combine_collision_system,
        input_driver::<InputEvent>,
        update_collision_resource_system,
        handle_action_points_system,
        creep_spawner_system,
        simple_movement_system,
        ai_pathing_system,
        ai_movement_system,
        toggle_game_speed_system,
        win_condition_system,
        //hero1_simple_movement_system, // TODO re-enable
        tower_ai_system,
        proximity_attack_system,
        //hero1_proximity_attack_system, // TODO re-enable
        tower_projectile_system,
        update_enemies_around_system,
        skill_cooldown_system::<Skills>,
        trigger_passive_skill_system::<Stats, Effectors, Skills, Items, (), ()>,
        exec_skill_system::<Stats, Effectors, Skills, Items>,
        apply_effector_system::<Stats, Effectors>,
        remove_outdated_effector_system::<Effectors>,
        nature_summon_system,
        spawn_creep_system,
        aoe_damage_system,
        damage_entity_system,
        kill_entity_system,
        goto_straight_system,
        select_hero_system,
        hero_teleport_system,
        root_system,
        game_stats_updater_system,
        quit_game_system,
    );
    // Remove old events at the end of the frame.
    dispatcher = dispatcher.add(
        |ev1: &mut Vec<GameEvent>,
         ev2: &mut Vec<SkillTriggerEvent<Skills>>,
         ev3: &mut Vec<InputEvent>,
         ev4: &mut Vec<VirtualKeyCode>| {
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

    world.initialize::<Components<Barrack>>();
    world.initialize::<Components<Core>>();
    world.initialize::<TeamLeaders>();
    world.initialize::<Camera>();

    *world.get_mut::<Option<CollisionResource>>().unwrap() = Some(CollisionResource::new(
        CollisionMap::new(PLAY_WIDTH, PLAY_HEIGHT),
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

    let keymap = load_yaml("assets/keymap.yaml");
    *world
        .get_mut::<HashMap<VirtualKeyCode, InputEvent>>()
        .unwrap() = keymap;

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

    let hero_defs = load_yaml("assets/leader_defs.yaml");
    world.initialize::<LeaderDefinitions>();
    *world.get_mut::<LeaderDefinitions>().unwrap() = hero_defs;

    let stat_defs: StatDefinitions<Stats> = load_yaml("assets/stat_defs.yaml");
    let default_stats = stat_defs.to_statset();
    *world.get_mut().unwrap() = stat_defs;

    // Create cores
    /*world
    .create()
    .with(Point::new(PLAY_WIDTH as i32 / 2, 1))
    .with(Sprite {
        glyph: to_cp437('C'),
        fg: RGBA::named(BLUE),
        bg: RGBA::named(RED),
    })
    .with(SpriteIndex(66))
    .with(Team::Other)
    .with(Core)
    .with(Comp(default_stats.clone()))
    .build();*/

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
        let x = PLAY_WIDTH as i32 / 2 + PLAY_WIDTH as i32 / 7 * i as i32;
        let y = PLAY_HEIGHT as i32 / 8;
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
        let x = PLAY_WIDTH as i32 / 2 + PLAY_WIDTH as i32 / 7 * i;
        let y = PLAY_HEIGHT as i32 - 1 - PLAY_HEIGHT as i32 / 8;
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
        );
        // Creep spawners
        // centity!(
        //     world,
        //     Point::new(x, y - 1),
        //     CreepSpawner(0, CREEP_SPAWN_TICKS),
        //     Team::Me,
        // );
    }

    // Create towers
    for i in -1..=1 {
        for j in 1..=2 {
            centity!(
                world,
                Point::new(
                    PLAY_WIDTH as i32 / 2 + PLAY_WIDTH as i32 / 4 * i,
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
                    PLAY_WIDTH as i32 / 2 + PLAY_WIDTH as i32 / 4 * i,
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
            );
        }
    }

    // hero1 skill set
    let mut skillset = SkillSet::new(HashMap::new());
    skillset.skills.insert(
        Skills::DoubleDamage,
        SkillInstance::new(Skills::DoubleDamage, 0.0),
    );
    skillset
        .skills
        .insert(Skills::AOE, SkillInstance::new(Skills::AOE, 0.0));

    let _default_inventory = Inventory::<Items, (), ()>::new_fixed(4);

    let team_heroes = TeamLeaders::new(vec![Leaders::Generic1; 5], vec![Leaders::Generic2; 5]);
    *world.get_mut::<TeamLeaders>().unwrap() = team_heroes;

    // TODO re-enable de the hero
    // currently disabled to make the game balanced
    // Create generic hero 1
    // centity!(
    //     world,
    //     Point::new(PLAY_WIDTH as i32 / 2, PLAY_HEIGHT as i32 - 11),
    //     Sprite {
    //         glyph: to_cp437('L'),
    //         //fg: RGBA::named(YELLOW),
    //         fg: RGBA::named(RED),
    //         bg: RGBA::named(GREEN),
    //     },
    //     SpriteIndex(6),
    //     Team::Me,
    //     _default_inventory,
    //     Leader(1),
    //     default_stats,
    //     skillset,
    // );
    /*let hero1 = world
    .create()
    .with(Point::new(PLAY_WIDTH as i32 / 2, PLAY_HEIGHT as i32 - 11))
    .with(Sprite {
        glyph: to_cp437('L'),
        //fg: RGBA::named(YELLOW),
        fg: RGBA::named(RED),
        bg: RGBA::named(GREEN),
    })
    .with(SpriteIndex(6))
    .with(Team::Me)
    .with(Hero1SimpleMovement)
    .with(Comp(default_inventory.clone()))
    .with(Comp(skillset))
    .with(AiPath::new(NavigationPath::new()))
    .with(Leader(1))
    .with(Hero1ProximityAttack::new(LEADER_ATTACK_RADIUS))
    .with(Name("Generic Leader 1".to_string()))
    .with(Comp(default_stats.clone()))
    .with(Comp(EffectorSet::<Effectors>::default()))
    .with(FleeToBase(50.0))
    .with(IsCaught(false))
    .build();*/

    // Make hero HP really high. Used for testing win conditions.
    //world.write_storage::<Comp<StatSet<Stats>>>().get_mut(hero1).unwrap().0.stats.get_mut(&Stats::Health).unwrap().value = 10000000.0;

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
