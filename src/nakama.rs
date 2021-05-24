use crate::*;
use nanoserde::*;
use std::collections::BTreeSet;

pub fn get_client() -> ApiClient {
    let mut nakama = ApiClient::new(
        "xITSpxZegnWc",
        "shotcaller.us-east1.nakamacloud.io",
        443,
        "https",
    );
    while nakama.in_progress() {
        nakama.tick();
        println!("Creating nakama client connection.");
    }
    nakama
}
pub fn connect(nakama: &mut ApiClient) {
    nakama.register("emale2@emale.com", "henloust", "owomyfriend2");
    //nakama.authenticate("emale@emale.com", "owomyfriend");
    while !nakama.authenticated() {
        nakama.tick();
        println!("Logging in");
        check_error(nakama);
    }
}
pub fn check_error(nakama: &mut ApiClient) {
    if let Some(error) = nakama.error().as_deref() {
        panic!("Nakama error detected: {}", error);
    }
}
pub fn get_match(nakama: &mut ApiClient) {
    let mut matchmaker = Matchmaker::new();
    matchmaker
        .min(2)
        .max(2)
        .add_string_property("engine", "minigene_matchmaking")
        .add_query_item(
            &QueryItemBuilder::new("engine")
                .required()
                .term("minigene_matchmaking")
                .build(),
        );
    nakama.socket_add_matchmaker(&matchmaker);
    let mut token = nakama.matchmaker_token.clone();
    while token.is_none() {
        nakama.tick();
        println!("Waiting match token");
        check_error(nakama);
        token = nakama.matchmaker_token.clone();
    }
    nakama.socket_join_match_by_token(&token.unwrap());
    while nakama.match_id().is_none() {
        nakama.tick();
        println!("Joining match");
        check_error(nakama);
    }
    println!("Joined match");
}

#[derive(Clone, Debug, SerBin, DeBin)]
pub enum NetworkEvent {
    PlayerJoin { id: String, username: String },
    Leaders(TeamLeaders),
    PlayerLeave { id: String },
    TeleportEntity { id: String, point: u32 },
}

impl NetworkEvent {
    pub fn op_code(&self) -> i32 {
        match *self {
            NetworkEvent::TeleportEntity { id: _, point: _ } => 0,
            _ => -1,
        }
    }
}

pub fn receive_events(nakama: &mut ApiClient) -> Vec<NetworkEvent> {
    nakama.tick();
    let mut evs = vec![];
    while let Some(event) = nakama.try_recv() {
        match event {
            Event::Presence { joins, leaves } => {
                //leaver.session_id
                //join.session_id
                //join.username
                for join in joins {
                    evs.push(NetworkEvent::PlayerJoin {
                        id: join.session_id,
                        username: join.username,
                    });
                }
                for leave in leaves {
                    evs.push(NetworkEvent::PlayerLeave {
                        id: leave.session_id,
                    });
                }
            }
            Event::MatchData {
                user_id,
                opcode,
                data,
            } => {
                if let Ok(deser) = DeBin::deserialize_bin(&data) {
                    evs.push(deser);
                }
                /*match opcode {
                    0 => {
                        if let Ok(deser) = DeBin::deserialize_bin(&data) {
                            evs.push(deser);
                        }
                    },
                    _ => {},
                }*/
                //DeBin::deserialize_bin(&data).expect("Failed to deser received data");
            }
        }
    }
    evs
}

pub fn network_player_manager_system(evs: &Vec<NetworkEvent>, players: &mut BTreeSet<String>) -> SystemResult {
    for ev in evs {
        match ev {
            NetworkEvent::PlayerJoin{ id, username: _ } => {players.insert(id.clone());},
            NetworkEvent::PlayerLeave{ id } => {players.remove(id);},
            _ => {},
        }
    }
    Ok(())
}

pub fn send_event(nakama: &mut ApiClient, ev: NetworkEvent) {
    //nakama.socket_send(ev.op_code(), &ev);
    nakama.socket_send(-1, &ev);
    nakama.tick();
}

pub fn is_host(nakama: &ApiClient, remote_players: &BTreeSet<String>) -> bool {
    // no other players connected
    if remote_players.len() == 0 {
        return true;
    }

    *nakama.session_id.as_ref().unwrap() < *remote_players.iter().next().unwrap()
}
/*let mut nakama = ApiClient::new("defaultkey", "127.0.0.1", 7350, "http");
nakama.register("email", "password", "username");
nakama.authenticate("email", "password");
nakama.authenticated();
nakama.username().unwrap();

let mut matchmaker = Matchmaker::new();
matchmaker.min(2).max(2).add_string_property("engine", "minigene_matchmaking")
    .add_query_item(&QueryItemBuilder::new("engine").required().term("minigene_matchmaking").build());
nakama.socket_add_matchmaker(&matchmaker);

let token = nakama.matchmaker_token.clone();
if token.is_some() {
    nakama.socket_join_match_by_token(&token.unwrap());
}
if nakama.match_id().is_some() {
    // start game here
}
//nakama.socket_send(opcode, &data);
while let Some(event) = nakama.try_recv() {
    match event {
        Event::Presence {joins, leaves} => {
            //leaver.session_id
            //join.session_id
            //join.username
        },
        Event::MatchData {user_id, opcode, data} => {
            //DeBin::deserialize_bin(&data).expect("Failed to deser received data");
        }
    }
}
//nakama.tick();*/
