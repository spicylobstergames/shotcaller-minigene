use minigene::*;

const MAP: &[&str] = &[
"###################################00000000#####################################",
"###################################00000000#####################################",
"###################################00000000#####################################",
"######################################00########################################",
"######################################00########################################",
"######################################00########################################",
"######################################00########################################",
"######################################00########################################",
"######################################00########################################",
"######################################00########################################",
"##################00000000000000000000000000000000000000000#####################",
"##################00000000000000000000000000000000000000000#####################",
"##################00##################00#################00#####################",
"##################00##################00#################00#####################",
"##################00##################00#################00#####################",
"##################00##################00#################00#####################",
"##################00##################00#################00#####################",
"##################00##################00#################00#####################",
"##################00##################00#################00#####################",
"##################00##################00#################00#####################",
"##################00##################00#################00#####################",
"##################00##################00#################00#####################",
"##################00##################00#################00#####################",
"##################00##################00#################00#####################",
"##################00##################00#################00#####################",
"##################00##################00#################00#####################",
"##################00##################00#################00#####################",
"##################00##################00#################00#####################",
"##################00##################00#################00#####################",
"##################00##################00#################00#####################",
"##################00##################00#################00#####################",
"##################00##################00#################00#####################",
"##################00##################00#################00#####################",
"##################00##################00#################00#####################",
"##################00##################00#################00#####################",
"##################00##################00#################00#####################",
"##################00##################00#################00#####################",
"##################00##################00#################00#####################",
"##################00##################00#################00#####################",
"##################00000000000000000000000000000000000000000#####################",
"##################00000000000000000000000000000000000000000#####################",
"######################################00########################################",
"######################################00########################################",
"######################################00########################################",
"######################################00########################################",
"######################################00########################################",
"######################################00########################################",
"###################################00000000#####################################",
"###################################00000000#####################################",
"###################################00000000#####################################",
];

#[derive(Component)]
pub struct Tower;
#[derive(Component)]
pub struct Creep;
#[derive(Component)]
pub struct CreepSpawner(u32);
#[derive(Component)]
pub struct Base;
pub struct Spawner<F: Fn(&mut World)> {
    f: F,
}
#[derive(Component)]
pub struct Player;

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub enum Stats {
    Health,
    Defense,
    Attack,
    Mana,
}

// non portable
system!(UpdateCollisionResourceSystem, |global_map: WriteExpect<'a, CollisionResource>, positions: ReadStorage<'a, Point>, players: ReadStorage<'a, Player>| {
    for j in 0..50usize {
        MAP[j].char_indices().for_each(|(i, c)| {
            if c == '#' {
                global_map.map.set(i as u32, j as u32);
            } else {
                global_map.map.unset(i as u32, j as u32);
            }
        });
    }
    for (pos, _) in (&positions, &players).join() {
        global_map.position.x = pos.x - 40;
        global_map.position.y = pos.y - 25;
    }
});

system!(CreepSpawnerSystem, |entities: Entities<'a>, positions: WriteStorage<'a, Point>, spawners: WriteStorage<'a, CreepSpawner>, creeps: WriteStorage<'a, Creep>,
        ai_destinations: WriteStorage<'a, AiDestination>, ai_paths: WriteStorage<'a, AiPath>, sprites: WriteStorage<'a, Sprite>| {
    let mut v = vec![];
    for (pos, mut spawner) in (&positions, &mut spawners).join() {
        if spawner.0 == 0 {
            spawner.0 = 20;
            // spawn
            v.push(pos.clone());
        }
        spawner.0 -= 1;
    }
    v.into_iter().for_each(|pos| {
        let creep = entities.create();
        positions.insert(creep, pos.clone()).unwrap();
        creeps.insert(creep, Creep).unwrap();
        ai_paths.insert(creep, AiPath::new(NavigationPath::new())).unwrap();
        ai_destinations.insert(creep, AiDestination::new(Point::new(39, 25))).unwrap();
        sprites.insert(creep, Sprite {
                glyph: to_cp437('c'),
                fg: RGBA::named(YELLOW),
                bg: RGBA::named(BLACK),
            }).unwrap();
    });
});

fn render<'a>(ctx: &mut BTerm, camera: &Camera, positions: ReadStorage<'a, Point>, multi_sprites: ReadStorage<'a, MultiSprite>, sprites: ReadStorage<'a, Sprite>) {
    ctx.cls();
    let mut i = 0;
    for s in MAP {
        ctx.print(0, i, s);
        i = i + 1;
    }
}

struct State {
    pub world: World,
    pub dispatcher: Dispatcher<'static, 'static>,
}
impl GameState for State {
    fn tick(&mut self, ctx : &mut BTerm) {
        // Input
        let mut input = INPUT.lock();
        for key in input.key_pressed_set().iter() {
            println!("KEYPRESS: {:?}", key);
        }
        //self.world.insert(ctx.key.clone());
        self.dispatcher.dispatch(&mut self.world);
        render(ctx, &self.world.read_resource(), self.world.read_storage(), self.world.read_storage(), self.world.read_storage());
        render_sprites(ctx, &self.world.read_resource(), self.world.read_storage(), self.world.read_storage(), self.world.read_storage());
        self.world.maintain();
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Shotcaller")
        .with_vsync(false)
        .with_advanced_input(true)
        .build()?;
    let mut world = World::new();
    world.register::<MultiSprite>();
    world.register::<Sprite>();
    world.register::<Comp<StatSet<Stats>>>();
    let mut dispatcher = DispatcherBuilder::new()
        .with(CombineCollisionSystem, "combine_collision", &[])
        .with(UpdateCollisionResourceSystem, "update_collision_res", &["combine_collision"])
        .with(CreepSpawnerSystem, "creep_spawner", &[])
        .with(AiPathingSystem, "ai_pathing", &["update_collision_res"])
        .with(AiMovementSystem, "ai_movement", &["ai_pathing"])
        .build();
    dispatcher.setup(&mut world);

    world.insert(Camera::new(Point::new(0,0), Point::new(80, 50)));
    let stat_defs = StatDefinitions::from(vec![
        StatDefinition::new(Stats::Health, String::from("health"), String::from("HP"), 100.0),
        StatDefinition::new(Stats::Defense, String::from("defense"), String::from("Defense"), 0.0),
        StatDefinition::new(Stats::Attack, String::from("attack"), String::from("Attack"), 10.0),
        StatDefinition::new(Stats::Mana, String::from("mana"), String::from("MP"), 100.0),
    ]);

    // player
    world.create_entity()
        .with(Point::new(0, 0))
        .with(MultiSprite::new(MultiTileSprite::from_string("@@", 1, 2)))
        .with(Comp(stat_defs.to_statset()))
        //.with(Player)
        .build();

    world.insert(stat_defs);
    world.insert(CollisionResource::new(CollisionMap::new(80, 50), Point::new(0, 0)));

    // single tile test
    world.create_entity()
        .with(Point::new(5, 5))
        .with(Sprite {
            glyph: to_cp437('x'),
            fg: RGBA::named(YELLOW),
            bg: RGBA::named(BLACK),
        })
        .build();
    // creep spawner
    world.create_entity()
        .with(Point::new(55, 10))
        .with(CreepSpawner(0))
        .build();
    world.create_entity()
        .with(Point::new(25, 10))
        .with(CreepSpawner(0))
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

    let gs = State {
        world,
        dispatcher,
    };

    main_loop(context, gs)
}

