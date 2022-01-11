use std::ops::Add;
use std::ops::Index;
use std::vec::Vec;

use once_cell::sync::Lazy;


#[derive(Debug, PartialEq, Copy, Clone)]
enum Upos {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H
}

type File = Upos;
type Rank = Upos;

#[derive(Debug, PartialEq, Copy, Clone)]
struct Pos {
    file: File,
    rank: Rank
}

#[derive(Debug)]
struct Ray {
    orig: Pos,
    dest: Pos,
    between: Vec<Pos>
}

#[derive(Copy, Clone)]
enum Idir {
    STILL,
    UP,
    DOWN,
    UP2,
    DOWN2
}

#[derive(Copy, Clone)]
struct Dir(Idir, Idir);

type Projection = u8;

enum Role {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn
}

enum SlidingRole {
    King,
    Queen,
    Rook,
    Bishop,
    Knight
}

enum PromotingRole {
    Queen,
    Rook,
    Bishop,
    Knight
}

type HasOrigDest = (Pos, Pos);
type HasBlocks = Vec<Pos>;
type HasCapture = Pos;
type HasPromote = PromotingRole;

enum Mobility {
    Slide { 
        role: SlidingRole, 
        orig: HasOrigDest,
        blocks: HasBlocks,
        capture: Option<HasCapture> 
    },
    PawnPush {
        orig: HasOrigDest,
        blocks: HasBlocks,
        promote: Option<HasPromote>
    },
    PawnCapture {
        orig: HasOrigDest,
        capture: HasCapture,
        promote: Option<HasPromote>
    },
    Castle {
        orig: HasOrigDest,
        rook_orig: HasOrigDest,
        blocks: HasBlocks
    },
}


type PosMap<A> = [A; 64];
type SlidingRoleMap<A> = [A; 5];

impl Pos {
    pub fn new(file: File, rank: Rank) -> Pos {
        Pos {
            file,
            rank
        }
    }
}


impl Ray {
    pub fn new(pos: Pos, dir: Dir, projection: Projection)-> Option<Ray> {
        let orig = pos;

        let mut between = Vec::new();

        let mut next = pos;
        for _ in 1..projection {
            if let Some(_next) = next + dir {
                next = _next;
                between.push(next)
            } else {
                return None
            }
        }

        (next + dir).map(|dest|
                        Ray {
                            orig,
                            dest,
                            between
                        }
                       )
    }
}

impl Add<Dir> for Pos {
    type Output = Option<Pos>;

    fn add(self, dir: Dir) -> Option<Pos> {
        (self.file + dir.0)
            .and_then(|file|
                 (self.rank + dir.1)
                 .map(|rank|
                      Pos::new(file, rank)
                     )
                 )
    }
}

impl Add<Idir> for Upos {
    type Output = Option<Upos>;

    fn add(self, dir: Idir) -> Option<Upos> {
        match dir {
            Idir::STILL => Some(self),
            Idir::UP => match self {
                Upos::H => None,
                Upos::G => Some(Upos::H),
                Upos::F => Some(Upos::G),
                Upos::E => Some(Upos::F),
                Upos::D => Some(Upos::E),
                Upos::C => Some(Upos::D),
                Upos::B => Some(Upos::C),
                Upos::A => Some(Upos::B),
            },
            Idir::UP2 => match self {
                Upos::G | Upos::H => None,
                Upos::F => Some(Upos::H),
                Upos::E => Some(Upos::G),
                Upos::D => Some(Upos::E),
                Upos::C => Some(Upos::F),
                Upos::B => Some(Upos::D),
                Upos::A => Some(Upos::C)
            },
            Idir::DOWN => match self {
                Upos::A => None,
                Upos::B => Some(Upos::A),
                Upos::C => Some(Upos::B),
                Upos::D => Some(Upos::C),
                Upos::E => Some(Upos::D),
                Upos::F => Some(Upos::E),
                Upos::G => Some(Upos::F),
                Upos::H => Some(Upos::G),
            },
            Idir::DOWN2 => match self {
                Upos::A | Upos::B => None,
                Upos::C => Some(Upos::A),
                Upos::D => Some(Upos::B),
                Upos::E => Some(Upos::C),
                Upos::F => Some(Upos::D),
                Upos::G => Some(Upos::E),
                Upos::H => Some(Upos::F),
            }

        }
    }

}


impl<A> Index<Pos> for PosMap<A> {
    type Output = A;

    fn index(&self, pos: Pos) -> &Self::Output {
        &self[pos.file as usize * 8 + pos.rank as usize]
    }
}


