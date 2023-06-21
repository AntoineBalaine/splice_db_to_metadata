use lofty::Probe;

use lofty::{Accessor, TaggedFileExt};
use std::path::Path;

pub(crate) fn tag_reader() {
    let path_str = std::env::args().nth(1).expect("ERROR: No path specified!");
    let path = Path::new(&path_str);

    if !path.is_file() {
        panic!("ERROR: Path is not a file!");
    }

    let tagged_file = Probe::open(path)
        .expect("ERROR: Bad path provided!")
        .read()
        .expect("ERROR: Failed to read file!");

    let tag = match tagged_file.primary_tag() {
        Some(primary_tag) => primary_tag,
        // If the "primary" tag doesn't exist, we just grab the
        // first tag we can find. Realistically, a tag reader would likely
        // iterate through the tags to find a suitable one.
        None => tagged_file.first_tag().expect("ERROR: No tags found!"),
    };

    println!("Artist: {}", tag.artist().as_deref().unwrap_or("None"));
    // import keys from https://docs.rs/lofty/latest/lofty/enum.ItemKey.html
}
