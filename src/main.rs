use colored::*;
use image::{Pixel, imageops, GenericImageView};
use std::io;
use std::io::Write;
use clap::{Arg, App};


fn main() {
	let matches = App::new("Terminal Picture Viewer")
	                      .version("0.1")
	                      .author("Author: t0a5ted")
	                      .about("View RGB8 images inside the terminal")
	                      .arg(Arg::with_name("file")
	                      		.value_name("FILE")
	                      		.help("Relative or Absolute File Path to Image")
	                      		.takes_value(true)
	                      		.required(true))
	                      .arg(Arg::with_name("preserveAspectRatio")
	                      		.short("p")
	                      		.help("Preserve Aspect Ratio Of Image"))
	                      .arg(Arg::with_name("width")
	                      		.short("w")
	                      		.takes_value(true)
	                      		.help("Resized Width of Image"))
	                      .arg(Arg::with_name("height")
	                      		.short("h")
	                      		.takes_value(true)
	                      		.help("Resized Height of Image"))
	                      .get_matches();

	let file_path = matches.value_of("file").unwrap();
	let preserve_aspect_ratio = matches.is_present("preserveAspectRatio");
	let resize_height = matches.value_of("height").unwrap_or("NONE");
	let resize_width = matches.value_of("width").unwrap_or("NONE");

	
	let img = image::open(file_path).unwrap();
	let (mut x_max, mut y_max) = img.dimensions();

	if resize_width != "NONE" {
		x_max = resize_width.parse().unwrap();
	}
	if resize_height != "NONE" {
		y_max = resize_height.parse().unwrap();
	}

	let processed_img; 
	if preserve_aspect_ratio {
		processed_img = img.resize(x_max, y_max, imageops::FilterType::Nearest).to_rgb8();
		x_max = processed_img.dimensions().0;
		y_max = processed_img.dimensions().1;
	} else {
		processed_img = img.resize_exact(x_max, y_max, imageops::FilterType::Nearest).to_rgb8();
	};

	
	for y in 0..y_max-1 {
		for x in 0..x_max-1 {
			let pixel = processed_img.get_pixel(x,y);

			let channels = pixel.channels();
			print!("{}", " ".on_truecolor(channels[0], channels[1], channels[2]));
	
			io::stdout().flush().unwrap();
		}
		println!("");
	}

}

