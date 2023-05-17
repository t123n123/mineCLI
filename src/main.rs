use cursive::views::{Dialog, TextView};

fn main() {
    let mut siv = cursive::default();

    siv.add_layer(
        Dialog::around(TextView::new("mineTerm"))
            .title("mineTerm")
            .button("Quit", |s| s.quit()),
    );

    siv.run();

    println!("Hello, world!");
}
