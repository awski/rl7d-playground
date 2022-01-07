use tcod::Console;
use tcod::input::{Key, KeyCode};

const WINDOW_WIDTH: i32 = 80;
const WINDOW_HEIGHT: i32 = 50;
const FPS_LIMIT: u32 = 20;

struct Tcod {
    root: tcod::console::Root
}

fn main() {
    let mut ctx = Tcod {
        root: tcod::console::Root::initializer()
            .font("arial10x10.png", tcod::FontLayout::Tcod)
            .font_type(tcod::FontType::Greyscale)
            .size(WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32)
            .title("rl7d playground")
            .init()
    };
    tcod::system::set_fps(FPS_LIMIT as i32);

    let mut player_x: i32 = WINDOW_WIDTH / 2;
    let mut player_y: i32 = WINDOW_HEIGHT / 2;

    while !ctx.root.window_closed() {
        ctx.root.set_default_foreground(tcod::colors::WHITE);
        ctx.root.clear();
        ctx.root.put_char(player_x, player_y, '@', tcod::console::BackgroundFlag::None);
        ctx.root.flush();
        let exit = handle_keys(&mut ctx, &mut player_x, &mut player_y);
        if exit {
            return;
        }
    }
}

fn handle_keys(ctx: &mut Tcod, player_x: &mut i32, player_y: &mut i32) -> bool {
    match ctx.root.wait_for_keypress(true) {
        Key { code: KeyCode::Up, .. } => { *player_y -= 1 }
        Key { code: KeyCode::Down, .. } => { *player_y += 1 }
        Key { code: KeyCode::Left, .. } => { *player_x -= 1 }
        Key { code: KeyCode::Right, .. } => { *player_x += 1 }
        Key { code: KeyCode::Enter, alt: true, .. } => { toggle_fullscreen(ctx) }
        Key { code: KeyCode::Escape, .. } => { return true }
        _ => {}
    }
    false
}

fn toggle_fullscreen(ctx: &mut Tcod) {
    let is_fullscreen = ctx.root.is_fullscreen();
    ctx.root.set_fullscreen(!is_fullscreen)
}