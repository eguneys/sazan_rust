mod sazan {

    use std::ops;
    use std::vec::Vec;

    use once_cell::sync::Lazy;

    pub struct Epos(u8);

    pub struct Pos(u8);

    struct Ray<'a> {
        orig: Pos,
        dest: Pos,
        between: &'a[Pos]
    }

    type File = Epos;
    type Rank = Epos;

    #[derive(Copy, Clone)]
    struct Edir(i8);

    #[derive(Copy, Clone)]
    struct Dir(Edir, Edir);

    pub struct PosMap<A>([A; 64]);

    pub struct Projection(Vec<u8>);


    pub struct Role(u8);
    #[derive(PartialEq, Eq)]
    pub struct SlidingRole(u8);
    pub struct PromotingRole(u8);

    pub struct SlidingRoleMap<A>([A; 5]);

    struct HasOrigDest(Pos, Pos);

    type HasBlocks = Vec<Pos>;
    type HasCapture = Pos;
    type HasPromote = PromotingRole;

    pub enum Mobility {
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

    const KING: Role = Role(1);
    const QUEEN: Role = Role(2);
    const ROOK: Role = Role(3);
    const BISHOP: Role = Role(4);
    const KNIGHT: Role = Role(5);
    const PAWN: Role = Role(6);

    const SLIDING_KING: SlidingRole = SlidingRole(KING.0);
    const SLIDING_QUEEN: SlidingRole = SlidingRole(QUEEN.0);
    const SLIDING_ROOK: SlidingRole = SlidingRole(ROOK.0);
    const SLIDING_BISHOP: SlidingRole = SlidingRole(BISHOP.0);
    const SLIDING_KNIGHT: SlidingRole = SlidingRole(KNIGHT.0);

    const ALL_SLIDING: [SlidingRole; 5] = [
    SLIDING_KING,
    SLIDING_QUEEN,
    SLIDING_ROOK,
    SLIDING_BISHOP,
    SLIDING_KNIGHT,
    ];

    impl<A> SlidingRoleMap<A> {

        pub fn new<F>(fna: F) -> SlidingRoleMap<A> where F: Fn(SlidingRole) -> A {
            SlidingRoleMap(ALL_SLIDING.map(fna))
        }


        pub fn map<F, B>(&self, fna: F) -> SlidingRoleMap<B> where F: Fn(A) -> B, A: Copy {

            SlidingRoleMap(self.0.map(fna))
        }

    }

    impl<'a> Ray<'a> {

        pub fn rays() -> Vector<Ray<'a>> {
        }
    }

    impl<A> PosMap<A> {
        pub fn new<F>(fna: F) -> PosMap<A>
            where F: Fn(Pos) -> A {
                PosMap(ALL_POS.map(fna))
            }
    }

    impl Pos {

        pub fn new(file: File, rank: Rank) -> Pos {
            Pos(file.0 * 8 + rank.0)
        }

        pub fn file(&self) -> File {
            return Epos((self.0 - self.0 % 8) / 8)
        }

        pub fn rank(&self) -> Rank {
            return Epos(self.0 % 8)
        }
    }

    impl ops::Add<Dir> for Pos {
        type Output = Option<Pos>;



        fn add(self, _rhs: Dir) -> Option<Pos> {
            (self.file() + _rhs.0)
                .and_then(|file|
                     (self.rank() + _rhs.1)
                     .map(|rank| Pos::new(file, rank)))
        }

    }


    impl Epos {
        pub fn new(epos: u8) -> Option<Epos> {
            if epos >= 1 && epos <= 8 {
                Some(Epos(epos))
            } else {
                None
            }
        }
    }

    impl ops::Add<Edir> for Epos {
        type Output = Option<Epos>;

        fn add(self, _rhs: Edir) -> Option<Epos> {
            Epos::new((self.0 as i8 + _rhs.0) as u8)
        }
    }



    const STILL: Edir = Edir(0);
    const UP2: Edir = Edir(2);
    const DOWN2: Edir = Edir(-2);
    const UP: Edir = Edir(1);
    const DOWN: Edir = Edir(-1);
    const UP8: Edir = Edir(8);

    const KNIGHT_DIRS: [Dir; 8] = [
        Dir(UP2, UP),
        Dir(UP2, DOWN),
        Dir(DOWN2, UP),
        Dir(DOWN2, DOWN),
        Dir(UP, UP2),
        Dir(UP, DOWN2),
        Dir(DOWN, UP2),
        Dir(DOWN, DOWN),
    ];

    const BISHOP_DIRS: [Dir; 8] = [
        Dir(UP, DOWN),
        Dir(UP, UP),
        Dir(DOWN, UP),
        Dir(DOWN, DOWN),
        Dir(UP8, UP8),
        Dir(UP8, UP8),
        Dir(UP8, UP8),
        Dir(UP8, UP8),
    ];


    const ROOK_DIRS: [Dir; 8] = [
        Dir(UP, STILL),
        Dir(DOWN, STILL),
        Dir(STILL, UP),
        Dir(STILL, DOWN),
        Dir(UP8, UP8),
        Dir(UP8, UP8),
        Dir(UP8, UP8),
        Dir(UP8, UP8),
    ];

    const QUEEN_DIRS: [Dir; 8] = [
        Dir(UP, DOWN),
        Dir(UP, UP),
        Dir(DOWN, UP),
        Dir(DOWN, DOWN),
        Dir(UP, STILL),
        Dir(DOWN, STILL),
        Dir(STILL, UP),
        Dir(STILL, DOWN),
    ];

    const KING_DIRS: [Dir; 8] = QUEEN_DIRS;

    const DIRS: Lazy<SlidingRoleMap<[Dir; 8]>> = Lazy::new(|| 
        SlidingRoleMap::new(|role| {
            match role {
                SLIDING_KING => KING_DIRS,
                SLIDING_QUEEN => QUEEN_DIRS,
                SLIDING_ROOK => ROOK_DIRS,
                SLIDING_BISHOP => BISHOP_DIRS,
                SLIDING_KNIGHT => KNIGHT_DIRS,
                _ => KING_DIRS
            }
        })
    );

    const RAYS: Lazy<SlidingRoleMap<PosMap<Vec<Ray>>>> = Lazy::new(||
        DIRS
        .map(|dirs| 
             PosMap::new(|pos|
                         dirs
                         .iter()
                         .map(|dir|
                              Ray::rays())
                         .flatten()
                         .collect()))
        );







    pub const A: Epos = Epos(1);
    pub const H: Epos = Epos(8);


    pub const ALL_EPOS: [Epos; 8] = [
        Epos(1),
        Epos(2),
        Epos(3),
        Epos(4),
        Epos(5),
        Epos(6),
        Epos(7),
        Epos(8),
    ];

    pub const ALL_POS: [Pos; 64] = [
        Pos(1 * 8 + 1),
        Pos(1 * 8 + 2),
        Pos(1 * 8 + 3),
        Pos(1 * 8 + 4),
        Pos(1 * 8 + 5),
        Pos(1 * 8 + 6),
        Pos(1 * 8 + 7),
        Pos(1 * 8 + 8),
        Pos(2 * 8 + 1),
        Pos(2 * 8 + 2),
        Pos(2 * 8 + 3),
        Pos(2 * 8 + 4),
        Pos(2 * 8 + 5),
        Pos(2 * 8 + 6),
        Pos(2 * 8 + 7),
        Pos(2 * 8 + 8),
        Pos(3 * 8 + 1),
        Pos(3 * 8 + 2),
        Pos(3 * 8 + 3),
        Pos(3 * 8 + 4),
        Pos(3 * 8 + 5),
        Pos(3 * 8 + 6),
        Pos(3 * 8 + 7),
        Pos(3 * 8 + 8),
        Pos(4 * 8 + 1),
        Pos(4 * 8 + 2),
        Pos(4 * 8 + 3),
        Pos(4 * 8 + 4),
        Pos(4 * 8 + 5),
        Pos(4 * 8 + 6),
        Pos(4 * 8 + 7),
        Pos(4 * 8 + 8),
        Pos(5 * 8 + 1),
        Pos(5 * 8 + 2),
        Pos(5 * 8 + 3),
        Pos(5 * 8 + 4),
        Pos(5 * 8 + 5),
        Pos(5 * 8 + 6),
        Pos(5 * 8 + 7),
        Pos(5 * 8 + 8),
        Pos(6 * 8 + 1),
        Pos(6 * 8 + 2),
        Pos(6 * 8 + 3),
        Pos(6 * 8 + 4),
        Pos(6 * 8 + 5),
        Pos(6 * 8 + 6),
        Pos(6 * 8 + 7),
        Pos(6 * 8 + 8),
        Pos(7 * 8 + 1),
        Pos(7 * 8 + 2),
        Pos(7 * 8 + 3),
        Pos(7 * 8 + 4),
        Pos(7 * 8 + 5),
        Pos(7 * 8 + 6),
        Pos(7 * 8 + 7),
        Pos(7 * 8 + 8),
        Pos(8 * 8 + 1),
        Pos(8 * 8 + 2),
        Pos(8 * 8 + 3),
        Pos(8 * 8 + 4),
        Pos(8 * 8 + 5),
        Pos(8 * 8 + 6),
        Pos(8 * 8 + 7),
        Pos(8 * 8 + 8),
    ];
}

use sazan::*;

fn main() {
    Epos::new(1);
    Pos::new(A, H);
    println!("Hello, world!");



    let a: [u8; 3] = [1, 2, 3];

    asd(&Foo(a), |x| x);

}

struct Foo([u8; 3]);

fn asd<F>(a: &Foo, fna: F) -> Foo where F: Fn(u8) -> u8 {
    Foo(a.0.map(fna))
}


#[cfg(test)]
mod test {

    use std::collections::HashMap;
    use sazan::*;


    #[test]
    fn hashmap() {

        Epos::new(1);
        Pos::new(Epos(1), Epos(2));

    }
}
