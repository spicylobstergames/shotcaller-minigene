use crate::*;
use nanoserde::*;
use std::collections::BTreeSet;

pub struct NakamaApi {
    pub client: ApiClient,
}

unsafe impl Send for NakamaApi {}
unsafe impl Sync for NakamaApi {}

impl NakamaApi {
    pub fn new() -> Self {
        /*let mut client = ApiClient::new(
            "xITSpxZegnWc",
            "shotcaller.us-east1.selfcloud.io",
            443,
            "https",
        );*/
        let mut client = ApiClient::new(
            "defaultkey",
            "127.0.0.1",
            7350,
            "http",
        );
        client.tick();
        while client.in_progress() {
            client.tick();
            println!("Creating self client connection.");
        }
        Self { client }
    }
    pub fn connect(&mut self) {
        self.client.tick();
        self.client.register("emale3@emale.com", "henloust8", "owomyfriend5");
        //self.client.register("emale6@emale.com", "henloust9", "owomyfriend6");
        //self.client.register("emale@emale.com", "henloust", "owomyfriend");
        //self.client.authenticate("emale2@emale.com", "owomyfriend2");
        //self.authenticate("emale@emale.com", "owomyfriend");
        while !self.client.authenticated() {
            self.client.tick();
            println!("Logging in");
            self.check_error();
        }
    }
    pub fn check_error(&mut self) {
        if let Some(error) = self.client.error().as_deref() {
            panic!("Nakama error detected: {}", error);
        }
    }
    pub fn get_match(&mut self) {
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
        self.client.socket_add_matchmaker(&matchmaker);
        let mut token = self.client.matchmaker_token.clone();
        while token.is_none() {
            self.client.tick();
            println!("Waiting match token");
            self.check_error();
            token = self.client.matchmaker_token.clone();
        }
        self.client.socket_join_match_by_token(&token.unwrap());
        while self.client.match_id().is_none() {
            self.client.tick();
            println!("Joining match");
            self.check_error();
        }
        println!("Joined match");
    }

    pub fn send_event(&mut self, ev: NetworkEvent) {
        //self.socket_send(ev.op_code(), &ev);
        self.client.socket_send(-1, &ev);
        self.client.tick();
    }

    pub fn is_host(&mut self, remote_players: &BTreeSet<String>) -> bool {
        // no other players connected
        if remote_players.len() == 0 {
            return true;
        }

        *self.client.session_id.as_ref().unwrap() == *remote_players.iter().next().unwrap()
    }
}

impl Default for NakamaApi {
    fn default() -> Self {
        Self::new()
    }
}

pub fn receive_events(nakama: &mut NakamaApi) -> Vec<NetworkEvent> {
    nakama.client.tick();
    let mut evs = vec![];
    while let Some(event) = nakama.client.try_recv() {
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

pub fn network_player_manager_system(
    evs: &Vec<NetworkEvent>,
    players: &mut BTreeSet<String>,
) -> SystemResult {
    for ev in evs {
        match ev {
            NetworkEvent::PlayerJoin { id, username: _ } => {
                players.insert(id.clone());
            }
            NetworkEvent::PlayerLeave { id } => {
                players.remove(id);
            }
            _ => {}
        }
    }
    Ok(())
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
