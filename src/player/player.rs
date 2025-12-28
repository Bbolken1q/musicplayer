use either::Either;
use rodio::Source;
// use core::time;
use std::{time::Duration};
use crate::player::song_object::SongObject;
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};

pub struct Player {
    is_playing: bool,
    is_alive: Arc<AtomicBool>,
    queue: Vec<SongObject>,
    stream_handle: rodio::OutputStream,
    sink: rodio::Sink,
    current_song: Option<SongObject>,

}

impl Player {
    pub fn new() -> Self {
        let stream_handle =
            rodio::OutputStreamBuilder::open_default_stream().expect("open default audio stream");
        let sink = rodio::Sink::connect_new(&stream_handle.mixer());

        Player {
            is_playing: false,
            is_alive: Arc::new(AtomicBool::new(true)),
            queue: Vec::new(),
            stream_handle,
            sink,
            current_song: None,
        }
    }

    pub fn play(&mut self, song: SongObject, start_time: Option<u32>) -> i8 {
        self.sink.stop();
            let value = match song.get_song() {
                Either::Left(source) => source,
                Either::Right(err) => {
                    println!("Error playing song: {}", err.read());
                    return -1;
                }
            };
            let source = value.skip_duration(Duration::from_secs(start_time.unwrap_or(0) as u64));
            self.current_song = Some(song);
            self.sink.append(source);
            self.is_playing = true;
        return 1;
    }

    pub fn enqueue(&mut self, song: SongObject) {
        // self.queue.push(song);
    }
}



impl Drop for Player {
    fn drop(&mut self) {

    }
}
