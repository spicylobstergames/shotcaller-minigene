#[macro_use]
extern crate specs_declaration;
#[macro_use]
extern crate specs_derive;
#[macro_use]
extern crate derive_new;

use bracket_lib::prelude::*;
use specs::prelude::*;
use hibitset::BitSet;
use game_features::*;

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

/// Component wrapper for types not implementing Component
#[derive(new)]
pub struct Comp<T>(T);
impl<T: Send+Sync+'static> Component for Comp<T> {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Component)]
pub struct Tower;
#[derive(Component)]
pub struct Creep;
#[derive(Component)]
pub struct CreepSpawner(u32);
#[derive(Component)]
pub struct Base;
#[derive(Component)]
pub struct Sprite {
    pub glyph: u16,
    pub fg: RGBA,
    pub bg: RGBA,
}
#[derive(Component, new)]
pub struct MultiSprite {
    pub tile: MultiTileSprite,
}
#[derive(Component, new)]
pub struct AiPath {
    pub path: NavigationPath,
}

#[derive(Component, new)]
pub struct AiDestination {
    pub target: Point,
}
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

pub struct GameSpeed(f32);

impl Default for GameSpeed {
    fn default() -> Self {
        GameSpeed(1.0)
    }
}

// Collision stuff
// Coords starts at the upper right corner
/// Collision of a single tile entity
#[derive(Component)]
pub struct Collision;
/// Collision of a multi tile entity. Not necessarily colliding everywhere.
/// Can be both used as a global resource and as a component for individual entities.
#[derive(Component)]
pub struct CollisionMap {
    bitset: BitSet,
    width: u32,
    height: u32,
}

impl CollisionMap {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            bitset: BitSet::with_capacity(width * height),
            width,
            height,
        }
    }

    pub fn set(&mut self, x: u32, y: u32) {
        self.bitset.add(self.index_of(x, y));
    }

    pub fn unset(&mut self, x: u32, y: u32) {
        self.bitset.remove(self.index_of(x, y));
    }

    pub fn is_set(&mut self, x: u32, y: u32) -> bool {
        self.bitset.contains(self.index_of(x, y))
   }

    pub fn size(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    pub fn clear(&mut self) {
        self.bitset.clear();
    }

    pub fn index_of(&self, x: u32, y: u32) -> u32 {
        y * self.width + x
    }

    pub fn position_of(&self, idx: u32) -> (u32, u32) {
        (idx % self.width, idx / self.width)
    }
}

#[cfg(test)]
mod tests {
    use crate::CollisionMap;
    #[test]
    fn collmap() {
        let mut m = CollisionMap::new(3, 3);
        m.set(2, 2);
        assert!(m.is_set(2, 2));
        assert_eq!(m.index_of(2,2), 8);
        assert_eq!(m.position_of(8), (2, 2));
    }
}

impl BaseMap for CollisionMap {
    fn is_opaque(&self, idx: usize) -> bool {
        self.bitset.contains(idx as u32)
    }

    fn get_available_exits(&self, idx: usize) -> SmallVec<[(usize, f32); 10]> {
        let mut o = smallvec![];
        //println!("idx: {}", idx);
        // right
        if (idx % self.width as usize) < (self.width as usize - 1) {
            let n = idx + 1;
            if !self.is_opaque(n) {
                //println!("ADDING AT {},{}, while it is {} opaque.", self.position_of(idx as u32).0, self.position_of(idx as u32).1, self.is_opaque(idx));
                o.push((n, 1.0));
            }
        }
        // left
        if (idx % self.width as usize) > 0 {
            let n = idx - 1;
            if !self.is_opaque(n) {
                o.push((n, 1.0));
            }
        }
        // down
        if (idx / self.width as usize) < (self.height as usize - 1) {
            let n = idx + self.width as usize;
            if !self.is_opaque(n) {
                o.push((n, 1.0));
            }
        }
        // up
        if idx >= (self.width as usize) {
            let n = idx - self.width as usize;
            if !self.is_opaque(n) {
                o.push((n, 1.0));
            }
        }
        o
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        let (x1, y1) = self.position_of(idx1 as u32);
        let (x2, y2) = self.position_of(idx2 as u32);
        ((x2 as f32 - x1 as f32).powf(2.0) + (y2 as f32 - y1 as f32).powf(2.0)).sqrt()
    }
}

#[derive(new)]
pub struct CollisionResource {
    pub map: CollisionMap,
    pub position: Point,
}

impl Default for CollisionResource {
    fn default() -> Self {
        Self {
            map: CollisionMap::new(80, 50),
            position: Point::new(0, 0),
        }
    }
}

