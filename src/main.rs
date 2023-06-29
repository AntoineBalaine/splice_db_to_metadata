// mod read_nksf;
mod splice_writer;
mod tag_ogg_file;
mod tag_writer;
mod xmp_read;
use anyhow::Context;
fn main() {
    //tag_reader::tag_reader();
    //tag_writer::tag_writer();

    /*     if let Err(err) = xmp_read::xmp_read().context("could not read XMP from file") {
        eprintln!("Error: {:?}", err);
        std::process::exit(1);
    } */
    /*     if let Err(err) = splice_writer::write_tags_from_db() {
        eprintln!("Error: {:?}", err);
    } */
    tag_ogg_file::tag_file();
}

mod tag_reader;
