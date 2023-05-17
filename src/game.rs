
pub struct Board {
    pub value : i32,
}

pub struct GameState {
    pub board : Board,
}

pub fn start_game() -> GameState {
    let board = Board { value:100 };
    let game :GameState = GameState { board };

    return game;
}
