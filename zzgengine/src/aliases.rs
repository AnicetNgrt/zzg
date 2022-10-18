pub enum YesOrNo<Reason> {
    Yes,
    No(Reason)
}

pub enum Interactible {
    Pawn(usize),
    Player(usize),
    Tile(usize)
}