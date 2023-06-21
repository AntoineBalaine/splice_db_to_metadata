mod splice_writer;
mod tag_writer;
mod xmp_read;
// splice_writer::write_tags_from_db();
//tag_writer::tag_writer();
use anyhow::Context;

fn main() {
    //tag_reader::tag_reader();

    if let Err(err) = xmp_read::xmp_read().context("could not read XMP from file") {
        eprintln!("Error: {:?}", err);
        std::process::exit(1);
    }
}

mod tag_reader;
