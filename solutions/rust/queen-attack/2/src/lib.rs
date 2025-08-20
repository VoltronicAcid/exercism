#[derive(Debug)]
pub struct ChessPosition {
    rank: i32,
    file: i32,
}

#[derive(Debug)]
pub struct Queen {
    rank: i32,
    file: i32,
    diagonal: i32,
    anti_diagonal: i32,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if (0..8).contains(&rank) && (0..8).contains(&file) {
            return Some(Self { rank, file });
        }

        None
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen {
            rank: position.rank,
            file: position.file,
            diagonal: position.rank - position.file,
            anti_diagonal: position.rank + position.file,
        }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        self.rank == other.rank
            || self.file == other.file
            || self.diagonal == other.diagonal
            || self.anti_diagonal == other.anti_diagonal
    }
}
