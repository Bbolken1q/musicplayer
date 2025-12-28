// use std::{fs, thread, time};

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

use ego_tree::NodeRef;
use player::player::*;
use player::song_object::*;
use filesystem::filesystem::*;

fn main() {
    init().expect("Failed to initialize filesystem");

    let fs_tree = get_file_tree("./media/");
    // print_tree_fancy(&fs_tree);
    let mut working_directory: NodeRef<DirectoryEntry> = fs_tree.root();
    list_directory(&working_directory);

    change_working_directory(0, &mut working_directory);
    list_directory(&working_directory);

    change_working_directory(-1, &mut working_directory);
    list_directory(&working_directory);
    
    change_working_directory(-1, &mut working_directory);
    list_directory(&working_directory);
    // let mut player = Player::new();
    // let song = SongObject::new("./testing/test.mp3", Some("Test Song"), Some("Test Artist"));

    // player.play(song, Some(10));
    
    // loop {

    // }
}
