

mod sazan {

    use std::ops::Index;
    use std::vec::Vec;

    use once_cell::sync::Lazy;


    #[derive(Debug, PartialEq)]
    pub enum Upos {
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

    #[derive(Debug, PartialEq)]
    pub struct Pos {
        file: File,
        rank: Rank
    }

    struct Ray<'a> {
        orig: Pos,
        dest: Pos,
        between: &'a[Pos]
    }


    enum Idir {
        STILL,
        UP,
        DOWN,
        UP2,
        DOWN2
    }
    struct Dir(Idir, Idir);

    type Projection = u8;

    pub enum Role {
        King,
        Queen,
        Rook,
        Bishop,
        Knight,
        Pawn
    }

    pub enum SlidingRole {
        King,
        Queen,
        Rook,
        Bishop,
        Knight
    }

    pub enum PromotingRole {
        Queen,
        Rook,
        Bishop,
        Knight
    }

    type HasOrigDest = (Pos, Pos);
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


    impl<'a> Ray<'a> {
        pub fn rays(pos: &Pos, dir: &Dir, projection: &Projection)-> Vec<Ray<'a>> {
            let res = Vec::new();

            res
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

        let KING_DIR: Vec<Dir> = 
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

        let SHORT_PROJECTION: Vec<Projection> = vec!(1);

        let LONG_PROJECTION: Vec<Projection> = 
            vec!(1, 2, 3, 4, 5, 6, 7);



        let DIRS: SlidingRoleMap<(&Vec<Dir>, &Vec<Projection>)> = 
            ALL_SLIDING.map(|role| match role {
                _ => (&KING_DIR, &SHORT_PROJECTION)
            });


        DIRS.map(|dirs| 
                 ALL_POS
                 .map(|pos|
                      dirs.0
                      .iter()
                      .flat_map(|dir|

                               dirs.1
                               .iter()
                               .flat_map(|projection|
                                    Ray::rays(&pos, dir, projection)
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

    pub const ALL_POS: PosMap<Pos> = [
        Pos{file: Upos::A, rank: Upos::A },
        Pos{file: Upos::A, rank: Upos::B },
        Pos{file: Upos::A, rank: Upos::C },
        Pos{file: Upos::A, rank: Upos::D },
        Pos{file: Upos::A, rank: Upos::E },
        Pos{file: Upos::A, rank: Upos::F },
        Pos{file: Upos::A, rank: Upos::G },
        Pos{file: Upos::B, rank: Upos::H },
        Pos{file: Upos::B, rank: Upos::A },
        Pos{file: Upos::B, rank: Upos::B },
        Pos{file: Upos::B, rank: Upos::C },
        Pos{file: Upos::B, rank: Upos::D },
        Pos{file: Upos::B, rank: Upos::E },
        Pos{file: Upos::B, rank: Upos::F },
        Pos{file: Upos::B, rank: Upos::G },
        Pos{file: Upos::B, rank: Upos::H },
        Pos{file: Upos::C, rank: Upos::A },
        Pos{file: Upos::C, rank: Upos::B },
        Pos{file: Upos::C, rank: Upos::C },
        Pos{file: Upos::C, rank: Upos::D },
        Pos{file: Upos::C, rank: Upos::E },
        Pos{file: Upos::C, rank: Upos::F },
        Pos{file: Upos::C, rank: Upos::G },
        Pos{file: Upos::C, rank: Upos::H },
        Pos{file: Upos::D, rank: Upos::A },
        Pos{file: Upos::D, rank: Upos::B },
        Pos{file: Upos::D, rank: Upos::C },
        Pos{file: Upos::D, rank: Upos::D },
        Pos{file: Upos::D, rank: Upos::E },
        Pos{file: Upos::D, rank: Upos::F },
        Pos{file: Upos::D, rank: Upos::G },
        Pos{file: Upos::D, rank: Upos::H },
        Pos{file: Upos::E, rank: Upos::A },
        Pos{file: Upos::E, rank: Upos::B },
        Pos{file: Upos::E, rank: Upos::C },
        Pos{file: Upos::E, rank: Upos::D },
        Pos{file: Upos::E, rank: Upos::E },
        Pos{file: Upos::E, rank: Upos::F },
        Pos{file: Upos::E, rank: Upos::G },
        Pos{file: Upos::E, rank: Upos::H },
        Pos{file: Upos::F, rank: Upos::A },
        Pos{file: Upos::F, rank: Upos::B },
        Pos{file: Upos::F, rank: Upos::C },
        Pos{file: Upos::F, rank: Upos::D },
        Pos{file: Upos::F, rank: Upos::E },
        Pos{file: Upos::F, rank: Upos::F },
        Pos{file: Upos::F, rank: Upos::G },
        Pos{file: Upos::F, rank: Upos::H },
        Pos{file: Upos::G, rank: Upos::A },
        Pos{file: Upos::G, rank: Upos::B },
        Pos{file: Upos::G, rank: Upos::C },
        Pos{file: Upos::G, rank: Upos::D },
        Pos{file: Upos::G, rank: Upos::E },
        Pos{file: Upos::G, rank: Upos::F },
        Pos{file: Upos::G, rank: Upos::G },
        Pos{file: Upos::G, rank: Upos::H },
        Pos{file: Upos::H, rank: Upos::A },
        Pos{file: Upos::H, rank: Upos::B },
        Pos{file: Upos::H, rank: Upos::C },
        Pos{file: Upos::H, rank: Upos::D },
        Pos{file: Upos::H, rank: Upos::E },
        Pos{file: Upos::H, rank: Upos::F },
        Pos{file: Upos::H, rank: Upos::G },
        Pos{file: Upos::H, rank: Upos::H },
        ];

}


fn main() {
    println!("Hello, world!");

}

#[cfg(test)]
mod test {

    use crate::sazan::*;
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
