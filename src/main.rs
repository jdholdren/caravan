use tcod::colors::*;
use tcod::console::*;
use tcod::input::{Key, KeyCode};

// actual size of the window
const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;

const LIMIT_FPS: i32 = 20; // 20 frames-per-second maximum

struct Tcod {
    root: Root,
}

fn main() {
    // Init the window
    tcod::system::set_fps(LIMIT_FPS);
    let root = Root::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Caravan")
        .init();

    let mut tcod = Tcod { root };
    let mut player_pos = (SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2);

    while !tcod.root.window_closed() {
        tcod.root.set_default_foreground(WHITE);
        tcod.root.clear();
        tcod.root
            .put_char(player_pos.0, player_pos.1, '@', BackgroundFlag::None);
        tcod.root.flush();

        // Process user input
        let key = tcod.root.wait_for_keypress(true);

        match key {
            Key {
                code: KeyCode::Up, ..
            } => player_pos.1 -= 1,
            Key {
                code: KeyCode::Down,
                ..
            } => player_pos.1 += 1,
            Key {
                code: KeyCode::Left,
                ..
            } => player_pos.0 -= 1,
            Key {
                code: KeyCode::Right,
                ..
            } => player_pos.0 += 1,
            _ => continue,
        }
    }
}