impl<A> Index<SlidingRole> for SlidingRoleMap<A> {
    type Output = A;

    fn index(&self, role: SlidingRole) -> &Self::Output {
        &self[role as usize]
    }
}


const RAYS: Lazy<SlidingRoleMap<PosMap<Vec<Ray>>>> = Lazy::new(|| {

    let king_dir: Vec<Dir> = 
        vec!(
            Dir(Idir::UP, Idir::STILL),
            Dir(Idir::UP, Idir::UP),
            Dir(Idir::UP, Idir::DOWN),
            Dir(Idir::DOWN, Idir::STILL),
            Dir(Idir::DOWN, Idir::UP),
            Dir(Idir::DOWN, Idir::DOWN),
            Dir(Idir::STILL, Idir::UP),
            Dir(Idir::STILL, Idir::DOWN),
            );


    let rook_dir: Vec<Dir> =
        vec!(
            Dir(Idir::UP, Idir::STILL),
            Dir(Idir::DOWN, Idir::STILL),
            Dir(Idir::STILL, Idir::UP),
            Dir(Idir::STILL, Idir::DOWN)
            );

    let short_projection: Vec<Projection> = vec!(1);

    let long_projection: Vec<Projection> = 
        vec!(1, 2, 3, 4, 5, 6, 7);



    let dirs: SlidingRoleMap<(&Vec<Dir>, &Vec<Projection>)> = 
        ALL_SLIDING.map(|role| match role {
            SlidingRole::Rook => (&rook_dir, &long_projection),
            _ => (&king_dir, &short_projection)
        });


    dirs.map(|dirs| 
             ALL_POS
             .map(|pos|
                  dirs.0
                  .iter()
                  .flat_map(|&dir|
                            dirs.1
                            .iter()
                            .flat_map(move |&projection|
                                      Ray::new(pos, dir, projection)
                                     )
                           )
                  .collect()
                 )
            )
});




const ALL_SLIDING: SlidingRoleMap<SlidingRole> = [
    SlidingRole::King,
    SlidingRole::Queen,
    SlidingRole::Rook,
    SlidingRole::Bishop,
    SlidingRole::Knight,
];


const ALL_POS: PosMap<Pos> = [
    A1, A2, A3, A4, A5, A6, A7, A8,
    B1, B2, B3, B4, B5, B6, B7, B8,
    C1, C2, C3, C4, C5, C6, C7, C8,
    D1, D2, D3, D4, D5, D6, D7, D8,
    E1, E2, E3, E4, E5, E6, E7, E8,
    F1, F2, F3, F4, F5, F6, F7, F8,
    G1, G2, G3, G4, G5, G6, G7, G8,
    H1, H2, H3, H4, H5, H6, H7, H8,
    ];



