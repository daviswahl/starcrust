use protobuf::Message;
use protos::{self, sc2api};

use connection::Connection;
use tokio::prelude::Sink;
use std::sync::Arc;
use std::sync::Mutex;
use futures::Future;
use futures::future::Loop;
use unit::unit;
use std::time::Instant;
use std::time::Duration;
use std::ops::Add;

pub struct Engine<C: Connection> {
    conn: Arc<Mutex<C>>,
}

pub fn new<C: Connection>(conn: C) -> Engine<C> {
    Engine { conn: Arc::new(Mutex::new(conn)) }
}

impl<C: Connection> Engine<C> {
    pub fn run(&self) -> impl Future<Item=()> {
        if let Ok(mut conn) = self.conn.lock() {
            conn.send(create_game_req());
            conn.send(join());
        }

        futures::future::loop_fn(self.conn.clone(), |conn| {
            if let Ok(mut conn) = conn.try_lock() {
                while let Some(t) = conn.recv() {
                    println!("message: {:?}", t)
                }
                let mut request = sc2api::Request::new();
                let mut obs = sc2api::RequestObservation::new();
                let mut data = sc2api::RequestData::new();
                data.set_ability_id(true);
                data.set_buff_id(true);
                data.set_unit_type_id(true);

                request.set_data(data);
                conn.send(request);
            }




            tokio::timer::Delay::new(Instant::now() + Duration::from_millis(100)).map(|_| Loop::Continue(conn))
        })
    }
}

// join game
fn join() -> sc2api::Request {
    let mut req = sc2api::Request::new();
    let mut join_game = sc2api::RequestJoinGame::new();
    let mut options = sc2api::InterfaceOptions::new();
    options.set_raw(false);
    join_game.set_race(protos::common::Race::Zerg);
    join_game.set_options(options);
    req.set_join_game(join_game);
    req

}
fn create_game_req() -> sc2api::Request {
    let mut req = sc2api::Request::new();
    let mut game = sc2api::RequestCreateGame::new();
    game.set_realtime(true);
    let mut map = sc2api::LocalMap::new();

    let mut player_setup1 = sc2api::PlayerSetup::new();
    player_setup1.set_race(protos::common::Race::Zerg);
    player_setup1.set_player_name("foo".to_owned());
    player_setup1.set_field_type(sc2api::PlayerType::Computer);
    player_setup1.set_difficulty(sc2api::Difficulty::VeryHard);

    let mut player_setup2 = sc2api::PlayerSetup::new();
    player_setup2.set_race(protos::common::Race::Zerg);
    player_setup2.set_player_name("foo2".to_owned());
    player_setup2.set_field_type(sc2api::PlayerType::Participant);

    game.set_player_setup(vec![player_setup1, player_setup2].into());

    map.set_map_path("/Applications/StarCraft II/Maps/Ladder/void.SC2Map".to_owned());
    game.set_local_map(map);
    req.set_create_game(game);

    req
}