use cursive::event::Key;
use cursive::traits::Identifiable;
use cursive::views::{Checkbox, Dialog, EditView, ListView, TextView};

struct PikasayOptions<'a> {
    message: &'a str,
    pokeball: bool,
}

fn input_step(app: &mut cursive::Cursive) {
    app.pop_layer();
    app.add_global_callback(Key::Esc, |s| s.quit());
    app.add_layer(
        Dialog::new()
            .title("pika pikaaa !!!!")
            .content(
                ListView::new()
                    .child("Message", EditView::new().with_name("message"))
                    .child("Return", Checkbox::new().with_name("return")),
            )
            .button("OK", |s| {
                let message = s
                    .call_on_name("message", |t: &mut EditView| t.get_content())
                    .unwrap();
                let returned = s
                    .call_on_name("return", |t: &mut Checkbox| t.is_checked())
                    .unwrap();
                let options = PikasayOptions {
                    message: &message,
                    pokeball: returned,
                };

                result_step(s, &options);
            }),
    )
}

fn result_step(app: &mut cursive::Cursive, options: &PikasayOptions) {
    let message = if options.message.is_empty() {
        "Hi from Simranjeet!"
    } else {
        options.message
    };

    let text = if options.pokeball {
        format!("{}", POKEBALL)
    } else {
        format!(
            " {}
| {} |
 {}
  \\  /
   \\/
{}",
            "-".repeat(message.chars().count() + 2),
            message,
            "-".repeat(message.chars().count() + 2),
            PIKACHU
        )
    };

    app.pop_layer();
    app.add_layer(
        Dialog::around(TextView::new(text))
            .title("Pika...")
            .button("OK", |s| input_step(s)),
    );
}

fn main() {
    let mut app = cursive::default();
    // app.add_layer(Dialog::around(TextView::new(PIKACHU)).button("Ok", |s| s.quit()));
    // app.add_global_callback(Key::Esc, |s| s.quit());

    input_step(&mut app);

    app.run();
}

const PIKACHU: &str = "⣿⣿⣿⣿⣿⡏⠉⠛⢿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⣿
⣿⣿⣿⣿⣿⣿⠀⠀⠀⠈⠛⢿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠿⠛⠉⠁⠀⣿
⣿⣿⣿⣿⣿⣿⣧⡀⠀⠀⠀⠀⠙⠿⠿⠿⠻⠿⠿⠟⠿⠛⠉⠀⠀⠀⠀⠀⣸⣿
⣿⣿⣿⣿⣿⣿⣿⣷⣄⠀⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣴⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⠏⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠠⣴⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⡟⠀⠀⢰⣹⡆⠀⠀⠀⠀⠀⢰⣹⡆⠀⠀⠀⠸⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⠃⠀⠀⠈⠉⠀⠀⠤⠄⠀⠀⠀⠉⠁⠀⠀⠀⠀⢿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⢾⣿⣷⠀⠀⠀⠀⡠⠤⢄⠀⠀⠀⠠⣿⣿⣷⠀⢸⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⡀⠉⠀⠀⠀⠀⠀⢄⠀⢀⠀⠀⠀⠀⠉⠉⠁⠀⠀⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣧⠀⠀⠀⠀⠀⠀⠀⠈⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢹⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⠃⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⣿⣿";

const POKEBALL: &str = "────────▄███████████▄────────
─────▄███▓▓▓▓▓▓▓▓▓▓▓███▄─────
────███▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓███────
───██▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓██───
──██▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓██──
─██▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓██─
██▓▓▓▓▓▓▓▓▓███████▓▓▓▓▓▓▓▓▓██
██▓▓▓▓▓▓▓▓██░░░░░██▓▓▓▓▓▓▓▓██
██▓▓▓▓▓▓▓██░░███░░██▓▓▓▓▓▓▓██
███████████░░███░░███████████
██░░░░░░░██░░███░░██░░░░░░░██
██░░░░░░░░██░░░░░██░░░░░░░░██
██░░░░░░░░░███████░░░░░░░░░██
─██░░░░░░░░░░░░░░░░░░░░░░░██─
──██░░░░░░░░░░░░░░░░░░░░░██──
───██░░░░░░░░░░░░░░░░░░░██───
────███░░░░░░░░░░░░░░░███────
─────▀███░░░░░░░░░░░███▀─────
────────▀███████████▀────────";
