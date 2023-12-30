use crate::basic_types::*;
use crate::color_processor;
use crate::color_processor::create_16bit_color_argb16;
use crate::image_settings::*;
use itertools::Itertools;

pub fn process_image(data: Vec<Color>, settings: TileSettings) -> FinishedRawData {
    let expected_length: usize =
        settings.size_per_tile.x as usize * settings.size_per_tile.y as usize;

    if data.len() < expected_length {
        panic!(
            "File: {} Line: {}, data.len() < expected_length",
            file!(),
            line!()
        );
    }

    // Split the image into tiles
    let tiled = tile(
        &data,
        settings.image_size,
        Vector2 {
            x: settings.size_per_tile.x as usize,
            y: settings.size_per_tile.y as usize,
        },
    );

    // Group metatiles together
    let metatiled = tile_sequentially(
        &tiled,
        Vector2 {
            x: settings.image_size.x / settings.size_per_tile.x as usize,
            y: settings.image_size.y / settings.size_per_tile.y as usize,
        },
        Vector2 {
            x: settings.metatile_size.x as usize,
            y: settings.metatile_size.y as usize,
        },
    );

    // This is the raw byte arrays that will be written to the file
    let mut final_data: FinishedRawData = FinishedRawData {
        image_data: Vec::new(),
        palette_data: Vec::new(),
    };

    match settings.bpp {
        BitsPerPixel::Bpp4 | BitsPerPixel::Bpp8 => {
            // Get the palete of the entire image
            let palette = get_image_palette(&data, &settings);

            // Process all the tiles and add it to final_data
            for tile in metatiled {
                let mut paletted = process_tile_paletted(&tile, &settings, &palette);

                final_data.image_data.append(&mut paletted);
            }

            // Convert all the colors into palette into 16 bit color
            for item in palette {
                final_data.palette_data.append(
                    &mut color_processor::create_16bit_color(
                        item.r as u16,
                        item.g as u16,
                        item.b as u16,
                        item.a as u16,
                    )
                    .to_le_bytes()
                    .to_vec(),
                );
            }

            return final_data;
        }
        BitsPerPixel::Bpp16 => {
            // TODO: allow splitting bitmap into multiple images

            for color in data {
                final_data.image_data.append(
                    &mut create_16bit_color_argb16(
                        color.r as u16,
                        color.g as u16,
                        color.b as u16,
                        color.a as u16,
                    )
                    .to_le_bytes()
                    .to_vec(),
                );
            }

            return final_data;
        }
    }
}

/// Tile data together, and put them each into a Vec<T>
fn tile<T: Clone>(
    data: &Vec<T>,
    total_size: Vector2<usize>,
    tile_size: Vector2<usize>,
) -> Vec<Vec<T>> {
    // The needed length of the vector for the function to work
    let expected_length = total_size.x * total_size.y;

    assert!(
        data.len() >= expected_length,
        "Error: Data is too short! (Is your image the correct size?)"
    );
    assert!(
        total_size.x % tile_size.x == 0,
        "Error: The x size of the image must be divisable by the size of the tile!"
    );
    assert!(
        total_size.y % tile_size.y == 0,
        "Error: The x size of the image must be divisable by the size of the tile!"
    );

    let total_tiles: Vector2<usize> = Vector2 {
        x: total_size.x / tile_size.x,
        y: total_size.y / tile_size.y,
    };
    let total_items_per_line: Vector2<usize> = Vector2 {
        x: total_tiles.x * tile_size.x,
        y: total_tiles.y * tile_size.y,
    };
    let total_items_per_tile: usize = tile_size.x * tile_size.y;

    // The vector which will contain all the tiels
    let mut new_vec: Vec<Vec<T>> = Vec::new();

    for y in 0..total_tiles.y {
        for x in 0..total_tiles.x {
            // A new tile
            let mut new_tile: Vec<T> = Vec::new();

            // The position of the new tile
            let tile_position: Vector2<usize> = Vector2 { x, y };

            // Go through each item (pixel) of the tile
            for y in 0..tile_size.y {
                // Get the index of the y position of thet tile
                let index = (tile_position.x * tile_size.x)
                    + ((tile_position.y * (total_items_per_tile * total_tiles.x))
                        + (y * total_items_per_line.x));

                // Read each pixel of the y position
                for x in 0..tile_size.x {
                    new_tile.push(data[index + x].clone());
                }
            }

            new_vec.push(new_tile);
        }
    }

    new_vec
}

