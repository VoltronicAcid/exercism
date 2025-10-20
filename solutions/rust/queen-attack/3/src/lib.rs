#[derive(Debug, Clone, Copy)]
pub struct ChessPosition {
    rank: i8,
    file: i8,
}

#[derive(Debug)]
pub struct Queen {
    position: ChessPosition,
}

impl ChessPosition {
    pub fn new(rank: i8, file: i8) -> Option<Self> {
        match (rank, file) {
            (0..8, 0..8) => Some(ChessPosition { rank, file }),
            _ => None,
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen { position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        let (
            ChessPosition {
                rank: rank1,
                file: file1,
            },
            ChessPosition {
                rank: rank2,
                file: file2,
            },
        ) = (self.position, other.position);

        rank1 == rank2
            || file1 == file2
            || rank1 + file1 == rank2 + file2
            || rank1 - file1 == rank2 - file2
    }
}
