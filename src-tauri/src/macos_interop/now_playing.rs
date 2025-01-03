use objc2::rc::Retained;
use objc2::runtime::{AnyObject, Bool};
use objc2_foundation::{NSNumber, NSString};
use objc2_media_player::{
    MPMediaItemPropertyAlbumArtist, MPMediaItemPropertyAlbumTitle, MPMediaItemPropertyArtist,
    MPMediaItemPropertyArtwork, MPMediaItemPropertyTitle, MPNowPlayingInfoCenter,
    MPNowPlayingInfoPropertyAssetURL, MPNowPlayingInfoPropertyIsLiveStream,
    MPNowPlayingInfoPropertyMediaType, MPNowPlayingPlaybackState,
};

pub fn set_now_playing() {
    unsafe {
        let default = MPNowPlayingInfoCenter::defaultCenter();

        let keys = &[
            // MPNowPlayingInfoPropertyAssetURL,
            // MPNowPlayingInfoPropertyMediaType,
            // MPNowPlayingInfoPropertyIsLiveStream,
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
            // Retained::into_super(Retained::into_super(Retained::into_super(
            //     NSNumber::numberWithInt(1),
            // ))),
            // Retained::into_super(Retained::into_super(Retained::into_super(
            //     NSNumber::numberWithBool(false),
            // ))),
            Retained::into_super(Retained::into_super(NSString::from_str("Title"))),
            Retained::into_super(Retained::into_super(NSString::from_str("Artist"))),
            // Retained::into_super(Retained::into_super(NSString::from_str("Artwork"))),
            Retained::into_super(Retained::into_super(NSString::from_str("Album Artist"))),
            Retained::into_super(Retained::into_super(NSString::from_str("Album Title"))),
        ];
        let dictionary = objc2_foundation::NSDictionary::from_id_slice(keys, owned_objects);
        println!("{:?}", dictionary.get(MPMediaItemPropertyArtist));
        println!("{}", dictionary.len());
        MPNowPlayingInfoCenter::setNowPlayingInfo(&*default, Some(dictionary.as_ref()));
        MPNowPlayingInfoCenter::setPlaybackState(&*default, MPNowPlayingPlaybackState::Playing);
        let shared = objc2_media_player::MPRemoteCommandCenter::sharedCommandCenter();
        let play_handler = block2::StackBlock::new(|_: core::ptr::NonNull<objc2_media_player::MPRemoteCommandEvent>| {
            objc2_media_player::MPRemoteCommandHandlerStatus::Success
        });
        let play_handler = play_handler.copy();
        shared.playCommand().addTargetWithHandler(&play_handler);
    }
}
