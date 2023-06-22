enum TagNames {
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

impl TagNames {
    fn as_splice(&self) -> &'static str {
        match self {
            TagNames::Album => "TALB",
            TagNames::Artist => "TPE1",
            TagNames::ChordType => "",
            TagNames::Description => "TIT3", // subtitle field -- other option would be COMM ou ICMT for comment
            TagNames::Genre => "genre",
            TagNames::Key => "key",
            TagNames::Tempo => "bpm",
            TagNames::Loop => "sample_type",
            TagNames::SampleGenre => "sample_genre",
        }
    }
    fn as_id3(&self) -> &'static str {
        match self {
            TagNames::Album => "TALB",
            TagNames::Artist => "TPE1",
            TagNames::ChordType => "",
            TagNames::Description => "TIT3", // subtitle field -- other option would be COMM ou ICMT for comment
            TagNames::Genre => "TCON",
            TagNames::Key => "TKEY",
            TagNames::Tempo => "TBPM",
            TagNames::Loop => "",
            TagNames::SampleGenre => TagNames::Genre.as_id3(),
        }
    }
    fn as_xmp(&self) -> &'static str {
        match self {
            TagNames::Album => "album",
            TagNames::Artist => "artist",
            TagNames::ChordType => "scaleType",
            TagNames::Description => "logComment", // subtitle field
            TagNames::Genre => "genre",
            TagNames::Key => "key",
            TagNames::Tempo => "tempo",
            TagNames::Loop => "loop",
            TagNames::SampleGenre => TagNames::Genre.as_xmp(),
        }
    }
    fn as_riff(&self) -> &'static str {
        match self {
            TagNames::Album => "IPRD",
            TagNames::Artist => "IART",
            TagNames::ChordType => "",
            TagNames::Description => "ICMT", // comment field
            TagNames::Genre => "IGNR",
            TagNames::Key => "",
            TagNames::Tempo => "",
            TagNames::Loop => "",
            TagNames::SampleGenre => TagNames::Genre.as_riff(),
        }
    }
}
