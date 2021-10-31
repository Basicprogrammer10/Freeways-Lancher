use std::fmt;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use simple_config_parser::config::Config;
use tar::Archive;

pub struct ResourcePack {
    pub name: String,
    pub author: String,
    pub version: String,

    pub files: Vec<RawFile>,
}

#[derive(Debug)]
pub struct RawFile {
    pub name: String,
    pub data: Vec<u8>,
}

impl ResourcePack {
    pub fn load(path: PathBuf) -> Option<ResourcePack> {
        let mut conf = Config::new(None);
        let mut files = Vec::new();

        let data = File::open(path).ok()?;
        let mut a = Archive::new(data);

        // Get Pack Meta
        for i in a.entries().ok()? {
            let mut file = i.ok()?;

            let mut buf = Vec::new();
            file.read_to_end(&mut buf).ok()?;

            let name = file.path().ok()?.to_string_lossy().to_string();

            files.push(RawFile {
                name: name.clone(),
                data: buf.clone(),
            });

            if name.as_str().to_lowercase() != "pack.meta" || !conf.data.is_empty() {
                continue;
            }

            conf.parse(&String::from_utf8_lossy(&buf).replace('\r', ""))
                .ok()?;
        }

        Some(ResourcePack {
            name: conf.get("name")?,
            author: conf.get("author")?,
            version: conf.get("version")?,
            files,
        })
    }
}

impl fmt::Debug for ResourcePack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ResourcePack")
            .field("name", &self.name)
            .field("author", &self.author)
            .field("version", &self.version)
            .finish()
    }
}
