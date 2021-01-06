use crate::*;

/// Spawns a creep using the provided event.
pub fn spawn_leader_system(
    game_events: &Vec<GameEvent>,
    stat_def: &StatDefinitions<Stats>,
    team_leaders: &TeamLeaders,
    leader_defs: &LeaderDefinitions,
    entities: &mut Entities,
    positions: &mut Components<Point>,
    leaders: &mut Components<Leader>,
    //simple_movements: &mut Components<Hero1SimpleMovement>,
    //proximity_attacks: &mut Components<Hero1ProximityAttack>,
    simple_movements: &mut Components<SimpleMovement>,
    proximity_attacks: &mut Components<ProximityAttack>,
    stats: &mut Components<StatSet<Stats>>,
    teams: &mut Components<Team>,
    sprites: &mut Components<Sprite>,
    sprite_indices: &mut Components<SpriteIndex>,
    skillsets: &mut Components<SkillSet<Skills>>,
    effectors: &mut Components<EffectorSet<Effectors>>,
) -> SystemResult {
    for ev in game_events.iter() {
        if let GameEvent::SpawnLeader(pos, id) = ev {
            let leader = entities.create();
            positions.insert(leader, pos.clone());
            leaders.insert(leader, Leader(*id));
            let team = if *id < 5 {Team::Me} else {Team::Other};
            teams.insert(leader, team);
            stats.insert(leader, stat_def.to_statset());
            //simple_movements.insert(leader, Hero1SimpleMovement);
            simple_movements.insert(leader, SimpleMovement);
            //proximity_attacks.insert(leader, Hero1ProximityAttack::new(CREEP_ATTACK_RADIUS));
            proximity_attacks.insert(leader, ProximityAttack::new(CREEP_ATTACK_RADIUS));
            let bg = if team == Team::Me {
                RGBA::named(GREEN)
            } else {
                RGBA::named(WHITE)
            };
            sprites.insert(
                leader,
                Sprite {
                    glyph: to_cp437('L'),
                    fg: RGBA::named(RED),
                    bg,
                },
            );
            sprite_indices.insert(leader, SpriteIndex(6));
            let leader_id = if *id < 5 {
                team_leaders.me.get(*id as usize).expect("There isn't enough leaders in the me team!")
            } else {
                team_leaders.other.get((*id-5) as usize).expect("Leader ID is higher than 9, or there isn't enough leaders in the other team!")
            };
            skillsets.insert(leader, leader_defs.defs.get(leader_id).unwrap().skills.clone().into());
            effectors.insert(leader, EffectorSet::<Effectors>::default());
        }
    }
    Ok(())
}

