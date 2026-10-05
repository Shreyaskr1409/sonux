use std::{
    collections::{BTreeSet, HashMap},
    path::PathBuf,
    time::Duration,
};

use uuid::Uuid;

#[derive(Debug)]
pub struct Metadata {
    pub path: String,
    pub title: String,
    pub track_no: u32,
    pub disc_no: u32,
    pub artist: String,
    pub album: String,
    pub album_artist: String,
    pub _release_date: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SongId(Uuid);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AlbumId {
    pub album_artist: ArtistNameAsId,
    pub title: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ArtistNameAsId(pub String);

impl ArtistNameAsId {
    pub fn new(name: &String) -> Self {
        let name = name.trim();

        if name.is_empty() {
            Self("Unknown Artist".to_string())
        } else {
            Self(name.to_string())
        }
    }
}

#[derive(Debug)]
pub struct Song {
    // File
    pub _path: PathBuf,

    // Metadata
    pub _title: String,
    pub _track_no: u16,
    pub _disc_no: u16,

    // Relationships
    pub _artist: ArtistNameAsId,
    pub _album: AlbumId,

    // Technical information
    pub _duration: Option<Duration>,
}

#[derive(Debug)]
pub struct Album {
    pub _title: String,
    pub _album_artist: ArtistNameAsId,
    pub _year: Option<u16>,

    pub songs: Vec<SongId>,
}

impl Album {
    pub fn new(title: String, album_artist: String, year: Option<u16>) -> Album {
        Album {
            _title: title,
            _album_artist: ArtistNameAsId(album_artist),
            _year: year,
            songs: Vec::new(),
        }
    }

    pub fn add_song(&mut self, song_id: SongId) {
        self.songs.push(song_id);
    }
}

#[derive(Debug)]
pub struct Artist {
    pub id: ArtistNameAsId,
}

impl Artist {
    pub fn new(name: String) -> Artist {
        Artist {
            id: ArtistNameAsId(name),
        }
    }
}

#[derive(Debug, Default)]
pub struct Library {
    pub songs: HashMap<SongId, Song>,
    pub albums: HashMap<AlbumId, Album>,
    pub artists: HashMap<ArtistNameAsId, Artist>,
}

impl Library {
    pub fn new() -> Self {
        Library {
            songs: HashMap::new(),
            albums: HashMap::new(),
            artists: HashMap::new(),
        }
    }

    pub fn populate_fields(lib: &mut Self, metadata_list: &Vec<Metadata>) {
        for elem in metadata_list {
            let song_id = SongId(Uuid::now_v7());
            let artist_id = ArtistNameAsId::new(&elem.album_artist);
            let album_id = AlbumId {
                album_artist: artist_id.clone(),
                title: elem.album.clone(),
            };

            let year = Some(0000); // to be replaced with year extracted from release date field

            lib.artists
                .entry(artist_id.clone())
                .or_insert_with(|| Artist::new(artist_id.0.clone()));

            let album = lib
                .albums
                .entry(album_id.clone())
                .or_insert_with(|| Album::new(elem.album.clone(), artist_id.0.clone(), year));

            let song = Song {
                _path: PathBuf::from(&elem.path),
                _title: elem.title.clone(),
                _track_no: elem.track_no as u16,
                _disc_no: elem.disc_no as u16,
                _artist: ArtistNameAsId(elem.artist.clone()),
                _album: album_id,
                _duration: None,
            };

            lib.songs.insert(song_id, song);
            album.add_song(song_id);
        }
    }

    pub fn _songs_for_album(&self, album_id: &AlbumId) -> Vec<SongId> {
        let album = match self.albums.get(album_id) {
            Some(a) => a,
            None => return Vec::new(),
        };
        let mut songs = album.songs.clone();

        songs.sort_by_key(|id| {
            let song = self.songs.get(id).unwrap();
            (song._disc_no, song._track_no)
        });
        songs
    }

    pub fn albums_for_artists(&self, artist_ids: &[&ArtistNameAsId]) -> Vec<AlbumId> {
        let mut albums: Vec<AlbumId> = self
            .albums
            .iter()
            .filter(|(_, album)| artist_ids.contains(&&album._album_artist))
            .map(|(album_id, _)| album_id.clone())
            .collect();

        albums.sort_by(|a, b| a.title.cmp(&b.title));
        albums
    }

    pub fn artist_initial_char(&self) -> Vec<char> {
        let initials: BTreeSet<char> = self
            .artists
            .values()
            .filter_map(|artist| {
                artist
                    .id
                    .0
                    .chars()
                    .next()
                    .map(|c| c.to_uppercase().next().unwrap_or(c))
            })
            .collect();

        initials.into_iter().collect()
    }
}

/// Display data of a track, detached from `Library`
#[derive(Debug, Clone)]
pub struct TrackSummary {
    pub track_no: u16,
    pub title: String,
    pub duration: Option<Duration>,
}

/// Display data of an album with its tracks, detached from `Library`
#[derive(Debug, Clone)]
pub struct AlbumSummary {
    pub _title: String,
    pub album_artist: ArtistNameAsId,
    pub _year: Option<u16>,
    pub _total_duration: Option<Duration>,
    pub tracks: Vec<TrackSummary>,
}

impl Library {
    /// Collects everything needed to display an album; tracks are ordered by disc and track number
    pub fn album_summary(&self, album_id: &AlbumId) -> Option<AlbumSummary> {
        let album = self.albums.get(album_id)?;

        let tracks: Vec<TrackSummary> = self
            ._songs_for_album(album_id)
            .into_iter()
            .filter_map(|id| self.songs.get(&id))
            .map(|song| TrackSummary {
                track_no: song._track_no,
                title: song._title.clone(),
                duration: song._duration,
            })
            .collect();

        Some(AlbumSummary {
            _title: album._title.clone(),
            album_artist: album._album_artist.clone(),
            _year: album._year.filter(|y| *y != 0),
            _total_duration: tracks
                .iter()
                .filter_map(|t| t.duration)
                .reduce(|a, b| a + b),
            tracks,
        })
    }
}
