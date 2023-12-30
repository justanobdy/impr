use crate::basic_types::*;

use clap::builder::TypedValueParser;
use clap::*;
use clap_num::maybe_hex;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Settings {
    // Files to process
    #[arg(short, long, value_delimiter = ',', num_args=1..)]
    pub files: Vec<String>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum BitsPerPixel {
    Bpp4,
    Bpp8,
    Bpp16,
}

impl std::fmt::Display for BitsPerPixel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Bits per pixel: {}", self.to_num())
    }
}

impl BitsPerPixel {
    pub fn from_num(bpp: usize) -> Self {
        match bpp {
            4 => {
                return BitsPerPixel::Bpp4;
            }
            8 => {
                return BitsPerPixel::Bpp8;
            }
            16 => {
                return BitsPerPixel::Bpp16;
            }
            bpp => {
                panic!("Error: {} is not a bit depth!", bpp);
            }
        }
    }

    pub fn to_num(&self) -> usize {
        match self {
            BitsPerPixel::Bpp4 => {
                return 4;
            }
            BitsPerPixel::Bpp8 => {
                return 8;
            }
            BitsPerPixel::Bpp16 => {
                return 16;
            }
        }
    }

    pub fn get_max_palette_length(&self) -> usize {
        match self {
            BitsPerPixel::Bpp4 => 16,
            BitsPerPixel::Bpp8 => 256,
            BitsPerPixel::Bpp16 => {
                panic!("16 bit images don't need a palette (this should never happen, please report this as a bug)");
            }
        }
    }
}

impl From<u8> for BitsPerPixel {
    fn from(value: u8) -> Self {
        match value {
            4 => {
                return BitsPerPixel::Bpp4;
            }
            8 => {
                return BitsPerPixel::Bpp8;
            }
            16 => {
                return BitsPerPixel::Bpp16;
            }
            value => {
                panic!("Error: {} is not a bit depth!", value);
            }
        }
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct TileSettings {
    pub bpp: BitsPerPixel,
    pub size_per_tile: Vector2<u16>,
    pub include_map_data: bool,
    pub transparent_color: Option<Color>,
    pub starting_palette_index: u32,
    pub image_size: Vector2<usize>,
    pub metatile_size: Vector2<usize>,
    pub output_name: Option<String>,
}

impl From<&TileCLI> for TileSettings {
    fn from(value: &TileCLI) -> Self {
        Self {
            bpp: BitsPerPixel::from(value.bpp),
            size_per_tile: Vector2 {
                x: value.size_per_tile[0] as u16,
                y: value.size_per_tile[1] as u16,
            },
            include_map_data: value.include_map_data,
            transparent_color: {
                match value.transparent_color {
                    Some(value) => {
                        // We use big endian because red needs to be the first number
                        Some(Color::from(value.to_be_bytes()))
                    }
                    None => None,
                }
            },
            starting_palette_index: value.starting_palette_index,
            image_size: Vector2 { x: 0, y: 0 },
            metatile_size: Vector2 {
                x: value.size_per_metatile[0],
                y: value.size_per_metatile[1],
            },
            output_name: value.output_name.clone(),
        }
    }
}

#[derive(Clone, Copy, ValueEnum)]
pub enum OutputType {
    Raw,
    ImprF,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct TileCLI {
    /// The files to process
    #[arg(short, long, num_args = 1.., required=true)]
    pub files: Vec<String>,
    /// Bits per pixel that the image will be. 4 & 8 bits per pixel are paletted, while 16 is truecolor mode
    #[arg(
        long,
        default_value_t = 4,
        value_parser = clap::builder::PossibleValuesParser::new(["4", "8", "16"])
            .map(|s| s.parse::<u8>().unwrap()),
    )]
    pub bpp: u8,
    /// The size of each basic tile
    #[arg(long, num_args = 2, default_values_t = [8, 8], value_names = ["x", "y"])]
    pub size_per_tile: Vec<usize>,
    /// The size of each metatile (useful for keeping sprites that are larger than the basic tile size together)
    #[arg(long, num_args = 2, default_values_t = [1, 1])]
    pub size_per_metatile: Vec<usize>,
    /// If the map data (order in which the tiles are placed) should be included with the other data
    #[arg(long, default_value_t = false)]
    pub include_map_data: bool,
    /// The transparent color to use (will be first in the palette)
    #[arg(long, short, value_parser=maybe_hex::<u32>)]
    pub transparent_color: Option<u32>,
    /// The starting index of the palette (keep in mind that if starting_palette_index + num_of_colors > max_colors_for_bitdepth, it will give a warning)
    #[arg(long, default_value_t = 0)]
    pub starting_palette_index: u32,
    /// The output type for the data
    #[arg(long, short, value_enum, default_value_t = OutputType::Raw)]
    pub output_type: OutputType,
    /// Output filename for the data (only works if you have one file)
    #[arg(long)]
    pub output_name: Option<String>,
}

impl Default for TileSettings {
    fn default() -> Self {
        TileSettings {
            bpp: BitsPerPixel::Bpp8,
            size_per_tile: Vector2 { x: 8, y: 8 },
            include_map_data: false,
            transparent_color: Some(ColorStruct {
                r: 0xFF,
                g: 0,
                b: 0xFF,
                a: 0xFF,
            }),
            starting_palette_index: 0,
            image_size: Vector2 { x: 0, y: 0 },
            metatile_size: Vector2 { x: 1, y: 1 },
            output_name: None,
        }
    }
}
