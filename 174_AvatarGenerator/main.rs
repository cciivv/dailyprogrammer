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
use std::rand::{SeedableRng, IsaacRng, Rng};
use std::io::File;

#[inline]
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

struct BoundedMask<T> {
    data: Vec<T>,
    bounds: Vec<uint>,
    mask: uint,
    offset: uint,
}
impl<T: Clone> BoundedMask<T> {
    fn new(rng: &mut IsaacRng, set: &Vec<Vec<T>>, mask_size: uint, offset: uint) -> BoundedMask<T> {

        assert!(!set.is_empty());

        let mask = (1 << mask_size) - 1;

        let data = rng.choose(set.as_slice()).unwrap().clone();

        let bounds = Vec::from_fn(data.len() - 1, |_| {rng.gen_range(0u, mask)});

        BoundedMask { data: data, bounds: bounds, mask: mask, offset: offset}
    }

    fn select(&self, input: uint) -> T {
        assert!(!self.data.is_empty());
        let mut i = 0u;
        while i < self.bounds.len() &&
                ((input >> self.offset) & self.mask) < self.bounds[i] {i += 1;}
        self.data[i].clone()
    }
}

struct Position{
    p: (uint, uint)
}
impl Position {
    fn new(input: (uint,uint)) -> Position {
        Position{ p: input}
    }
    fn x(&self) -> uint {
        self.p.val0()
    }
    fn y(&self) -> uint {
        self.p.val1()
    }
    fn area(&self) -> uint {
        self.p.val0() * self.p.val1()
    }
}

struct SquarePosition {
    dim: uint,
    p: Position,
}
impl SquarePosition {
    fn from_index(dim: uint, index: uint) -> SquarePosition {
        SquarePosition{dim: dim, p:Position::new((index % dim, index / dim))}
    }

    fn from_position(dim: uint, pos: Position) -> SquarePosition{
        SquarePosition{dim: dim, p: pos}
    }

    fn index(&self) -> uint {
        self.p.x() + (self.p.y() * self.dim)
    }

    fn position(&self) -> Position {
        self.p
    }
}

trait IterativePattern {
    fn new(Position) -> Self;
    fn place(&self, uint) -> Position;
    fn step(&self, Position) -> Position;
    fn required_size(&self) -> uint;
    fn iter(&self) -> uint;
}

struct WindmillPattern {
    out: Position,
    iter: uint
}
impl IterativePattern for WindmillPattern {
    fn new(output_dim: Position) -> WindmillPattern {
        assert!(output_dim.x() % 2 == 0);
        assert!(output_dim.y() % 2 == 0);

        WindmillPattern{ out: output_dim, iter: 4u}
    }

    fn place(&self, input: uint) -> Position {
        let x_win = self.out.x() / 2;
        Position::new((input % x_win, input / x_win))
    }

    fn step(&self, input: Position) -> Position {
        let temp = Position::new((input.y(), input.x()));
        let x_bound = (self.out.x() / 2) as int;
        let new_x = (x_bound - 1) + (x_bound - (temp.x() as int));
        Position::new( (new_x as uint, temp.y()))
    }

    fn required_size(&self) -> uint {
        self.out.area() / 4
    }

    fn iter(&self) -> uint {
        self.iter
    }
}

struct VertMirrorPattern {
    out: Position,
    iter: uint
}
impl IterativePattern for VertMirrorPattern {
    fn new(output_dim: Position) -> VertMirrorPattern {
        VertMirrorPattern{ out: output_dim, iter: 2u}
    }

    fn place(&self, input: uint) -> Position {
        let x_win = self.out.x() / 2;
        Position::new((input % x_win, input / x_win))
    }

    fn step(&self, input: Position) -> Position {
        let x_bound = self.out.x() / 2;
        let new_x = (x_bound - 1) + (x_bound - input.x());
        Position::new((new_x, input.y()))
    }

    fn required_size(&self) -> uint {
        self.out.area() / 2
    }

    fn iter(&self) -> uint {
        self.iter
    }
}

struct HorizMirrorPattern {
    out: Position,
    iter: uint
}
impl IterativePattern for HorizMirrorPattern {
    fn new(output_dim: Position) -> HorizMirrorPattern {
        HorizMirrorPattern{ out: output_dim, iter: 2u}
    }

