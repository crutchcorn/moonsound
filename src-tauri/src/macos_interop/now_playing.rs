use objc2::runtime::AnyObject;
use objc2_media_player::{MPNowPlayingInfoCenter,
                         MPNowPlayingInfoPropertyAssetURL,
                         MPNowPlayingInfoPropertyMediaType,
                         MPNowPlayingInfoPropertyIsLiveStream,
                         MPMediaItemPropertyTitle,
                         MPMediaItemPropertyArtist,
                         MPMediaItemPropertyArtwork,
                         MPMediaItemPropertyAlbumArtist,
                         MPMediaItemPropertyAlbumTitle};
use objc2_foundation::{ns_string, NSMutableDictionary, NSMutableString, NSString};

pub fn set_now_playing() {
    unsafe {
        let default = MPNowPlayingInfoCenter::defaultCenter();
        MPNowPlayingInfoCenter::setNowPlayingInfo(&*default, {
            let keys = &[
                MPNowPlayingInfoPropertyAssetURL,
                MPNowPlayingInfoPropertyMediaType,
                MPNowPlayingInfoPropertyIsLiveStream,
                MPMediaItemPropertyTitle,
                MPMediaItemPropertyArtist,
                MPMediaItemPropertyArtwork,
                MPMediaItemPropertyAlbumArtist,
                MPMediaItemPropertyAlbumTitle
            ];
            let owned_objects = &[
                NSString::from_str("https://example.com"),
                NSString::from_str("1"),
                NSString::from_str("false"),
                NSString::from_str("Title"),
                NSString::from_str("Artist"),
                NSString::from_str("Artwork"),
                NSString::from_str("Album Artist"),
                NSString::from_str("Album Title")
            ];
            Some(objc2_foundation::NSDictionary::from_id_slice(keys, owned_objects).as_ref())
        });
    }
}
