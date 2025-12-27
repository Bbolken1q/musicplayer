use std::sync::{Arc, Mutex};
use std::{thread, time};

mod player {
    pub mod player;
    pub mod song_object;
}

mod util {
    pub mod error;
}

mod filesystem {
    pub mod filesystem;
}

use player::player::Player;
use player::song_object::SongObject;
use filesystem::filesystem::*;


fn main() {
    init().expect("Failed to initialize filesystem");
    // let mut player = Player::new();
    // let song = SongObject::new("./testing/test.mp3", Some("Test Song"), Some("Test Artist"));

    // player.play(song, Some(10));
    
    // loop {

    // }
}
