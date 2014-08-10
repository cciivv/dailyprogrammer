/*
 * Advanced Langton's Ant
 * http://www.reddit.com/r/dailyprogrammer/comments/2c4ka3/7302014_challenge_173_intermediate_advanced/
 */

use std::fmt::{Formatter, FormatError, Show};
use std::os;

enum Direction {
    North = 0i,
    East,
    South,
    West,
    DirectionMax
}

enum Rotation {
    CCW = -1i,
    CW = 1
}

//All rotations are 90degrees
impl Add<Rotation, Direction> for Direction {
    fn add(&self, _rhs: &Rotation) -> Direction {
        match (*self, *_rhs) {
            (North, CW) => East,
            (East, CW) => South,
            (South, CW) => West,
            (West, CW) => North,
            (North, CCW) => West,
            (West, CCW) => South,
            (South, CCW) => East,
            (East, CCW) => North,
            (_, _) => DirectionMax
        }
    }
}

struct Position {
    x: uint,
    y: uint
}

struct Ant {
    pos: Position,
    dir: Direction
}

impl Ant {
    fn new(position: Position, direction: Direction) -> Ant {
        Ant {pos: position, dir: direction}
    }

    fn move(&mut self, dim: uint) {
        match self.dir {
            North => if self.pos.y < dim - 1 {self.pos.y = self.pos.y + 1;} else {self.pos.y = 0;},
            East => if self.pos.x < dim - 1 {self.pos.x = self.pos.x + 1;} else {self.pos.x = 0;},
            South => if self.pos.y != 0 {self.pos.y = self.pos.y - 1;} else {self.pos.y = dim - 1;},
            West => if self.pos.x != 0 {self.pos.x = self.pos.x - 1;} else {self.pos.x = dim - 1;},
            _ => ()
        }
    }
}

struct Tile {
    color: u8,
    rotation: Rotation
}
impl Show for Tile {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FormatError> {
        write!(f, "{}:{}", self.color, match self.rotation {CW => "CW", CCW => "CCW"})
    }
}
impl Clone for Tile {
    fn clone(&self) -> Tile {
        Tile{ color: self.color, rotation: self.rotation}
    }
}

struct Plane {
    dim: uint,
    board: Vec<Vec<uint>>,
    tiles: Vec<Tile>
}

impl Plane {
    fn new(instruction: &String) -> Plane {
        let mid = 40u;

        let mut tiles = Vec::with_capacity(instruction.len());
        tiles.grow_fn(instruction.len(), |i| {
                Tile {
                   // color: ((255 / (instruction.len()-1))*i) as u8,
                    color : i as u8,
                    rotation: match instruction.as_slice().char_at(i) {
                        'L'|'l' => CCW,
                        'R'|'r' => CW,
                        _ => CW //defaulting to CW for all other characters...
                    }
                }
            });
        let tiles = tiles;

        println!("{}", tiles);

        let mut board: Vec<Vec<uint>> = Vec::with_capacity(2 * mid);
        board.grow_fn(2 * mid, |i| {Vec::with_capacity(2 * mid)});
        for v in board.mut_iter() {
            v.grow_fn(2 * mid, |j| {0u})
        }
        Plane {
            dim: mid * 2,
            board: board,
            tiles: tiles
            }
    }

    fn y_dim(&self) -> uint {self.dim}
    fn x_dim(&self) -> uint {self.dim}
    fn get_tile_rotation(&self, pos: Position) -> Rotation {
        self.tiles[self.board[pos.x][pos.y]].rotation
    }
    fn flip_tile(&mut self, pos: Position) {
        let tile = (self.board[pos.x][pos.y] + 1) % self.tiles.len();
        *self.board.get_mut(pos.x).get_mut(pos.y) = tile;
    }
}
fn csi(f: &mut Formatter) {
    write!(f, "{}[", '\x1B');
}
fn set_bg_color(f: &mut Formatter, offset: u8) {
    csi(f);
    write!(f, "{}m", 40 + offset);
}
fn reset_graphics(f: &mut Formatter) {
    csi(f);
    write!(f, "0m");
}

impl Show for Plane {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FormatError> {
        reset_graphics(f);
        for x_vec in self.board.iter() {
            for value in x_vec.iter() {
                set_bg_color(f, self.tiles[*value].color);
                write!(f, "{} ", value);
            }
            write!(f, "\n");
        }
        reset_graphics(f);
        write!(f, "\n")
    }
}

fn main() {
    let args = os::args();
    assert_eq!(args.len(), 3);

    let iter = from_str(args[2].as_slice()).expect("Not a number");
    let mut plane = Plane::new(&args[1]);
    let mut ant = Ant::new(Position{x:plane.x_dim()/2, y:plane.y_dim()/2}, North);

    for _ in range(0u, iter) {
        ant.dir = ant.dir + plane.get_tile_rotation(ant.pos);
        plane.flip_tile(ant.pos);
        ant.move(plane.dim);
    }
    println!("{}", plane);

}
