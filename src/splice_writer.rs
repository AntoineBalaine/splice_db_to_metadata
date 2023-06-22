use lofty::AudioFile;
use lofty::ParseOptions;
use lofty::{Accessor, TagExt};
use std::fs::File;
use std::path::Path;

use lofty;

use sqlite::State;

use sqlite::Connection;

pub(crate) fn write_tags_from_db() {
    let path = Path::new("/Users/antoine/Library/Application Support/com.splice.Splice/users/default/Perken/sounds.db");
    // create db connection using sqlite
    let connection = Connection::open(path).unwrap();
    // read users table and get 100 first entries
    let query = "SELECT s.local_path as path, s.audio_key as key, s.bpm, s.chord_type, s.tags, s.sample_type,  p.name as album_name, s.genre as sample_genre, p.genre, p.provider_name as artist
FROM samples AS s
JOIN packs AS p ON s.pack_uuid = p.uuid;";

    let mut statement = connection.prepare(query).unwrap();
    statement.bind((1, 50)).unwrap();

    while let Ok(State::Row) = statement.next() {
        let read_tag = |tag: &str| {
            let thing = statement.read::<String, _>(tag);
            match thing {
                Ok(thing) => thing,
                Err(_) => "".to_string(),
            }
        };
        let path = read_tag("path");
        // import type of statement
        let key = read_tag("key");
        let bpm = read_tag("bpm");
        let chord_type = read_tag("chord_type"); // included in xmpDM as scaleType
        let tags = read_tag("tags");
        let sample_type = read_tag("sample_type"); // loop/one-shot
        let album_name = read_tag("album_name");
        let sample_genre = read_tag("sample_genre");
        let genre = read_tag("genre");
        let artist = read_tag("artist");

        let mut file_content = File::open(path.clone()).unwrap();
        let mut wavfile =
            lofty::iff::wav::WavFile::read_from(&mut file_content, ParseOptions::new())
                .expect("ERROR: Bad path provided!");
        let tag = wavfile.riff_info_mut().unwrap();
        tag.insert("key".to_string(), key.clone());

        tag.insert("key".to_string(), key);
        tag.insert("tempo".to_string(), bpm);
        tag.insert("chord_type".to_string(), chord_type);
        tag.insert("description".to_string(), tags);
        tag.insert("type".to_string(), sample_type);
        tag.set_artist(artist);
        tag.set_album(album_name);
        tag.set_genre(match sample_genre {
            x if x == "".to_string() => genre,
            _ => sample_genre,
        });

        // how to set a bwf xml tag?

        tag.save_to_path(path)
            .expect("ERROR: Failed to write the tag!");

        println!("INFO: Tag successfully updated!");
    }
}
