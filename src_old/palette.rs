// Maybe implement a pallette struct
#[derive(Default)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

#[derive(Default)]
pub struct Pallete {
    colors: [Color; 16],
}