/// Tile the image, but instead of returning Vec<Vec<T>>, return Vec<T>, where each vector is appended to the end
fn tile_sequentially<T: Clone>(
    data: &Vec<T>,
    total_size: Vector2<usize>,
    tile_size: Vector2<usize>,
) -> Vec<T> {
    tile(data, total_size, tile_size)
        .into_iter()
        .flat_map(|item| item)
        .collect_vec()
}
/// Get the palette of an image by taking the image, then deduping it, and making sure the palette fits the parameters given
fn get_image_palette(data: &Vec<Color>, settings: &TileSettings) -> Vec<Color> {
    let mut palette: Vec<Color> = Vec::new();
    // The transperent color needs to be first
    if let Some(color) = settings.transparent_color {
        palette.push(color);
    }
    palette.append(&mut data.clone());
    let palette: Vec<Color> = palette.into_iter().unique().collect();

    if palette.len() > settings.bpp.get_max_palette_length() {
        panic!("Warning: palette length is {}, which is longer than {} colors, the max for your selected bit-depth. Please choose a higher bit-depth, or remove some colors from your image.", palette.len(), settings.bpp.get_max_palette_length());
    }

    palette
}

// Convert to a palette tile
pub fn convert_to_paletted(
    data: &Vec<Color>,
    settings: &TileSettings,
    palette: &Vec<Color>,
) -> Vec<u32> {
    let mut output: Vec<u32> = Vec::new();

    for i in 0..(settings.size_per_tile.x as usize * settings.size_per_tile.y as usize) {
        let item = data[i];

        if let Some(color_index) = palette.iter().position(|&color| color == item) {
            output.push(color_index as u32);
        } else {
            panic!("Error: Color does not exist in palette! (this should never happen, please report this as a bug)");
        }
    }

    // Start index at starting_palette_index instead of 0
    if settings.starting_palette_index != 0 {
        for item in output.iter_mut() {
            *item += settings.starting_palette_index;

            if *item > settings.bpp.get_max_palette_length() as u32 {
                panic!("starting palette index is set too high, please set it lower!");
            }
        }
    }

    return output;
}

/// This function takes the image data, and the palette that image uses, and converts it to a raw byte array, ready to be read.
fn process_tile_paletted(
    data: &Vec<Color>,
    settings: &TileSettings,
    palette: &Vec<Color>,
) -> Vec<u8> {
    let unprocessed = convert_to_paletted(data, settings, &palette);

    let mut raw_data: FinishedRawData = FinishedRawData {
        image_data: Vec::new(),
        palette_data: Vec::new(),
    };

    // This is used to make sure we only warn that you have two many colors only once
    // TODO: we should have a function that handles this
    let mut has_warned_too_many_colors = false;

    if settings.bpp == BitsPerPixel::Bpp4 {
        // We go 2 items at a time
        for i in 0..unprocessed.len() / 2 {
            let index = i * 2;

            let mut items = (unprocessed[index], unprocessed[index + 1]);

            // Make sure both items fit within the 16 color limit of 4 bit images
            if items.0 > 0xF as u32 {
                if !has_warned_too_many_colors {
                    println!("Line: {}, Warning: You have too many colors, please use a mode which supports more colors, or remove some colors", line!());
                    has_warned_too_many_colors = true;
                }

                items.0 = 0;
            }
            if items.1 > 0xF as u32 {
                if !has_warned_too_many_colors {
                    println!("Line: {}, Warning: You have too many colors, please use a mode which supports more colors, or remove some colors", line!());
                    has_warned_too_many_colors = true;
                }

                items.1 = 0;
            }

            raw_data.image_data.push(((items.0) | (items.1 << 4)) as u8);
        }
    }
    if settings.bpp == BitsPerPixel::Bpp8 {
        raw_data.image_data = unprocessed.into_iter().map(|item| {
            // Make sure the item fits within the 256 color limit for 8 bit images
            if item > 0xFF {
                if !has_warned_too_many_colors {
                    println!("Warning: You have too many colors, please use a mode which supports more colors, or remove some colors");
                    has_warned_too_many_colors = true;
                }

                return 0u8;
            }

            item as u8
        }).collect();
    }

    //for item in palette {
    //    raw_data.palette_data.append(&mut color_processor::create_16bit_color(item.r as u16, item.g as u16, item.b as u16, item.a as u16).to_le_bytes().into());
    //}

    return raw_data.image_data;
}
