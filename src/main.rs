use cursive::views::{Dialog, TextView};
use cursive::{Cursive};
mod game;

pub fn show_game(siv: &mut cursive::Cursive) {
    siv.add_layer(
        Dialog::around(TextView::new("Hello"))
            .button("Quit", |s| {s.pop_layer();}),
    );
}

fn main() {
    let mut siv = cursive::default();

    siv.add_layer(
        Dialog::around(TextView::new("mineTerm"))
            .title("mineTerm")
            .button("Start Game", show_game)
            .button("Quit", |s| s.quit()),
    );
    let game_state: game::GameState = game::start_game();

    siv.run();
}
