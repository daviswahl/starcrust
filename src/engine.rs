use protobuf::Message;
use protos::{self, sc2api};

fn create_game_req() -> Vec<u8> {
    let mut req = sc2api::Request::new();
    let mut game = sc2api::RequestCreateGame::new();
    let mut map = sc2api::LocalMap::new();

    let mut player_setup1 = sc2api::PlayerSetup::new();
    player_setup1.set_race(protos::common::Race::Zerg);
    player_setup1.set_player_name("foo".to_owned());
    player_setup1.set_field_type(sc2api::PlayerType::Computer);

    let mut player_setup2 = sc2api::PlayerSetup::new();
    player_setup2.set_race(protos::common::Race::Zerg);
    player_setup2.set_player_name("foo2".to_owned());
    player_setup2.set_field_type(sc2api::PlayerType::Observer);

    game.set_player_setup(vec![player_setup1, player_setup2].into());

    map.set_map_path("/Applications/StarCraft II/Maps/Ladder/(2)Bel\'ShirVestigeLE (Void).SC2Map".to_owned());
    game.set_local_map(map);
    req.set_create_game(game);

    let mut v = vec![];
    req.write_to_vec(& mut v);
    v
}