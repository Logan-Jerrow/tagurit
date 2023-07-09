use std::path::Path;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Mp3;

impl std::convert::TryFrom<&Path> for Mp3 {
    type Error = String;

    fn try_from(value: &Path) -> Result<Self, Self::Error> {
        if let Some(ext) = value.extension() {
            if ext == "mp3" {
                return Ok(Self);
            }
        }
        Err("Not a mp3".to_string())
    }
}
