use nokhwa::{
    pixel_format::{RgbAFormat, RgbFormat},
    query,
    utils::{ApiBackend, RequestedFormat, RequestedFormatType},
    CallbackCamera,
};

#[cfg(target_os = "macos")]
use nokhwa::nokhwa_initialize;

fn main() {
    #[cfg(target_os = "macos")]
    nokhwa_initialize(|granted| {
        if !granted {
            eprintln!("Camera access not granted");
            std::process::exit(1);
        }
    });

    let cameras = query(ApiBackend::Auto).unwrap();
    cameras.iter().for_each(|cam| println!("{:?}", cam));

    let format = RequestedFormat::new::<RgbFormat>(RequestedFormatType::AbsoluteHighestFrameRate);

    let first_camera = cameras.first().unwrap();

    let mut threaded = CallbackCamera::new(first_camera.index().clone(), format, |buffer| {
        let image = buffer.decode_image::<RgbAFormat>().unwrap();
        println!("{}x{} {}", image.width(), image.height(), image.len());
    })
    .unwrap();
    threaded.open_stream().unwrap();
    #[allow(clippy::empty_loop)] // keep it running
    loop {
        let frame = threaded.poll_frame().unwrap();
        let image = frame.decode_image::<RgbAFormat>().unwrap();
        println!(
            "{}x{} {} naripoggers",
            image.width(),
            image.height(),
            image.len()
        );
    }
}
