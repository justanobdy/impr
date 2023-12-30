use basic_types::{error_out, Color, Vector2};
use clap::Parser;
use image::{open, DynamicImage};
use image_settings::{OutputType, TileSettings};

pub mod basic_types;
pub mod color_processor;
pub mod image_processor;
pub mod image_settings;
pub mod output_processor;

/// Read an image and get the raw image data from the image
fn get_raw_image_data(file: &str) -> (Vec<Color>, Vector2<usize>) {
    let bufferresult = open(file);
    let buffer: DynamicImage;

    match bufferresult {
        Result::Ok(result) => {
            buffer = result;
        }
        Result::Err(error) => {
            panic!(
                "Unable to open file {}! Reason: {}",
                file,
                error.to_string()
            );
        }
    }

    let size: Vector2<usize> = Vector2 {
        x: buffer.width() as usize,
        y: buffer.height() as usize,
    };

    let buffer = buffer.into_rgb8();
    let buffer_vec = buffer.as_raw();

    let mut raw_data: Vec<Color> = Vec::new();

    for i in 0..buffer_vec.len() / 3 {
        // Red, Green, Blue
        let items: (u8, u8, u8) = (
            buffer_vec[i * 3 + 0],
            buffer_vec[i * 3 + 1],
            buffer_vec[i * 3 + 2],
        );

        raw_data.push(Color {
            r: items.0 as u8,
            g: items.1 as u8,
            b: items.2 as u8,
            a: 255,
        })
    }

    (raw_data, size)
}

fn main() {
    let cli = image_settings::TileCLI::parse();

    let settings = TileSettings::from(&cli);

    if cli.output_name.is_some() && cli.files.len() > 1 {
        panic!("Output Name cannot be used if more than 1 files is being processed!");
    }

    for file in &cli.files {
        // Get the raw image data
        let data = get_raw_image_data(&file);

        // Process the image
        let data = image_processor::process_image(
            data.0,
            TileSettings {
                image_size: data.1,
                ..settings.clone()
            },
        );

        // Output the image
        match cli.output_type {
            OutputType::ImprF => {
                error_out("Error: ImprF is not implemented!");
            }
            OutputType::Raw => {
                if let Some(ref name) = cli.output_name {
                    output_processor::to_raw_binary_files(name.as_str(), &data);
                } else {
                    output_processor::to_raw_binary_files(file.as_str(), &data);
                }
            }
        }
    }
}
