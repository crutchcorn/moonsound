#[cfg(target_os = "macos")]
use objc2::rc::Retained;
#[cfg(target_os = "macos")]
use objc2::runtime::AnyObject;
#[cfg(target_os = "macos")]
use objc2_foundation::{NSNumber, NSString};
#[cfg(target_os = "macos")]
use objc2_media_player::{
    MPMediaItemPropertyAlbumArtist, MPMediaItemPropertyAlbumTitle, MPMediaItemPropertyArtist,
    MPMediaItemPropertyArtwork, MPMediaItemPropertyTitle, MPNowPlayingInfoCenter,
    MPNowPlayingInfoPropertyAssetURL, MPNowPlayingInfoPropertyIsLiveStream,
    MPNowPlayingInfoPropertyMediaType, MPNowPlayingPlaybackState,
};

#[cfg(target_os = "macos")]
pub fn setup_handlers(app: &AppHandle, state: State<'static, crate::state::AppData>) {
    unsafe {
        let shared = objc2_media_player::MPRemoteCommandCenter::sharedCommandCenter();
        let state_clone = state.clone();
        let app_clone = app.clone();
        let play_handler = block2::StackBlock::new(
            move |_: core::ptr::NonNull<objc2_media_player::MPRemoteCommandEvent>| {
                crate::music::commands::resume_inner(app_clone.clone(), state_clone.clone());
                objc2_media_player::MPRemoteCommandHandlerStatus::Success
            },
        );
        let state_clone = state.clone();
        let app_clone = app.clone();
        let pause_handler = block2::StackBlock::new(
            move |_: core::ptr::NonNull<objc2_media_player::MPRemoteCommandEvent>| {
                crate::music::commands::pause_inner(app_clone.clone(), state_clone.clone());
                objc2_media_player::MPRemoteCommandHandlerStatus::Success
            },
        );
        let state_clone = state.clone();
        let app_clone = app.clone();
        let toggle_play_pause_handler = block2::StackBlock::new(
            move |_: core::ptr::NonNull<objc2_media_player::MPRemoteCommandEvent>| {
                let app_clone_clone = app_clone.clone();
                let state_clone_clone = state_clone.clone();
                if crate::music::core::is_playing(&state_clone_clone) {
                    crate::music::commands::pause(app_clone_clone, state_clone_clone);
                } else {
                    crate::music::commands::resume(app_clone_clone, state_clone_clone);
                }
                objc2_media_player::MPRemoteCommandHandlerStatus::Success
            },
        );
        shared.playCommand().addTargetWithHandler(&play_handler);
        shared.pauseCommand().addTargetWithHandler(&pause_handler);
        shared
            .togglePlayPauseCommand()
            .addTargetWithHandler(&toggle_play_pause_handler);
    }
}

#[cfg(target_os = "macos")]
pub fn set_now_playing(metadata: crate::music::core::MetadataResult) {
    unsafe {
        let default = MPNowPlayingInfoCenter::defaultCenter();

        let keys = &[
            // MPNowPlayingInfoPropertyAssetURL,
            MPNowPlayingInfoPropertyMediaType,
            MPNowPlayingInfoPropertyIsLiveStream,
            MPMediaItemPropertyTitle,
            MPMediaItemPropertyArtist,
            // MPMediaItemPropertyArtwork,
            MPMediaItemPropertyAlbumArtist,
            MPMediaItemPropertyAlbumTitle,
        ];
        let owned_objects: &[Retained<AnyObject>] = &[
            // Retained::into_super(Retained::into_super(NSString::from_str(
            //     "https://example.com",
            // ))),
            Retained::into_super(Retained::into_super(Retained::into_super(
                NSNumber::numberWithInt(1),
            ))),
            Retained::into_super(Retained::into_super(Retained::into_super(
                NSNumber::numberWithBool(false),
            ))),
            Retained::into_super(Retained::into_super(NSString::from_str(
                &metadata
                    .tags
                    .get("TrackTitle")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Unknown Title")
                    .to_string(),
            ))),
            Retained::into_super(Retained::into_super(NSString::from_str(
                &metadata
                    .tags
                    .get("Artist")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Unknown Artist")
                    .to_string(),
            ))),
            // Retained::into_super(Retained::into_super(NSString::from_str("Artwork"))),
            Retained::into_super(Retained::into_super(NSString::from_str(
                &metadata
                    .tags
                    .get("AlbumArtist")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Unknown Artist")
                    .to_string(),
            ))),
            Retained::into_super(Retained::into_super(NSString::from_str(
                &metadata
                    .tags
                    .get("Album")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Unknown Album")
                    .to_string(),
            ))),
        ];
        let dictionary = objc2_foundation::NSDictionary::from_id_slice(keys, owned_objects);
        MPNowPlayingInfoCenter::setNowPlayingInfo(&*default, Some(dictionary.as_ref()));
        MPNowPlayingInfoCenter::setPlaybackState(&*default, MPNowPlayingPlaybackState::Playing);
    }
}