#[derive(new)]
pub struct Camera {
    pub position: Point,
    pub size: Point,
}

pub fn position_inside_rect(pos_x: i32, pos_y: i32, rect_x: i32, rect_y: i32, size_x: u32, size_y: u32) -> bool {
    pos_x >= rect_x &&
    pos_y >= rect_y &&
    pos_x < rect_x + size_x as i32 &&
    pos_y < rect_y + size_y as i32
}

system!(CombineCollisionSystem, |positions: ReadStorage<'a, Point>, collisions: ReadStorage<'a, Collision>, maps: ReadStorage<'a, CollisionMap>, global_map: Write<'a, CollisionResource>| {
    global_map.map.clear();

    for (pos, _) in (&positions, &collisions).join() {
        let (x, y) = (pos.x, pos.y);
        if position_inside_rect(x, y, global_map.position.x, global_map.position.y, global_map.map.size().0, global_map.map.size().1) {
            let (t_x, t_y) = (global_map.position.x, global_map.position.y);
            global_map.map.set((x - t_x) as u32, (y - t_y) as u32);
        }
    }

    for (pos, coll) in (&positions, &maps).join() {
        for i in 0..coll.size().0 as i32{
            for j in 0..coll.size().1 as i32 {
                let (x, y) = (pos.x + i, pos.y + j);
                if position_inside_rect(x, y, global_map.position.x, global_map.position.y, global_map.map.size().0, global_map.map.size().1) {
                    let (t_x, t_y) = (global_map.position.x, global_map.position.y);
                    global_map.map.set((x - t_x) as u32, (y - t_y) as u32);
                }
            }
        }
    }
});

// non portable
system!(UpdateCollisionResourceSystem, |global_map: Write<'a, CollisionResource>, positions: ReadStorage<'a, Point>, players: ReadStorage<'a, Player>| {
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

system!(AiPathingSystem, |dests: ReadStorage<'a, AiDestination>, global_map: Read<'a, CollisionResource>, positions: ReadStorage<'a, Point>, paths: WriteStorage<'a, AiPath>| {
    for (pos, dest, path) in (&positions, &dests, &mut paths).join() {
        if pos.x == dest.target.x && pos.y == dest.target.y {
            continue;
        }
        // TODO Safety check for < 0 or out of map bounds
        let d = global_map.map.index_of((pos.x - global_map.position.x) as u32, (pos.y - global_map.position.y) as u32);
        let t = global_map.map.index_of((dest.target.x - global_map.position.x) as u32, (dest.target.y - global_map.position.y) as u32);
        let p = a_star_search(d, t, &global_map.map);
        path.path = p;
    }
});

system!(AiMovementSystem, |positions: WriteStorage<'a, Point>, paths: WriteStorage<'a, AiPath>, global_map: Read<'a, CollisionResource>| {
    // doesn't handle two entities that want to go to the same tile.
    for (pos, path) in (&mut positions, &mut paths).join() {
        if path.path.success && path.path.steps.len() > 1 {
            let dest = path.path.steps.remove(1);
            let (x, y) = global_map.map.position_of(dest as u32);
            *pos = Point::new(x as i32 + global_map.position.x, y as i32 + global_map.position.y);
        }
    }
});

fn render<'a>(ctx: &mut BTerm, camera: &Camera, positions: ReadStorage<'a, Point>, multi_sprites: ReadStorage<'a, MultiSprite>, sprites: ReadStorage<'a, Sprite>) {
    ctx.cls();
    let mut i = 0;
    for s in MAP {
        ctx.print(0, i, s);
        i = i + 1;
    }
    for (pos, sprite) in (&positions, &multi_sprites).join() {
        sprite.tile.render(ctx, Point::new(pos.x - camera.position.x, pos.y - camera.position.y));
    }
    for (pos, sprite) in (&positions, &sprites).join() {
        ctx.set(pos.x - camera.position.x, pos.y - camera.position.y, sprite.fg, sprite.bg, sprite.glyph);
    }
}

struct State {
    pub world: World,
    pub dispatcher: Dispatcher<'static, 'static>,
}
impl GameState for State {
    fn tick(&mut self, ctx : &mut BTerm) {
        self.world.insert(ctx.key.clone());
        self.dispatcher.dispatch(&mut self.world);
        render(ctx, &self.world.read_resource(), self.world.read_storage(), self.world.read_storage(), self.world.read_storage());
        self.world.maintain();
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Shotcaller")
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

    world.insert(CollisionResource::default());
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

