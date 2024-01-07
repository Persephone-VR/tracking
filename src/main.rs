use nokhwa::{
    pixel_format::{RgbAFormat, RgbFormat},
    query,
    utils::{ApiBackend, RequestedFormat, RequestedFormatType},
    CallbackCamera,
};

#[cfg(target_os = "macos")]
use nokhwa::nokhwa_initialize;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(target_os = "macos")]
    nokhwa_initialize(|granted| {
        if !granted {
            eprintln!("Camera access not granted");
            std::process::exit(1);
        }
    });

    let cameras: Vec<nokhwa::utils::CameraInfo> = query(ApiBackend::Auto)?;
    if cameras.is_empty() {
        eprintln!("No cameras found");
        return Ok(());
    } else {
        println!("Found {} cameras", cameras.len());
        cameras
            .iter()
            .for_each(|cam: &nokhwa::utils::CameraInfo| println!("{:?}", cam));
    }

    let format: RequestedFormat<'_> =
        RequestedFormat::new::<RgbFormat>(RequestedFormatType::AbsoluteHighestFrameRate);
    let first_camera: &nokhwa::utils::CameraInfo = &cameras[0];

    let mut threaded: CallbackCamera = CallbackCamera::new(
        first_camera.index().clone(),
        format,
        |buffer: nokhwa::Buffer| {
            let image: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> =
                buffer.decode_image::<RgbAFormat>().unwrap();
            println!("{}x{} {}", image.width(), image.height(), image.len());
        },
    )
    .unwrap();
    threaded.open_stream().unwrap();

    loop {
        match threaded.poll_frame() {
            Ok(frame) => match frame.decode_image::<RgbAFormat>() {
                Ok(image) => println!("{}x{} {}", image.width(), image.height(), image.len()),
                Err(e) => eprintln!("Error decoding image: {}", e),
            },
            Err(e) => {
                eprintln!("Error polling frame: {}", e);
                return Ok(()); // exit
            }
        }
    }
}
