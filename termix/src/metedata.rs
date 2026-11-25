use lofty::file::{AudioFile, TaggedFileExt};
use lofty::read_from_path;
use lofty::tag::Accessor;
use std::error::Error;
use std::path::Path;
use std::time::Duration;

pub struct TrackInfo {
    pub title: String,
    pub artist: String,
    pub album: String,
    pub duration: Duration,
    pub cover_art: Option<Vec<u8>>,
}
impl TrackInfo {
    pub fn from_path(path: &Path) -> Result<Self, Box<dyn Error>> {
        let tagged_file = read_from_path(path)?;
        let duration = tagged_file.properties().duration();
        let tag = tagged_file.primary_tag();

        
        
        let title = tag
            .and_then(|t| t.title())
            .map(|s| s.into_owned())
            .unwrap_or_else(|| "unknown Title".to_string());
        
        
        let artist = tag
            .and_then(|t| t.artist())
            .map(|s| s.into_owned())
            .unwrap_or_else(|| "unknown Artist".to_string());
        
        
        let album = tag
            .and_then(|t| t.album())
            .map(|s| s.into_owned())
            .unwrap_or_else(|| "unknown Album".to_string());
        
        
        let cover_art = tag
            .and_then(|t| t.pictures().first())
            .map(|pic| pic.data().to_vec());

        Ok(Self {
            title,
            artist,
            album,
            duration,
            cover_art,
        })
    }
}