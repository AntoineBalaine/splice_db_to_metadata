use crate::tag_reader::read_file;
use bwavfile::WaveReader;

pub(crate) fn ixml_reader() {
    let mut file_content = read_file().unwrap();
    let mut r = WaveReader::new(file_content).unwrap();
    /*     let format = r.format().unwrap();

    let mut frame_reader = r.audio_frame_reader().unwrap();
    let mut buffer = format.create_frame_buffer::<i32>(1);

    let read = frame_reader.read_frames(&mut buffer).unwrap(); */
    let mut vector: Vec<u8> = Vec::new();
    let ixml = r.read_ixml(&mut vector);
    match ixml {
        Ok(ixml) => {
            println!("IXML");
            println!("{:?}", ixml);
            println!("{:?}", String::from_utf8(vector));
        }
        Err(err) => {
            println!("Error: {:?}", err);
        }
    }
}
