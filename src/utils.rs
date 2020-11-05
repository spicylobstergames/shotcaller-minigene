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
