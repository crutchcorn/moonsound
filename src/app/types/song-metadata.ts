export interface SongTagsMetadata { "Album": string, "AlbumArtist": string, "Artist": string, "Comment": string, "Date": string, "IdentIsrc": string, "Lyrics": string, "TXXX:replaygain_album_gain": string, "TXXX:replaygain_track_gain": string, "TXXX:replaygain_track_peak": string, "TrackNumber": string, "TrackTitle": string }

export interface SongVisualsTagsMetadata {
    "Description": string,
}

export interface SongVisualsMetadata {
    "media_type": string,
    // 0x0 ???
    "dimensions": string,
    "data": number[],
    tags: SongVisualsTagsMetadata
}

export interface SongMetadata {
    tags: SongTagsMetadata,
    visuals: Record<string, SongVisualsMetadata>
}