const A1: Pos = Pos{file: Upos::A, rank: Upos::A };
const A2: Pos = Pos{file: Upos::A, rank: Upos::B };
const A3: Pos = Pos{file: Upos::A, rank: Upos::C };
const A4: Pos = Pos{file: Upos::A, rank: Upos::D };
const A5: Pos = Pos{file: Upos::A, rank: Upos::E };
const A6: Pos = Pos{file: Upos::A, rank: Upos::F };
const A7: Pos = Pos{file: Upos::A, rank: Upos::G };
const A8: Pos = Pos{file: Upos::B, rank: Upos::H };
const B1: Pos = Pos{file: Upos::B, rank: Upos::A };
const B2: Pos = Pos{file: Upos::B, rank: Upos::B };
const B3: Pos = Pos{file: Upos::B, rank: Upos::C };
const B4: Pos = Pos{file: Upos::B, rank: Upos::D };
const B5: Pos = Pos{file: Upos::B, rank: Upos::E };
const B6: Pos = Pos{file: Upos::B, rank: Upos::F };
const B7: Pos = Pos{file: Upos::B, rank: Upos::G };
const B8: Pos = Pos{file: Upos::B, rank: Upos::H };
const C1: Pos = Pos{file: Upos::C, rank: Upos::A };
const C2: Pos = Pos{file: Upos::C, rank: Upos::B };
const C3: Pos = Pos{file: Upos::C, rank: Upos::C };
const C4: Pos = Pos{file: Upos::C, rank: Upos::D };
const C5: Pos = Pos{file: Upos::C, rank: Upos::E };
const C6: Pos = Pos{file: Upos::C, rank: Upos::F };
const C7: Pos = Pos{file: Upos::C, rank: Upos::G };
const C8: Pos = Pos{file: Upos::C, rank: Upos::H };
const D1: Pos = Pos{file: Upos::D, rank: Upos::A };
const D2: Pos = Pos{file: Upos::D, rank: Upos::B };
const D3: Pos = Pos{file: Upos::D, rank: Upos::C };
const D4: Pos = Pos{file: Upos::D, rank: Upos::D };
const D5: Pos = Pos{file: Upos::D, rank: Upos::E };
const D6: Pos = Pos{file: Upos::D, rank: Upos::F };
const D7: Pos = Pos{file: Upos::D, rank: Upos::G };
const D8: Pos = Pos{file: Upos::D, rank: Upos::H };
const E1: Pos = Pos{file: Upos::E, rank: Upos::A };
const E2: Pos = Pos{file: Upos::E, rank: Upos::B };
const E3: Pos = Pos{file: Upos::E, rank: Upos::C };
const E4: Pos = Pos{file: Upos::E, rank: Upos::D };
const E5: Pos = Pos{file: Upos::E, rank: Upos::E };
const E6: Pos = Pos{file: Upos::E, rank: Upos::F };
const E7: Pos = Pos{file: Upos::E, rank: Upos::G };
const E8: Pos = Pos{file: Upos::E, rank: Upos::H };
const F1: Pos = Pos{file: Upos::F, rank: Upos::A };
const F2: Pos = Pos{file: Upos::F, rank: Upos::B };
const F3: Pos = Pos{file: Upos::F, rank: Upos::C };
const F4: Pos = Pos{file: Upos::F, rank: Upos::D };
const F5: Pos = Pos{file: Upos::F, rank: Upos::E };
const F6: Pos = Pos{file: Upos::F, rank: Upos::F };
const F7: Pos = Pos{file: Upos::F, rank: Upos::G };
const F8: Pos = Pos{file: Upos::F, rank: Upos::H };
const G1: Pos = Pos{file: Upos::G, rank: Upos::A };
const G2: Pos = Pos{file: Upos::G, rank: Upos::B };
const G3: Pos = Pos{file: Upos::G, rank: Upos::C };
const G4: Pos = Pos{file: Upos::G, rank: Upos::D };
const G5: Pos = Pos{file: Upos::G, rank: Upos::E };
const G6: Pos = Pos{file: Upos::G, rank: Upos::F };
const G7: Pos = Pos{file: Upos::G, rank: Upos::G };
const G8: Pos = Pos{file: Upos::G, rank: Upos::H };
const H1: Pos = Pos{file: Upos::H, rank: Upos::A };
const H2: Pos = Pos{file: Upos::H, rank: Upos::B };
const H3: Pos = Pos{file: Upos::H, rank: Upos::C };
const H4: Pos = Pos{file: Upos::H, rank: Upos::D };
const H5: Pos = Pos{file: Upos::H, rank: Upos::E };
const H6: Pos = Pos{file: Upos::H, rank: Upos::F };
const H7: Pos = Pos{file: Upos::H, rank: Upos::G };
const H8: Pos = Pos{file: Upos::H, rank: Upos::H };


fn main() {
    println!("Hello, world!");
}



#[cfg(test)]
mod tests {

    use super::*;
#[test]
    fn rays() {

        let res = Ray::new(A1, Dir(Idir::STILL, Idir::STILL), 1);

        assert_eq!(res.map_or(C3, |ray| ray.orig), A1);


        let res = Ray::new(A1, Dir(Idir::UP, Idir::STILL), 2);


        assert_eq!(res.map(|ray| ray.between), Some(vec!(B1)));


        let rays = &RAYS[SlidingRole::Rook][B3];

        let ray = rays
            .iter()
            .find(|ray|
                    ray.dest == H3);

        println!("{:?}", ray);

        assert_eq!(ray.map(|ray| vec_eq(&ray.between, &vec!(C3, D3, E3, F3, G3))), Some(true));

    }

    fn vec_eq<A: PartialEq>(aa: &Vec<A>, bb: &Vec<A>) -> bool {
        let res = aa.iter().zip(bb.iter()).filter(|&(a, b)| a == b).count();
        res == aa.len() && res == bb.len()
    }


#[test]
    fn hashmap() {
        assert_eq!(ALL_POS[Pos::new(Upos::A, Upos::A)],
        Pos::new(Upos::A, Upos::A));
        assert_eq!(ALL_POS[Pos::new(Upos::C, Upos::H)],
        Pos::new(Upos::C, Upos::H));
        assert_eq!(ALL_POS[Pos::new(Upos::H, Upos::H)],
        Pos::new(Upos::H, Upos::H));

    }
}
