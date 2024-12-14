use ori::prelude::*;
use ori_font_awesome as fa;

fn ui() -> impl View {
    let buttons = hstack![
        button(fa::icon("repeat")),
        button(fa::icon("bug")),
        button(fa::icon("font-awesome")),
    ]
    .gap(10.0);

    on_event(pad(50.0, center(buttons)), |cx, _, event| {
        if let Event::KeyPressed(e) = event {
            if e.is_key(Key::Escape) {
                cx.cmd(AppCommand::Quit);
                return true;
            }
        }

        false
    })
}

fn main() {
    ori::log::install().unwrap();

    let window = Window::new().title("Icon Example").fit_content();

    ori::run_simple(window, ui).unwrap();
}