    fn place(&self, input: uint) -> Position {
        Position::new((input % self.out.y(), input / self.out.y()))
    }

    fn step(&self, input: Position) -> Position {
        let y_bound = self.out.y() / 2;
        let new_y = (y_bound - 1) + (y_bound - input.y());
        Position::new((input.x(), new_y))
    }

    fn required_size(&self) -> uint {
        self.out.area() / 2
    }

    fn iter(&self) -> uint {
        self.iter
    }
}

fn tessle(t: &IterativePattern, p: uint, cl: |Position|) {
    let mut pos = t.place(p);
    for _ in range(0u, t.iter()) {
        cl(pos);
        pos = t.step(pos);
    }
}

struct Avatar {
    dim: uint,
    image: Vec<Color>
}

impl Avatar {
    fn new(colors: &Vec<Vec<Color>>, hash: u64, dim: uint, rng: &mut IsaacRng) -> Avatar {
        assert_eq!((dim as f64).log2(),(dim as f64).log2().floor());

        //TODO make input but sqrt of bits in hash
        let SIZE = 8u;

        let windmill: &WindmillPattern = &IterativePattern::new(Position::new((SIZE,SIZE)));
        let vert: &VertMirrorPattern = &IterativePattern::new(Position::new((SIZE,SIZE)));
        //let horiz: &HorizMirrorPattern = &IterativePattern::new(Position::new((SIZE,SIZE)));
        let patterns = vec!(
                windmill as &IterativePattern,
                vert as &IterativePattern,
         //       horiz as &IterativePattern,
        );
        let pattern = rng.choose(patterns.as_slice()).unwrap();

        let points = pattern.required_size();
        //let bits = (points as f64).log2().floor() as uint;
        let bits = 32;
        let color_mask = BoundedMask::new(rng, colors, bits, 0);
        let pallete = Vec::from_fn(points, |_| {
            rng.reseed(vec!(rng.next_u32(), rng.next_u32()).as_slice());
            color_mask.select(rng.next_u32() as uint)
        });

        let mut mini = Vec::from_fn(SIZE*SIZE, |_| {
            Color::new("rgb888", 0)
        });
        for i in range(0u, points) {
            tessle(*pattern, i, |p| {
                let sp: SquarePosition = SquarePosition::from_position(SIZE, p);
                let p = mini.get_mut(sp.index());
                *p = pallete[i].clone();
            });
        }

        //blow up image to size requested
        let expand = dim / SIZE;
        let image = Vec::from_fn(dim*dim, |i| {
            let sp = SquarePosition::from_index(dim, i);
            let mini_sp = SquarePosition::from_position(SIZE,
                            Position::new((sp.position().x() / expand, sp.position().y() / expand)));
            mini[mini_sp.index()].clone()
        });

        Avatar {dim: dim, image: image}
    }
}

fn main() {
    let color_pairs = vec![
        vec![Color::new("rgb888", 0xEDB231u),Color::new("rgb888", 0x4AF0A9u)],
        vec![Color::new("rgb888", 0x77E761u),Color::new("rgb888", 0xA1DFE1u)],
        vec![Color::new("rgb888", 0x8E14BAu),Color::new("rgb888", 0x4AF0A9u)],
        vec![Color::new("rgb888", 0x0D5799u),Color::new("rgb888", 0xFF4C22u)],
        vec![Color::new("rgb888", 0x93648Du),Color::new("rgb888", 0xF16745u)],
        vec![Color::new("rgb888", 0xFFC65Du),Color::new("rgb888", 0x404040u)],
        vec![Color::new("rgb888", 0x666699u),Color::new("rgb888", 0xBEF243u)],
    ];

    let args = os::args();
    let args = args.tail();
    let mut sip = SipState::new_with_keys(0xFEED_DEAD_BEEF_BAFFu64, 0xDEAD_BEE5_DECA_DE00u64);
    for arg in args.iter() {

        arg.hash(&mut sip);
        let hash = sip.result();

        let mut rng: IsaacRng = SeedableRng::from_seed(vec!((hash as u32),((hash >> 32) as u32)).as_slice());
        let avatar = Avatar::new(&color_pairs, hash, 128, &mut rng);
        sip.reset();

        //TODO: put file creation on separate thread
        let mut file = File::create(&Path::new(format!("{}.ppm",arg)));
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
}
