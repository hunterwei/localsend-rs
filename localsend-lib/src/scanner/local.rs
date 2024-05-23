use std::{
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
};

use localsend_proto::Device;

#[derive(Debug, Clone)]
pub struct LocalDeviceScanner {
    path: PathBuf,
}

impl LocalDeviceScanner {
    pub fn new(path: impl AsRef<Path>) -> std::io::Result<Self> {
        let path = shellexpand::path::full(path.as_ref())
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::NotFound, e))?;
        Ok(Self {
            path: path.into_owned(),
        })
    }

    pub async fn read_local_devices(&self) -> std::io::Result<Vec<Device>> {
        if !self.path.exists() {
            return Ok(vec![]);
        }
        let file = File::open(&self.path)?;
        let reader = BufReader::new(file);
        let local_devices = serde_json::from_reader(reader)?;
        log::trace!("found local devices: {:?}", local_devices);
        Ok(local_devices)
    }
}
