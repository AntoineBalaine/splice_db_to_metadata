use std::{
    env,
    fs::File,
    path::{Path, PathBuf},
};

use riff_io::{ChunkMeta, Entry, RiffFile};
pub struct NksFile {
    file: RiffFile,
}

impl NksFile {
    pub fn load(path: &Path) -> Result<Self, &'static str> {
        let file = RiffFile::open(&path.to_string_lossy())
            .map_err(|_| "Couldn't fine preset file or doesn't have RIFF format")?;
        Ok(Self { file })
    }
    fn relevant_bytes_of_chunk(&self, chunk: &ChunkMeta) -> &[u8] {
        let skip = 4;
        let offset = chunk.data_offset + skip;
        let size = chunk.chunk_size - skip;
        let range = offset..(offset + size);
        self.file.read_bytes(range)
    }
    pub fn content(&self) -> Result<NksFileContent, &'static str> {
        let entries = self
            .file
            .read_entries()
            .map_err(|_| "couldn't read entries")?;
        let mut nisi_chunk = None;
        for entry in entries {
            if let Entry::Chunk(chunk_meta) = entry {
                match &chunk_meta.chunk_id {
                    /*                     b"PLID" => plid_chunk = Some(chunk_meta),
                    b"NICA" => nica_chunk = Some(chunk_meta),
                    b"PCHK" => pchk_chunk = Some(chunk_meta), */
                    b"NISI" => nisi_chunk = Some(chunk_meta),
                    _ => {}
                }
            }
        }
        let content = NksFileContent {
            metadata: nisi_chunk
                .and_then(|ch| {
                    let bytes = self.relevant_bytes_of_chunk(&ch);
                    rmp_serde::from_slice(bytes).ok()
                })
                .unwrap_or_default(),
        };
        Ok(content)
    }
}

#[derive(Debug)]
pub struct NksFileContent {
    pub metadata: NisiChunkContent,
    /*     pub plugin_id: PluginId,
    pub vst_chunk: &'a [u8],
    pub macro_param_banks: Vec<MacroParamBank>, */
}
#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct NisiChunkContent {
    #[serde(rename = "author")]
    pub author: Option<String>,
    #[serde(rename = "bankchain")]
    pub bankchain: Vec<String>,
    #[serde(rename = "deviceType")]
    pub device_type: Option<String>,
    #[serde(rename = "modes")]
    pub modes: Vec<String>,
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[serde(rename = "types")]
    pub types: Vec<String>,
    #[serde(rename = "vendor")]
    pub vendor: Option<String>,
}

pub struct Destination {
    pub name: String,
    pub path: PathBuf,
}

fn load_nks_preset(path: &Path /* , destination: &Destination */) -> Result<(), &'static str> {
    let nks_file = NksFile::load(path)?;
    let nks_content = nks_file.content()?;
    let metadata = nks_content.metadata;
    // print all contents of metadata
    println!("{:#?}", metadata);
    Ok(())
}

pub fn read_presets_from_dir() {
    let dir_in = env::args().nth(1).expect("Please specify an input file.");
    let path = Path::new(dir_in.as_str());
    if let Ok(entries) = std::fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if let Some(extension) = path.extension() {
                    if extension == "nksf" {
                        load_nks_preset(&path);
                    }
                }
            }
        }
    } else {
        println!("Couldn't read directory");
    }
    ()
}
