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
