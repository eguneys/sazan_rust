mod sazan {

    use std::ops;
    use std::vec::Vec;
    use once_cell::sync::Lazy;

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

    struct Pos {
        file: File,
        rank: Rank
    }

    struct Ray<'a> {
        orig: Pos,
        dest: Pos,
        between: &'a[Pos]
    }


    struct Ipos(bool, Upos);
    struct Dir(Ipos, Ipos);

    type PosMap<A> = [A; 64];

    type Projection = Upos;


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

    const ALL_SLIDING: [SlidingRole; 5] = [
        SlidingRole::King,
        SlidingRole::Queen,
        SlidingRole::Rook,
        SlidingRole::Bishop,
        SlidingRole::Knight,
    ];


}

use sazan::*;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {

    #[test]
    fn hashmap() {


    }
}
