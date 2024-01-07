use nokhwa::{
    pixel_format::{RgbAFormat, RgbFormat},
    query,
    utils::{ApiBackend, RequestedFormat, RequestedFormatType},
    CallbackCamera,
};

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
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

    let cameras: Vec<nokhwa::utils::CameraInfo> =
        query(ApiBackend::Auto).map_err(|e| -> Box<dyn std::error::Error> { Box::new(e) })?;
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

    // window shit down here

    let event_loop = EventLoop::new().map_err(|e| -> Box<dyn std::error::Error> { Box::new(e) })?;

    event_loop.set_control_flow(ControlFlow::Poll);

    let window = WindowBuilder::new()
        .with_title("Persephone VR Tracking")
        .with_inner_size(winit::dpi::LogicalSize::new(128.0, 128.0))
        .build(&event_loop)
        .unwrap();

    event_loop
        .run(move |event, elwt| {
            println!("{event:?}");

            match event {
                Event::WindowEvent { event, window_id } if window_id == window.id() => {
                    match event {
                        WindowEvent::CloseRequested => elwt.exit(),
                        WindowEvent::RedrawRequested => {
                            // Notify the windowing system that we'll be presenting to the window.
                            window.pre_present_notify();
                        }
                        _ => (),
                    }
                }
                Event::AboutToWait => {
                    match threaded.poll_frame() {
                        Ok(frame) => match frame.decode_image::<RgbAFormat>() {
                            Ok(image) => {
                                println!("{}x{} {}", image.width(), image.height(), image.len())
                            }
                            Err(e) => eprintln!("Error decoding image: {}", e),
                        },
                        Err(e) => {
                            eprintln!("Error polling frame: {}", e);
                        }
                    }

                    window.request_redraw();
                }

                _ => (),
            }
        })
        .unwrap();
    Ok(())
}
