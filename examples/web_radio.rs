extern crate cast_sender;

use macro_rules_attribute::apply;
use smol_macros::main;

use cast_sender::namespace::media::{
    MediaInformationBuilder, MusicTrackMediaMetadataBuilder, StreamType,
};
use cast_sender::{AppId, Error, ImageBuilder, MediaController, Receiver};

#[apply(main!)]
async fn main() -> Result<(), Error> {
    let receiver = Receiver::new();
    receiver.connect("192.168.178.50").await?;

    let app = receiver.launch_app(AppId::DefaultMediaReceiver).await?;
    let media_controller = MediaController::new(app.clone(), receiver.clone())?;

    let metadata = MusicTrackMediaMetadataBuilder::default()
        .title("BBC World Service")
        .images(vec![ImageBuilder::default()
            .url("https://sounds.files.bbci.co.uk/3.7.0/networks/bbc_world_service/colour_default.svg")
            .build()
            .unwrap()])
        .build()
        .unwrap();

    let media_info = MediaInformationBuilder::default()
        .content_id("http://stream.live.vc.bbcmedia.co.uk/bbc_world_service")
        .stream_type(StreamType::Live)
        .content_type("audio/*")
        .metadata(metadata)
        .build()
        .unwrap();

    media_controller.load(media_info).await?;
    Ok(())
}
