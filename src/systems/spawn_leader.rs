use crate::*;

/// Spawns a leader using the provided event.
pub fn spawn_leader_system(
    game_events: &Vec<GameEvent>,
    stat_def: &StatDefinitions<Stats>,
    team_leaders: &TeamLeaders,
    leader_defs: &LeaderDefinitions,
    entities: &mut Entities,
    positions: &mut Components<Point>,
    leaders: &mut Components<Leader>,
    // retreats: &mut Components<FleeToBase>,
    // is_caught: &mut Components<IsCaught>,
    spell_steals: &mut Components<SpellSteal>,
    simple_movements: &mut Components<SimpleMovement>,
    proximity_attacks: &mut Components<ProximityAttack>,
    order_queue: &mut Components<OrderQueue>,
    // leader1_simple_movements: &mut Components<Leader1SimpleMovement>,
    // leader2_simple_movements: &mut Components<Leader2SimpleMovement>,
    // leader1_proximity_attacks: &mut Components<Leader1ProximityAttack>,
    stats: &mut Components<StatSet<Stats>>,
    teams: &mut Components<Team>,
    sprites: &mut Components<Sprite>,
    sprite_indices: &mut Components<SpriteIndex>,
    skillsets: &mut Components<SkillSet<Skills>>,
    effectors: &mut Components<EffectorSet<Effectors>>,
    uuids: &mut Components<Uuid>,
) -> SystemResult {
    for ev in game_events.iter() {
        if let GameEvent::SpawnLeader(pos, id, uuid_opt) = ev {
            let leader = entities.create();
            positions.insert(leader, pos.clone());
            leaders.insert(leader, Leader(*id));
            let team = if *id < 5 { Team::Me } else { Team::Other };
            teams.insert(leader, team);
            stats.insert(leader, stat_def.to_statset());
            stats
                .get_mut(leader)
                .unwrap()
                .stats
                .get_mut(&Stats::ActionPointRefillRate)
                .unwrap()
                .value = 25.0;
            let bg = if team == Team::Me {
                RGBA::named(GREEN)
            } else {
                RGBA::named(WHITE)
            };

            let leader_id = if *id < 5 {
                team_leaders
                    .me
                    .get(*id as usize)
                    .expect("There isn't enough leaders in the me team!")
            } else {
                team_leaders.other.get((*id - 5) as usize).expect(
                    "Leader ID is higher than 9, or there isn't enough leaders in the other team!",
                )
            };

            // Spawn with Hold position order. To stop leaders when game mode is changed to micro-input.
            // order_queue.insert(leader, OrderQueue::new(vec![UnitOrder::HoldPosition]));
            order_queue.insert(leader, OrderQueue::new());

            skillsets.insert(
                leader,
                leader_defs
                    .defs
                    .get(leader_id)
                    .unwrap()
                    .skills
                    .clone()
                    .into(),
            );

            effectors.insert(leader, EffectorSet::<Effectors>::default());

            // let mut inv =  Inventory::new_fixed(4);
            // inv.insert(ItemInstance::new(Items::Coffee, 1));
            // inventories.insert(leader, inv);

            match leader_id {
                Leaders::Generic1 => {
                    sprites.insert(
                        leader,
                        Sprite {
                            glyph: to_cp437('1'),
                            fg: RGBA::named(RED),
                            bg,
                        },
                    );
                    sprite_indices.insert(leader, SpriteIndex(TileMapping::FatMan1.into()));
                    simple_movements.insert(leader, SimpleMovement);
                    proximity_attacks
                        .insert(leader, ProximityAttack::new(MELEE_LEADER_ATTACK_RADIUS));
                    // TODO: Add higher threshold for retreating and re-enable
                    // leader1_simple_movements.insert(leader, Leader1SimpleMovement);
                    // leader1_proximity_attacks.insert(leader, Leader1ProximityAttacks::new(MELEE_LEADER_ATTACK_RADIUS));
                    // retreats.insert(leader, FleeToBase(0.0));
                    // is_caught.insert(leader, IsCaught(false));
                }
                Leaders::Generic2 => {
                    sprites.insert(
                        leader,
                        Sprite {
                            glyph: to_cp437('2'),
                            fg: RGBA::named(RED),
                            bg,
                        },
                    );
                    sprite_indices.insert(leader, SpriteIndex(TileMapping::FatMan2.into()));
                    simple_movements.insert(leader, SimpleMovement);
                    proximity_attacks
                        .insert(leader, ProximityAttack::new(RANGED_LEADER_ATTACK_RADIUS));
                    // TODO: Add higher threshold for retreating and re-enable
                    // leader2_simple_movements.insert(leader, Leader2SimpleMovement);
                    // leader1_proximity_attacks.insert(leader, Leader1ProximityAttack::new(RANGED_LEADER_ATTACK_RADIUS));
                    // retreats.insert(leader, FleeToBase(0.0));
                    // is_caught.insert(leader, IsCaught(false));
                }
                Leaders::TreePersonLeader => {
                    sprites.insert(
                        leader,
                        Sprite {
                            glyph: to_cp437('T'),
                            fg: RGBA::named(RED),
                            bg,
                        },
                    );
                    sprite_indices.insert(leader, SpriteIndex(TileMapping::Tree1.into()));
                    simple_movements.insert(leader, SimpleMovement);
                    proximity_attacks
                        .insert(leader, ProximityAttack::new(MELEE_LEADER_ATTACK_RADIUS));
                    // TODO: Add higher threshold for retreating and re-enable
                    // leader1_simple_movements.insert(leader, Leader1SimpleMovement);
                    // leader1_proximity_attacks.insert(leader, Leader1ProximityAttack::new(MELEE_LEADER_ATTACK_RADIUS));
                    // retreats.insert(leader, FleeToBase(0.0));
                    // is_caught.insert(leader, IsCaught(false));
                }
                Leaders::Raja => {
                    sprites.insert(
                        leader,
                        Sprite {
                            glyph: to_cp437('B'),
                            fg: RGBA::named(RED),
                            bg,
                        },
                    );
                    sprite_indices.insert(leader, SpriteIndex(TileMapping::SwordMan1.into()));
                    simple_movements.insert(leader, SimpleMovement);
                    proximity_attacks
                        .insert(leader, ProximityAttack::new(MELEE_LEADER_ATTACK_RADIUS));
                    // TODO: Add higher threshold for retreating and re-enable
                    // leader1_simple_movements.insert(leader, Leader1SimpleMovement);
                    // leader1_proximity_attacks.insert(leader, Leader1ProximityAttack::new(MELEE_LEADER_ATTACK_RADIUS));
                    // retreats.insert(leader, FleeToBase(0.0));
                    // is_caught.insert(leader, IsCaught(false));
                }
                Leaders::AxePersonLeader => {
                    sprites.insert(
                        leader,
                        Sprite {
                            glyph: to_cp437('A'),
                            fg: RGBA::named(RED),
                            bg,
                        },
                    );
                    sprite_indices.insert(leader, SpriteIndex(TileMapping::Axe1.into()));
                    simple_movements.insert(leader, SimpleMovement);
                    proximity_attacks
                        .insert(leader, ProximityAttack::new(MELEE_LEADER_ATTACK_RADIUS));
                    // TODO: Add higher threshold for retreating and re-enable
                    // leader1_simple_movements.insert(leader, Leader1SimpleMovement);
                    // leader1_proximity_attacks.insert(leader, Leader1ProximityAttack::new(MELEE_LEADER_ATTACK_RADIUS));
                    // retreats.insert(leader, FleeToBase(0.0));
                    // is_caught.insert(leader, IsCaught(false));
                }
                Leaders::CentaurPersonLeader => {
                    sprites.insert(
                        leader,
                        Sprite {
                            glyph: to_cp437('N'),
                            fg: RGBA::named(RED),
                            bg,
                        },
                    );
                    sprite_indices.insert(leader, SpriteIndex(TileMapping::Lance1.into()));
                    simple_movements.insert(leader, SimpleMovement);
                    proximity_attacks
                        .insert(leader, ProximityAttack::new(MELEE_LEADER_ATTACK_RADIUS));
                    // TODO: Add higher threshold for retreating and re-enable
                    // leader1_simple_movements.insert(leader, Leader1SimpleMovement);
                    // leader1_proximity_attacks.insert(leader, Leader1ProximityAttack::new(MELEE_LEADER_ATTACK_RADIUS));
                    // retreats.insert(leader, FleeToBase(0.0));
                    // is_caught.insert(leader, IsCaught(false));
                }
                Leaders::Celsus => {
                    sprites.insert(
                        leader,
                        Sprite {
                            glyph: to_cp437('C'),
                            fg: RGBA::named(RED),
                            bg,
                        },
                    );
                    sprite_indices.insert(leader, SpriteIndex(TileMapping::Lance3.into()));
                    simple_movements.insert(leader, SimpleMovement);
                    proximity_attacks
                        .insert(leader, ProximityAttack::new(RANGED_LEADER_ATTACK_RADIUS));
                    // TODO: Add higher threshold for retreating and re-enable
                    // leader2_simple_movements.insert(leader, Leader2SimpleMovement);
                    // leader1_proximity_attacks.insert(leader, Leader1ProximityAttack::new(RANGED_LEADER_ATTACK_RADIUS));
                    // retreats.insert(leader, FleeToBase(0.0));
                    // is_caught.insert(leader, IsCaught(false));
                }
                Leaders::Erno => {
                    sprites.insert(
                        leader,
                        Sprite {
                            glyph: to_cp437('E'),
                            fg: RGBA::named(RED),
                            bg,
                        },
                    );
                    sprite_indices.insert(leader, SpriteIndex(TileMapping::Archer1.into()));
                    simple_movements.insert(leader, SimpleMovement);
                    proximity_attacks
                        .insert(leader, ProximityAttack::new(RANGED_LEADER_ATTACK_RADIUS));
                    // TODO: Add higher threshold for retreating and re-enable
                    // leader2_simple_movements.insert(leader, Leader2SimpleMovement);
                    // leader1_proximity_attacks.insert(leader, Leader1ProximityAttack::new(RANGED_LEADER_ATTACK_RADIUS));
                    // retreats.insert(leader, FleeToBase(0.0));
                    // is_caught.insert(leader, IsCaught(false));
                    spell_steals.insert(leader, SpellSteal(false));
                }
                Leaders::SoulsCollector => {
                    sprites.insert(
                        leader,
                        Sprite {
                            glyph: to_cp437('S'),
                            fg: RGBA::named(RED),
                            bg,
                        },
                    );
                    sprite_indices.insert(leader, SpriteIndex(TileMapping::SwordSmallMan1.into()));
                    simple_movements.insert(leader, SimpleMovement);
                    proximity_attacks
                        .insert(leader, ProximityAttack::new(RANGED_LEADER_ATTACK_RADIUS));
                    // TODO: Add higher threshold for retreating and re-enable
                    // leader2_simple_movements.insert(leader, Leader2SimpleMovement);
                    // leader1_proximity_attacks.insert(leader, Leader1ProximityAttack::new(RANGED_LEADER_ATTACK_RADIUS));
                    // retreats.insert(leader, FleeToBase(0.0));
                    // is_caught.insert(leader, IsCaught(false));
                }
                Leaders::BristlebackPersonLeader => {
                    sprites.insert(
                        leader,
                        Sprite {
                            glyph: to_cp437('B'),
                            fg: RGBA::named(RED),
                            bg,
                        },
                    );
                    sprite_indices.insert(leader, SpriteIndex(TileMapping::SwordMan2.into()));
                    simple_movements.insert(leader, SimpleMovement);
                    proximity_attacks
                        .insert(leader, ProximityAttack::new(MELEE_LEADER_ATTACK_RADIUS));
                    // TODO: Add higher threshold for retreating and re-enable
                    // leader1_simple_movements.insert(leader, Leader1SimpleMovement);
                    // leader1_proximity_attacks.insert(leader, Leader1ProximityAttack::new(MELEE_LEADER_ATTACK_RADIUS));
                    // retreats.insert(leader, FleeToBase(0.0));
                    // is_caught.insert(leader, IsCaught(false));
                }
            }
            if let Some(uuid) = uuid_opt {
                uuids.insert(leader, uuid.clone());
            } else {
                uuids.insert(leader, Uuid::new_v4());
            }
        }
    }
    Ok(())
}
