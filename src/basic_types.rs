use image::Rgba;

/// Simple Vector2 struct
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Vector2<T>
where
    T: Clone + Copy,
{
    pub x: T,
    pub y: T,
}

impl<T> Vector2<T>
where
    T: Clone + Copy,
{
    pub fn new(x: T, y: T) -> Self {
        Self { x: x, y: y }
    }
}

/// Simple Color struct
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ColorStruct<T>
where
    T: Clone + Copy,
{
    pub r: T,
    pub g: T,
    pub b: T,
    pub a: T,
}

impl<T> From<Rgba<T>> for ColorStruct<T>
where
    T: Clone + Copy,
{
    fn from(value: Rgba<T>) -> Self {
        Self {
            r: value[0],
            g: value[1],
            b: value[2],
            a: value[3],
        }
    }
}

impl<T: Copy> From<[T; 4]> for ColorStruct<T> {
    fn from(value: [T; 4]) -> Self {
        Self {
            r: value[0],
            g: value[1],
            b: value[2],
            a: value[3],
        }
    }
}

impl<T> Into<[T; 4]> for ColorStruct<T>
where
    T: Clone + Copy,
{
    fn into(self) -> [T; 4] {
        [self.r, self.g, self.b, self.a]
    }
}

pub type Color = ColorStruct<u8>;

/// Raw data of the image, in vectors of bytes, ready to be written to a file
pub struct FinishedRawData {
    pub image_data: Vec<u8>,
    pub palette_data: Vec<u8>,
}

pub fn error(error_string: &str) {
    println!("File: {}, Line: {}: {}", file!(), line!(), error_string);
}

pub fn error_out(error_string: &str) {
    panic!("File: {}, Line: {}: {}", file!(), line!(), error_string);
}
