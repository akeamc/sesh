use std::{collections::HashMap, io::Write, path::PathBuf};

use clap::Parser;
use sesh::Stream;
use tabwriter::TabWriter;

#[derive(Debug, Parser)]
struct Cli {
    path: PathBuf,
    #[clap(long = "count", short = 'n', default_value = "100")]
    n_tracks: usize,
}

struct TrackInfo {
    name: String,
    artist: String,
}

fn get_track_info(stream: &Stream) -> Option<TrackInfo> {
    let name = stream.master_metadata_track_name.as_ref()?;
    let artist = stream.master_metadata_album_artist_name.as_ref()?;
    Some(TrackInfo {
        name: name.to_owned(),
        artist: artist.to_owned(),
    })
}

fn main() -> anyhow::Result<()> {
    let Cli { path, n_tracks } = Cli::parse();

    // find the least played tracks
    let mut track_info = HashMap::new();
    let mut track_play_counts = HashMap::new();

    sesh::read_zip(path, |stream| {
        if let Some(track) = &stream.spotify_track_uri {
            if let Some(info) = get_track_info(&stream) {
                track_info.insert(track.clone(), info);
            }
            *track_play_counts.entry(track.clone()).or_insert(0u32) += 1;
        }
    })?;

    let mut track_play_counts: Vec<_> = track_play_counts.into_iter().collect();
    track_play_counts.sort_by_key(|(_, count)| *count);

    let mut tw = TabWriter::new(vec![]);

    tw.write_all(b"Track\tArtist\tCount\n").unwrap();
    for (track, count) in track_play_counts
        .iter()
        .rev()
        .filter_map(|(track_uri, count)| track_info.get(track_uri).map(|info| (info, count)))
        .take(n_tracks)
    {
        tw.write_all(format!("{}\t{}\t{count}\n", track.name, track.artist).as_bytes())
            .unwrap();
    }
    tw.flush()?;

    std::io::stdout().write_all(&tw.into_inner()?)?;

    Ok(())
}
