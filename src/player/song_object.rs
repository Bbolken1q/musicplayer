use std::fs::File;
use either::Either;
use rodio::Decoder;
use crate::util::error::Error;


pub struct SongObject {
    pub title: String,
    pub artist: String,
    path: String,
}

impl SongObject {
    pub fn new(path: &str, title: Option<&str>, artist: Option<&str>) -> Self {
        SongObject {
            title: title.unwrap_or("None").to_string(),
            artist: artist.unwrap_or("None").to_string(),
            path: path.to_string(),
        }
    }

    pub fn update(&mut self, title: Option<&str>, artist: Option<&str>){
        self.title = title.unwrap_or(&self.title).to_string();
        self.artist = artist.unwrap_or(&self.artist).to_string();
    }

    pub fn get_song(&self) -> Either<Decoder<std::io::BufReader<File>>, Error> {
        let music_file = match File::open(self.path.clone()) {
            Ok(file) => file,
            Err(_) => return Either::Right(Error{ code: 1 }), // return file not found
        };
        let source = match Decoder::try_from(music_file) {
            Ok(decoder) => decoder,
            Err(_) => return Either::Right(Error{ code: 2 }), // return unsupported format
        };
        return Either::Left(source);
    }
}

impl Clone for SongObject {
    fn clone(&self) -> Self {
        SongObject::new(&self.path.clone(), Some(&self.title.clone()), Some(&self.artist.clone()))
    }
}