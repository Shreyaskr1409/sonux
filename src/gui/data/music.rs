use std::{collections::HashMap, path::PathBuf};

use jiff::Zoned;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SongId(pub usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct AlbumId(pub usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ArtistId(pub usize);

#[derive(Debug)]
pub struct Song {
    pub id: SongId,
    pub title: String,
    pub track_no: u16,
    pub disc_no: u16,
    pub artist: ArtistId,
    pub album_artist: ArtistId,
    pub year: Option<u16>,
    pub duration: Zoned,
    pub path: PathBuf,
}

#[derive(Debug)]
pub struct Album {
    pub id: AlbumId,
    pub title: String,
    pub album_artist: ArtistId,
    pub year: Option<u16>,
    pub songs: Vec<SongId>,
    pub duration: Zoned,
}

#[derive(Debug)]
pub struct Artist {
    pub id: ArtistId,
    pub name: String,
    pub albums: Vec<AlbumId>,
}

pub struct Library {
    pub songs: HashMap<SongId, Song>,
    pub albums: HashMap<AlbumId, Album>,
    pub artists: HashMap<ArtistId, Artist>,
}
