use lofty::{AudioFile, ParseOptions, TagType};
use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::path::Path;

pub(crate) fn tag_reader() {
    let mut file_content = read_file().unwrap();
    // put the file in a variable
    // if it fails to open, print an error
    // if it succeeds, put the file in a variable

    let tagged_file = lofty::iff::wav::WavFile::read_from(&mut file_content, ParseOptions::new())
        .expect("ERROR: Bad path provided!");
    if tagged_file.contains_tag_type(TagType::Id3v2) {
        let id3_tag = tagged_file.id3v2().unwrap();
        println!("ID3v2");
        id3_tag.into_iter().for_each(|frame| {
            println!("{}: {:?}", frame.id_str(), id3_tag.get_text(frame.id_str()));
        });
        // TBPM -> tempo in id3v2
        /*
                TALB: album
        TBPM: bpm
        TKEY: key
        TPE1: artist
                         */
        let id = id3_tag.get("TBPM");
        if let Some(id) = id {
            let text = id3_tag.get_text(id.id_str());
            println!("{}: {:?}", id.id_str(), text);
        }
    }

    if tagged_file.contains_tag_type(TagType::RiffInfo) {
        let riff_info = tagged_file.riff_info().unwrap();

        println!("RIFF INFO");
        riff_info.into_iter().for_each(|frame| {
            let (first, second) = frame;
            println!("{}: {:?}", first, second);
        });
    }
}

pub(crate) fn read_file() -> Result<File, io::Error> {
    let path_str = std::env::args().nth(1).expect("ERROR: No path specified!");
    let path = Path::new(&path_str);

    if !path.is_file() {
        return Err(io::Error::new(ErrorKind::NotFound, "Custom error message"));
    }
    File::open(path)
}

use xmp_toolkit;
use xmp_toolkit::XmpFile;
fn adobe_xmp_reapd() {
    let mut xmp_file = xmp_toolkit::XmpFile::new().unwrap();

    let path_str = std::env::args().nth(1).expect("ERROR: No path specified!");
    let path = Path::new(&path_str);
    let t = xmp_file
        .open_file(path, xmp_toolkit::OpenFileOptions::default())
        .unwrap();
}
