mod splice_writer;
mod tag_writer;
mod xmp_read;

fn main() {
    //tag_reader::tag_reader();
    //tag_writer::tag_writer();
    if let Err(err) = splice_writer::write_tags_from_db() {
        eprintln!("Error: {:?}", err);
    }
}

mod tag_reader;
