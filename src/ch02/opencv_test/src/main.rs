use opencv::{highgui, prelude::*, videoio, Result};

fn main() -> Result<()> {
	let window = "video capture";
	highgui::named_window(window, highgui::WINDOW_AUTOSIZE)?;
    let file_name = "1.mp4";
	let mut cam = videoio::VideoCapture::from_file(&file_name, videoio::CAP_ANY)?; // 0 is the default camera
	let opened = videoio::VideoCapture::is_opened(&cam)?;
	if !opened {
		panic!("Unable to open default camera!");
	}
	loop {
		let mut frame = Mat::default();
		cam.read(&mut frame)?;
		if frame.size()?.width > 0 {
			highgui::imshow(window, &frame)?;
		}
		let key = highgui::wait_key(10)?;
		if key == 113 { // q
			break;
		}
	}
	Ok(())
}