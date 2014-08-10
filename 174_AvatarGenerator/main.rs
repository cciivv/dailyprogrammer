/*
 * Avatar Generator:
 * http://www.reddit.com/r/dailyprogrammer/comments/2crqml/8062014_challenge_174_intermediate_forum_avatar/
 *
 */

 /*
    IDEAS

    try to exagerate differences in similar names
        -can't just random hash a name, want similar hash inputs to be similar,
        apply other function to (increase differences | decrease simlarities)

    symettric patterns look better, require less information...

    artistic variations,
        2/3/4 color pallets,
            select color based on segments of "hashed" name being (>=|<=) certain boundaries
        varying degrees of symetry
            selected by specific bits of "hashed" input

 */
use std::os;

fn sel_u8_from(data:uint, offset: uint) -> u8 {
    (data >> offset) as u8
}

struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    fn new(format: &str, data: uint) -> Color {
        match format {
            "RGB888"|"rgb888" => Color{
                                        r: sel_u8_from(data, 16),
                                        g: sel_u8_from(data, 8),
                                        b: sel_u8_from(data, 0)
                                        },
                            _ => Color{r: 0, g: 0, b: 0}
        }
    }
}

trait Selector<T> {
    fn select(&self, data: u32) -> T;
}

struct BoundedMask {
    index: uint,
    bounds: Vec<uint>,
    mask: uint,
    offset: uint,
}
/*
impl Selector for BoundedMask<T> {
    fn select(&self, input: u32, data:Vec<Vec<T>>) -> T {
        assert!(!data.is_empty());
        assert_eq!(self.bounds.len(), data[0].len());
        let i = 0u;
        while i < self.bounds.len() &&
                ((input >> self.offset) & self.mask) < self.bounds[i] {i += 1;}
        data[self.index][i]
    }
}
*/
fn main() {

    let color_pairs: [[Color, ..2], ..4] = [
                [Color::new("rgb888", 0xEDB231u),Color::new("rgb888", 0x4AF0A9u)],
                [Color::new("rgb888", 0x77E761u),Color::new("rgb888", 0xA1DFE1u)],
                [Color::new("rgb888", 0x8E14BAu),Color::new("rgb888", 0x4AF0A9u)],
                [Color::new("rgb888", 0x0D5799u),Color::new("rgb888", 0xFF4C22u)],
                ];

    for group in color_pairs.iter() {
        println!("Color group");
        for color in group.iter() {
            println!("\t{} {} {}", color.r, color.g, color.b);
        }
    }
}
