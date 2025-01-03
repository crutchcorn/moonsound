use objc2::rc::Retained;
use objc2::runtime::AnyObject;
use objc2_foundation::{NSNumber, NSString};
use objc2_media_player::{
    MPMediaItemPropertyAlbumArtist, MPMediaItemPropertyAlbumTitle, MPMediaItemPropertyArtist,
    MPMediaItemPropertyArtwork, MPMediaItemPropertyTitle, MPNowPlayingInfoCenter,
    MPNowPlayingInfoPropertyAssetURL, MPNowPlayingInfoPropertyIsLiveStream,
    MPNowPlayingInfoPropertyMediaType, MPNowPlayingPlaybackState,
};

pub fn setup_handlers(state: crate::state::AppData) {
    unsafe {
        let shared = objc2_media_player::MPRemoteCommandCenter::sharedCommandCenter();
        let state_clone1 = state.clone();
        let play_handler = block2::StackBlock::new(
            move |_: core::ptr::NonNull<objc2_media_player::MPRemoteCommandEvent>| {
                crate::music::core::resume(&state_clone1);
                objc2_media_player::MPRemoteCommandHandlerStatus::Success
            },
        );
        let state_clone2 = state.clone();
        let pause_handler = block2::StackBlock::new(
            move |_: core::ptr::NonNull<objc2_media_player::MPRemoteCommandEvent>| {
                crate::music::core::pause(&state_clone2);
                objc2_media_player::MPRemoteCommandHandlerStatus::Success
            },
        );
        let state_clone3 = state.clone();
        let toggle_play_pause_handler = block2::StackBlock::new(
            move |_: core::ptr::NonNull<objc2_media_player::MPRemoteCommandEvent>| {
                if crate::music::core::is_playing(&state_clone3) {
                    crate::music::core::pause(&state_clone3);
                } else {
                    crate::music::core::resume(&state_clone3);
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

pub fn set_now_playing() {
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
            Retained::into_super(Retained::into_super(NSString::from_str("Title"))),
            Retained::into_super(Retained::into_super(NSString::from_str("Artist"))),
            // Retained::into_super(Retained::into_super(NSString::from_str("Artwork"))),
            Retained::into_super(Retained::into_super(NSString::from_str("Album Artist"))),
            Retained::into_super(Retained::into_super(NSString::from_str("Album Title"))),
        ];
        let dictionary = objc2_foundation::NSDictionary::from_id_slice(keys, owned_objects);
        MPNowPlayingInfoCenter::setNowPlayingInfo(&*default, Some(dictionary.as_ref()));
        MPNowPlayingInfoCenter::setPlaybackState(&*default, MPNowPlayingPlaybackState::Playing);
    }
}
