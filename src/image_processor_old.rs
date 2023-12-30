use crate::color_processor::*;
use crate::image_settings::*;
use crate::color_processor;
use std::os::macos::raw;
use std::rc::Rc;
use std::sync::Mutex;
use image::Rgba;
use crate::basic_types::*;

pub struct ProcessedData {
    pub image_data: Vec<u8>,
    pub palette_data: Vec<u8>,
}

pub struct UnprocessedData {
    pub image_data: Vec<u32>,
    pub palette_data: Vec<Color>
}

pub fn process_tile(data: Vec<Color>, settings: TileSettings) -> ProcessedData {
    let expected_length: usize =
        settings.size_per_tile.x as usize * settings.size_per_tile.y as usize;

    if data.len() < expected_length {
        panic!(
            "File: {} Line: {}, data.len() < expected_length",
            file!(),
            line!()
        );
    }

    match settings.bpp {
        BitsPerPixel::Bpp4 | BitsPerPixel::Bpp8 => {
            return process_tile_paletted(&data, &settings);
        }
        BitsPerPixel::Bpp16 => {
            return ProcessedData {
                image_data: Vec::new(),
                palette_data: Vec::new(),
            };
        }
    }
}

pub fn convert_to_paletted(data: &Vec<Color>, settings: &TileSettings) -> UnprocessedData {
    let mut output: UnprocessedData = UnprocessedData { image_data: Vec::new(), palette_data: Vec::new() };

    if let Some(transparent) = settings.transparent_color {
        output.palette_data.push(transparent);
    }

    for i in 0..(settings.size_per_tile.x as usize * settings.size_per_tile.y as usize) {
        let item = data[i];

        if let Some(color_index) = output.palette_data.iter().position(|&color| color == item) {
            output.image_data.push(color_index as u32);
        } else {
            output.palette_data.push(item.clone());
            output.image_data.push((output.palette_data.len() - 1) as u32);
        }
    }

    return output;
}

fn process_tile_paletted(data: &Vec<Color>, settings: &TileSettings) -> ProcessedData {
    let mut unprocessed = convert_to_paletted(data, settings);

    let mut raw_data: ProcessedData = ProcessedData { image_data: Vec::new(), palette_data: Vec::new() };

    if settings.bpp == BitsPerPixel::Bpp4 {
        let mut has_warned_too_many_colors = false;

        for i in 0..unprocessed.image_data.len() / 2 {
            let index = i*2;

            let mut items = (unprocessed.image_data[i], unprocessed.image_data[i+1]);

            if items.0 > 0xF as u32 {
                if !has_warned_too_many_colors {
                    println!("Warning: You have too many colors, please use a mode which supports more colors, or remove some colors");
                    has_warned_too_many_colors = true;
                }

                items.0 = 0;
            }if items.1 > 0xF as u32 {
                if !has_warned_too_many_colors {
                    println!("Warning: You have too many colors, please use a mode which supports more colors, or remove some colors");
                    has_warned_too_many_colors = true;
                }
                
                items.1 = 0;
            }

            raw_data.image_data.push(((items.0) | (items.1 << 4)) as u8);
        }

        if unprocessed.palette_data.len() > 0xF {
            if !has_warned_too_many_colors {
                println!("Warning: You have too many colors, please use a mode which supports more colors, or remove some colors");
                has_warned_too_many_colors = true;
            }

            unprocessed.palette_data.resize(0xF, ColorStruct { r: 0, g: 0, b: 0, a: 0 });
        }

        for item in unprocessed.palette_data {
            raw_data.palette_data.append(&mut color_processor::create_16bit_color(item.r as u16, item.g as u16, item.b as u16, item.a as u16).to_le_bytes().into());
        }
    }

    return raw_data;
}
