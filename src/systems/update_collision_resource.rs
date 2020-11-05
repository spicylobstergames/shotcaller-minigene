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
