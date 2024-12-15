use symphonia::core::meta::StandardTag;

/**
 * This code may seem bad. It is.
 *
 * But also, I have no idea how to dynamically generate the keys for the tags in a safe way,
 * as I don't currently believe Rust supports this. If it does 🤷??
 */
pub fn get_keys_for_standard_tag(tag: StandardTag) -> String {
    match tag {
        StandardTag::AccurateRipCount(_) => "AccurateRipCount",
        StandardTag::AccurateRipCountAllOffsets(_) => "AccurateRipCountAllOffsets",
        StandardTag::AccurateRipCountWithOffset(_) => "AccurateRipCountWithOffset",
        StandardTag::AccurateRipCrc(_) => "AccurateRipCrc",
        StandardTag::AccurateRipDiscId(_) => "AccurateRipDiscId",
        StandardTag::AccurateRipId(_) => "AccurateRipId",
        StandardTag::AccurateRipOffset(_) => "AccurateRipOffset",
        StandardTag::AccurateRipResult(_) => "AccurateRipResult",
        StandardTag::AccurateRipTotal(_) => "AccurateRipTotal",
        StandardTag::AcoustIdFingerprint(_) => "AcoustIdFingerprint",
        StandardTag::AcoustIdId(_) => "AcoustIdId",
        StandardTag::Album(_) => "Album",
        StandardTag::AlbumArtist(_) => "AlbumArtist",
        StandardTag::Arranger(_) => "Arranger",
        StandardTag::Artist(_) => "Artist",
        StandardTag::Author(_) => "Author",
        StandardTag::Bpm(_) => "Bpm",
        StandardTag::CdToc(_) => "CdToc",
        StandardTag::Comment(_) => "Comment",
        StandardTag::CompilationFlag(_) => "CompilationFlag",
        StandardTag::Composer(_) => "Composer",
        StandardTag::Conductor(_) => "Conductor",
        StandardTag::ContentAdvisory(_) => "ContentAdvisory",
        StandardTag::Copyright(_) => "Copyright",
        StandardTag::CueToolsDbDiscConfidence(_) => "CueToolsDbDiscConfidence",
        StandardTag::CueToolsDbTrackConfidence(_) => "CueToolsDbTrackConfidence",
        StandardTag::Date(_) => "Date",
        StandardTag::Description(_) => "Description",
        StandardTag::DiscNumber(_) => "DiscNumber",
        StandardTag::DiscSubtitle(_) => "DiscSubtitle",
        StandardTag::DiscTotal(_) => "DiscTotal",
        StandardTag::EncodedBy(_) => "EncodedBy",
        StandardTag::Encoder(_) => "Encoder",
        StandardTag::EncoderSettings(_) => "EncoderSettings",
        StandardTag::EncodingDate(_) => "EncodingDate",
        StandardTag::Engineer(_) => "Engineer",
        StandardTag::Ensemble(_) => "Ensemble",
        StandardTag::Genre(_) => "Genre",
        StandardTag::Grouping(_) => "Grouping",
        StandardTag::IdentAsin(_) => "IdentAsin",
        StandardTag::IdentBarcode(_) => "IdentBarcode",
        StandardTag::IdentCatalogNumber(_) => "IdentCatalogNumber",
        StandardTag::IdentEanUpn(_) => "IdentEanUpn",
        StandardTag::IdentIsbn(_) => "IdentIsbn",
        StandardTag::IdentIsrc(_) => "IdentIsrc",
        StandardTag::IdentPn(_) => "IdentPn",
        StandardTag::IdentPodcast(_) => "IdentPodcast",
        StandardTag::IdentUpc(_) => "IdentUpc",
        StandardTag::IndexNumber(_) => "IndexNumber",
        StandardTag::InitialKey(_) => "InitialKey",
        StandardTag::InternetRadioName(_) => "InternetRadioName",
        StandardTag::InternetRadioOwner(_) => "InternetRadioOwner",
        StandardTag::Label(_) => "Label",
        StandardTag::LabelCode(_) => "LabelCode",
        StandardTag::Language(_) => "Language",
        StandardTag::License(_) => "License",
        StandardTag::Lyricist(_) => "Lyricist",
        StandardTag::Lyrics(_) => "Lyrics",
        StandardTag::MediaFormat(_) => "MediaFormat",
        StandardTag::MixDj(_) => "MixDj",
        StandardTag::MixEngineer(_) => "MixEngineer",
        StandardTag::Mood(_) => "Mood",
        StandardTag::MovementName(_) => "MovementName",
        StandardTag::MovementNumber(_) => "MovementNumber",
        StandardTag::MovementTotal(_) => "MovementTotal",
        StandardTag::Mp3GainAlbumMinMax(_) => "Mp3GainAlbumMinMax",
        StandardTag::Mp3GainMinMax(_) => "Mp3GainMinMax",
        StandardTag::Mp3GainUndo(_) => "Mp3GainUndo",
        StandardTag::MusicBrainzAlbumArtistId(_) => "MusicBrainzAlbumArtistId",
        StandardTag::MusicBrainzAlbumId(_) => "MusicBrainzAlbumId",
        StandardTag::MusicBrainzArtistId(_) => "MusicBrainzArtistId",
        StandardTag::MusicBrainzDiscId(_) => "MusicBrainzDiscId",
        StandardTag::MusicBrainzGenreId(_) => "MusicBrainzGenreId",
        StandardTag::MusicBrainzLabelId(_) => "MusicBrainzLabelId",
        StandardTag::MusicBrainzOriginalAlbumId(_) => "MusicBrainzOriginalAlbumId",
        StandardTag::MusicBrainzOriginalArtistId(_) => "MusicBrainzOriginalArtistId",
        StandardTag::MusicBrainzRecordingId(_) => "MusicBrainzRecordingId",
        StandardTag::MusicBrainzReleaseGroupId(_) => "MusicBrainzReleaseGroupId",
        StandardTag::MusicBrainzReleaseStatus(_) => "MusicBrainzReleaseStatus",
        StandardTag::MusicBrainzReleaseTrackId(_) => "MusicBrainzReleaseTrackId",
        StandardTag::MusicBrainzReleaseType(_) => "MusicBrainzReleaseType",
        StandardTag::MusicBrainzTrackId(_) => "MusicBrainzTrackId",
        StandardTag::MusicBrainzTrmId(_) => "MusicBrainzTrmId",
        StandardTag::MusicBrainzWorkId(_) => "MusicBrainzWorkId",
        StandardTag::Narrator(_) => "Narrator",
        StandardTag::Opus(_) => "Opus",
        StandardTag::OriginalAlbum(_) => "OriginalAlbum",
        StandardTag::OriginalArtist(_) => "OriginalArtist",
        StandardTag::OriginalDate(_) => "OriginalDate",
        StandardTag::OriginalFile(_) => "OriginalFile",
        StandardTag::OriginalLyricist(_) => "OriginalLyricist",
        StandardTag::OriginalWriter(_) => "OriginalWriter",
        StandardTag::OriginalYear(_) => "OriginalYear",
        StandardTag::Owner(_) => "Owner",
        StandardTag::Part(_) => "Part",
        StandardTag::PartNumber(_) => "PartNumber",
        StandardTag::PartTotal(_) => "PartTotal",
        StandardTag::Performer(_) => "Performer",
        StandardTag::PlayCounter(_) => "PlayCounter",
        StandardTag::PodcastCategory(_) => "PodcastCategory",
        StandardTag::PodcastDescription(_) => "PodcastDescription",
        StandardTag::PodcastFlag(_) => "PodcastFlag",
        StandardTag::PodcastKeywords(_) => "PodcastKeywords",
        StandardTag::Producer(_) => "Producer",
        StandardTag::ProductionCopyright(_) => "ProductionCopyright",
        StandardTag::PurchaseDate(_) => "PurchaseDate",
        StandardTag::Rating(_) => "Rating",
        StandardTag::RecordingDate(_) => "RecordingDate",
        StandardTag::RecordingLocation(_) => "RecordingLocation",
        StandardTag::RecordingTime(_) => "RecordingTime",
        StandardTag::ReleaseCountry(_) => "ReleaseCountry",
        StandardTag::ReleaseDate(_) => "ReleaseDate",
        StandardTag::Remixer(_) => "Remixer",
        StandardTag::ReplayGainAlbumGain(_) => "ReplayGainAlbumGain",
        StandardTag::ReplayGainAlbumPeak(_) => "ReplayGainAlbumPeak",
        StandardTag::ReplayGainAlbumRange(_) => "ReplayGainAlbumRange",
        StandardTag::ReplayGainReferenceLoudness(_) => "ReplayGainReferenceLoudness",
        StandardTag::ReplayGainTrackGain(_) => "ReplayGainTrackGain",
        StandardTag::ReplayGainTrackPeak(_) => "ReplayGainTrackPeak",
        StandardTag::ReplayGainTrackRange(_) => "ReplayGainTrackRange",
        StandardTag::Script(_) => "Script",
        StandardTag::Soloist(_) => "Soloist",
        StandardTag::SortAlbum(_) => "SortAlbum",
        StandardTag::SortAlbumArtist(_) => "SortAlbumArtist",
        StandardTag::SortArtist(_) => "SortArtist",
        StandardTag::SortComposer(_) => "SortComposer",
        StandardTag::SortTrackTitle(_) => "SortTrackTitle",
        StandardTag::SortTvShowTitle(_) => "SortTvShowTitle",
        StandardTag::TaggingDate(_) => "TaggingDate",
        StandardTag::TermsOfUse(_) => "TermsOfUse",
        StandardTag::TrackNumber(_) => "TrackNumber",
        StandardTag::TrackSubtitle(_) => "TrackSubtitle",
        StandardTag::TrackTitle(_) => "TrackTitle",
        StandardTag::TrackTotal(_) => "TrackTotal",
        StandardTag::TvEpisode(_) => "TvEpisode",
        StandardTag::TvEpisodeTitle(_) => "TvEpisodeTitle",
        StandardTag::TvNetwork(_) => "TvNetwork",
        StandardTag::TvSeason(_) => "TvSeason",
        StandardTag::TvShowTitle(_) => "TvShowTitle",
        StandardTag::Url(_) => "Url",
        StandardTag::UrlArtist(_) => "UrlArtist",
        StandardTag::UrlCopyright(_) => "UrlCopyright",
        StandardTag::UrlInternetRadio(_) => "UrlInternetRadio",
        StandardTag::UrlLabel(_) => "UrlLabel",
        StandardTag::UrlOfficial(_) => "UrlOfficial",
        StandardTag::UrlPayment(_) => "UrlPayment",
        StandardTag::UrlPodcast(_) => "UrlPodcast",
        StandardTag::UrlPurchase(_) => "UrlPurchase",
        StandardTag::UrlSource(_) => "UrlSource",
        StandardTag::Version(_) => "Version",
        StandardTag::Work(_) => "Work",
        StandardTag::Writer(_) => "Writer",
        _ => {
            // Twitch said that the previous placeholder would get me in trouble for royalties, this should totally solve that :)
            panic!("AT THE DISCOGRAPHY OF MY FAVORITE ARTIST");
        }
    }
    .to_string()
}
