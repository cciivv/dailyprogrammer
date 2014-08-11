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
use std::hash::sip::SipState;
use std::hash::Hash;
use std::rand;
use std::rand::{IsaacRng, Rng};
use std::io::File;

fn sel_u8_from(data:uint, offset: uint) -> u8 {
    (data >> offset) as u8
}

struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Clone for Color {
    fn clone(&self) -> Color {
        Color { r: self.r, g: self.g, b: self.b}
    }
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
    fn select(&self, input: uint) -> T;
}

struct BoundedMask<T> {
    data: Vec<T>,
    bounds: Vec<uint>,
    mask: uint,
    offset: uint,
}
impl<T: Clone> BoundedMask<T> {
    fn new(rng: &mut IsaacRng, set: &Vec<Vec<T>>, mask_size: uint, offset: uint) -> BoundedMask<T> {

        assert!(!set.is_empty());

        let data = rng.choose(set.as_slice()).unwrap().clone();

        let mask = (1 << mask_size) - 1;

        let mut bounds = Vec::new();
        bounds.push(rng.gen_range(1u, mask));

        BoundedMask { data: data, bounds: bounds, mask: mask, offset: offset}
    }
}

impl<T: Clone> Selector<T> for BoundedMask<T> {
    fn select(&self, input: uint) -> T {
        assert!(!self.data.is_empty());
        let mut i = 0u;
        while i < self.bounds.len() &&
                ((input >> self.offset) & self.mask) < self.bounds[i] {i += 1;}
        self.data[i].clone()
    }
}

struct Avatar {
    dim: uint,
    image: Vec<Color>
}

impl Avatar {
    fn new(colors: &Vec<Vec<Color>>, hash: u64, dim: uint, rng: &mut IsaacRng) -> Avatar {
        let color_mask = BoundedMask::new(rng, colors, 4, 0);
        let image = Vec::from_fn(dim*dim, |i| {
                                if i % 3 == 0 {
                                    Color::new("rgb888", 0xFF00FF)
                                } else {
                                    Color::new("rgb888", 0x00FF00)
                                }});
        Avatar {dim: dim, image: image}
    }
}

fn main() {
    let mut color_pairs = Vec::new();
    color_pairs.push(Vec::from_slice(
                [Color::new("rgb888", 0xEDB231u),Color::new("rgb888", 0x4AF0A9u)]
                ));
    color_pairs.push(Vec::from_slice(
                [Color::new("rgb888", 0x77E761u),Color::new("rgb888", 0xA1DFE1u)]
                ));
    color_pairs.push(Vec::from_slice(
                [Color::new("rgb888", 0x8E14BAu),Color::new("rgb888", 0x4AF0A9u)]
                ));
    color_pairs.push(Vec::from_slice(
                [Color::new("rgb888", 0x0D5799u),Color::new("rgb888", 0xFF4C22u)]
                ));

    let args = os::args();
    let args = args.tail();
    let mut sip = SipState::new_with_keys(0xFEED_DEAD_BEEF_BAFFu64, 0xDEAD_BEE5_DECA_DE00u64);
    let mut rng = IsaacRng::new_unseeded();
    for arg in args.iter() {
        arg.hash(&mut sip);
        let hash = sip.result();
        let avatar = Avatar::new(&color_pairs, hash, 64, &mut rng);
        let mut file = File::create(&Path::new(format!("{}.txt",arg)));
        write!(file, "P6\n{dim} {dim}\n{maxval}\n", dim = avatar.dim, maxval = 255u);
        assert_eq!(avatar.image.len(), avatar.dim * avatar.dim);
        for c in avatar.image.iter() {
        //*
            file.write_u8(c.r);
            file.write_u8(c.g);
            file.write_u8(c.b);
        //*/
        /* Glitch'd write
            write!(file, "{}{}{}", c.r as char, c.g as char, c.b as char);
        */
        }
        println!("made avatar for {}", arg);
    }

    for group in color_pairs.iter() {
        println!("Color group");
        for color in group.iter() {
            println!("\t{} {} {}", color.r, color.g, color.b);
        }
    }
}
