use crate::*;

/// Increment the attacks dealt stat by one.
pub fn increment_attacks_dealt(stat_set: &mut StatSet<Stats>) {
    stat_set.stats.get_mut(&Stats::AttacksDealt).unwrap().value += 1.0;
}

/// Increment the attacks received stat by one.
pub fn increment_attacks_received(stat_set: &mut StatSet<Stats>) {
    stat_set.stats.get_mut(&Stats::AttacksReceived).unwrap().value += 1.0;
}

/// Damage this `StatSet` by the provided damage amount.
pub fn damage(stat_set: &mut StatSet<Stats>, damage: f64) -> bool {
    let mut health_inst = stat_set.stats.get_mut(&Stats::Health).unwrap();
    health_inst.value -= damage;
    health_inst.value <= 0.0
}

/// Get all entities in a radius using filters.
pub fn entities_in_radius<F1: Fn(Entity, Point) -> bool, F2: Fn(Entity, Point, f32) -> bool>(
    around: &Point,
    entities: &Entities,
    positions: &Components<Point>,
    pre_filter: F1,
    post_filter: F2,
) -> Vec<(Entity, Point, f32)> {
    let mut vec = join!(&entities && &positions)
        .map(|(e, p)| (e.unwrap(), p.unwrap()))
        .filter(|(e, p)| pre_filter(*e, **p))
        .map(|(e, p)| (e, p.clone(), dist(around, p)))
        .filter(|(e, p, d)| post_filter(*e, *p, *d))
        .collect::<Vec<_>>();
    // Sort by distance
    vec.sort_by(|e1, e2| e1.2.partial_cmp(&e2.2).unwrap());
    vec
}

/// Gets the closest entity of the other team around this point where the entity
/// also has a `StatSet` component.
pub fn find_closest_in_other_team(
    my_team: &Team,
    my_pos: &Point,
    teams: &Components<Team>,
    positions: &Components<Point>,
    stats: &Components<StatSet<Stats>>,
    entities: &Entities,
) -> Option<(Entity, Point)> {
    let mut vec = join!(&entities && &teams && &positions && &stats)
        .filter(|(_, t, _, _)| *t.unwrap() != *my_team)
        .map(|(e, _, p, _)| (dist(my_pos, p.unwrap()), p.unwrap().clone(), e.unwrap()))
        //.filter(|(d, _, _)| *d < TOWER_RANGE)
        .collect::<Vec<_>>();
    vec.sort_by(|e1, e2| e1.0.partial_cmp(&e2.0).unwrap());
    vec.into_iter().next().map(|(_d, p, e)| (e, p))
}

/// Loads a yaml file into a struct.
#[cfg(not(feature = "wasm"))]
pub fn load_yaml<T: serde::de::DeserializeOwned>(filepath: &str) -> T {
    return serde_yaml::from_reader(
        std::fs::File::open(filepath).expect("Failed to load yaml file"),
    )
    .expect("Failed to parse yaml file into the requested type.");
}

/// Loads a yaml file into a struct.
#[cfg(feature = "wasm")]
pub fn load_yaml<T: serde::de::DeserializeOwned>(filepath: &str) -> T {
    let content_bytes = EMBED
        .lock()
        .get_resource(filepath.to_string())
        .expect("Yaml file isn't embedded into the binary.");
    let content = String::from_utf8(content_bytes.to_vec()).unwrap();
    return serde_yaml::from_str(&content)
        .expect("Failed to parse yaml file into the requested type.");
}

/// Creates an entity using a list of components.
#[macro_export]
macro_rules! centity {
    ($world:ident, $($comps:expr),*$(,)?) => {
        let e = $world.get_mut::<Entities>().unwrap().create();
        $($world.get_mut::<Components<_>>().unwrap().insert(e, $comps);)*
    }
}

/// Adds a file as an embedded resource for wasm compatibility.
#[allow(unused)]
#[macro_export]
macro_rules! add_embed {
    ($($path:literal),*) => {$(EMBED.lock().add_resource($path.to_string().replace("../", ""), include_bytes!($path));)*}
}
