use anyhow::{anyhow, Context, Result};
use std::path::Path;
use xmp_toolkit::{OpenFileOptions, XmpFile, XmpMeta, XmpValue};

use sqlite::State;

use sqlite::Connection;

pub(crate) fn write_tags_from_db() -> Result<()> {
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
        let read_tag = |tag: &str| match statement.read::<String, _>(tag) {
            Ok(thing) => thing,
            Err(_) => "".to_string(),
        };
        let path = &read_tag("path");

        if path != "" {
            let tag = TagVal {
                album: read_tag(Tag::Album.as_splice()),
                artist: read_tag(Tag::Artist.as_splice()),
                chord_type: read_tag(Tag::ChordType.as_splice()),
                description: read_tag(Tag::Description.as_splice()),
                genre: read_tag(Tag::Genre.as_splice()),
                key: read_tag(Tag::Key.as_splice()),
                tempo: read_tag(Tag::Tempo.as_splice()),
                is_loop: read_tag(Tag::Loop.as_splice()),
                sample_genre: read_tag(Tag::SampleGenre.as_splice()),
            };
            let mut f = XmpFile::new().unwrap();
            f.open_file(
                path,
                OpenFileOptions::default()
                    .for_update()
                    .only_xmp()
                    .use_smart_handler(),
            )
            .or_else(|_err| {
                // There might not be an appropriate handler available.
                // Retry using packet scanning, providing a different set of
                // open-file options.
                eprintln!(
                    "No smart handler available for file {}. Trying packet scanning.",
                    path
                );
                f.open_file(path, OpenFileOptions::default().use_packet_scanning())
            })
            .with_context(|| format!("could not find XMP in file {}", path))?;

            // Retrieve the XMP from the file.
            let mut xmp = match f
                .xmp()
                .ok_or_else(|| anyhow!("unable to process XMP in file {}", path))
            {
                Ok(xmp) => xmp,
                Err(err) => {
                    eprintln!("Error: {:?}", err);
                    XmpMeta::new().unwrap()
                }
            };

            let xmp_dm_uri = "http://ns.adobe.com/xmp/1.0/DynamicMedia/".to_string();
            XmpMeta::register_namespace(xmp_dm_uri.as_str(), "xmpDM")?;

            if f.can_put_xmp(&xmp) {
                set_tags(&mut xmp, xmp_dm_uri, tag);

                f.put_xmp(&xmp).unwrap();
                f.close();
            } else {
                println!("can't update file");
            }
        }
    }
    Ok(())
}

fn set_tags(xmp: &mut XmpMeta, namespace: String, tag: TagVal) {
    xmp.set_property(
        namespace.as_str(),
        Tag::Artist.as_xmp(),
        &XmpValue::new(tag.artist),
    )
    .ok();
    xmp.set_property(
        namespace.as_str(),
        Tag::Album.as_xmp(),
        &XmpValue::new(tag.album),
    )
    .ok();
    xmp.set_property(
        namespace.as_str(),
        Tag::ChordType.as_xmp(),
        &XmpValue::new(tag.chord_type),
    )
    .ok();
    xmp.set_property(
        namespace.as_str(),
        Tag::Description.as_xmp(),
        &XmpValue::new(tag.description),
    )
    .ok();
    xmp.set_property(
        namespace.as_str(),
        Tag::Genre.as_xmp(),
        &XmpValue::new(match tag.sample_genre.as_str() {
            "" => tag.genre,
            _ => tag.sample_genre,
        }),
    )
    .ok();
    xmp.set_property(
        namespace.as_str(),
        Tag::Key.as_xmp(),
        &XmpValue::new(tag.key),
    )
    .ok();
    xmp.set_property(
        namespace.as_str(),
        Tag::Tempo.as_xmp(),
        &XmpValue::new(tag.tempo),
    )
    .ok();
    xmp.set_property_bool(
        namespace.as_str(),
        Tag::Loop.as_xmp(),
        &XmpValue::new(match tag.is_loop.as_str() {
            "loop" => true,
            _ => false,
        }),
    )
    .ok();
}

pub enum Tag {
    Album,
    Artist,
    ChordType,
    Description,
    Genre,
    Key,
    Tempo,
    Loop,
    SampleGenre,
}

impl Tag {
    fn as_splice(&self) -> &'static str {
        match self {
            Tag::Album => "album_name",
            Tag::Artist => "artist",
            Tag::ChordType => "chord_type",
            Tag::Description => "tags", // subtitle field -- other option would be COMM ou ICMT for comment
            Tag::Genre => "genre",
            Tag::Key => "key",
            Tag::Tempo => "bpm",
            Tag::Loop => "sample_type",
            Tag::SampleGenre => "sample_genre",
        }
    }
    fn as_id3(&self) -> &'static str {
        match self {
            Tag::Album => "TALB",
            Tag::Artist => "TPE1",
            Tag::ChordType => "",
            Tag::Description => "TIT3", // subtitle field -- other option would be COMM ou ICMT for comment
            Tag::Genre => "TCON",
            Tag::Key => "TKEY",
            Tag::Tempo => "TBPM",
            Tag::Loop => "",
            Tag::SampleGenre => Tag::Genre.as_id3(),
        }
    }
    fn as_xmp(&self) -> &'static str {
        match self {
            Tag::Album => "album",
            Tag::Artist => "artist",
            Tag::ChordType => "scaleType",
            Tag::Description => "logComment", // subtitle field
            Tag::Genre => "genre",
            Tag::Key => "key",
            Tag::Tempo => "tempo",
            Tag::Loop => "loop",
            Tag::SampleGenre => Tag::Genre.as_xmp(),
        }
    }
    fn as_riff(&self) -> &'static str {
        match self {
            Tag::Album => "IPRD",
            Tag::Artist => "IART",
            Tag::ChordType => "",
            Tag::Description => "ICMT", // comment field
            Tag::Genre => "IGNR",
            Tag::Key => "",
            Tag::Tempo => "",
            Tag::Loop => "",
            Tag::SampleGenre => Tag::Genre.as_riff(),
        }
    }
}

struct TagVal {
    album: String,
    artist: String,
    chord_type: String,
    description: String,
    genre: String,
    key: String,
    tempo: String,
    is_loop: String,
    sample_genre: String,
}
