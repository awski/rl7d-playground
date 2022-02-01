use tcod::{Console, Color, FontLayout, FontType};
use tcod::input::{Key, KeyCode};
use tcod::console::{Root, Offscreen, BackgroundFlag, blit};
use tcod::colors::*;
use rand::Rng;

const WINDOW_WIDTH: i32 = 80;
const WINDOW_HEIGHT: i32 = 50;
const FPS_LIMIT: u32 = 20;

const MAP_WIDTH: i32 = 80;
const MAP_HEIGHT: i32 = 45;
const WALL_DARK_COLOR: Color = Color { r: 0, g: 0, b: 100 };
const GROUND_DARK_COLOR: Color = Color { r: 50, g: 50, b: 150 };

const ROOM_MAX_SIZE: i32 = 10;
const ROOM_MIN_SIZE: i32 = 6;
const MAX_ROOMS: i32 = 30;

struct Tcod {
    root: Root,
    con: Offscreen
}

struct Object {
    x: i32,
    y: i32,
    char: char,
    color: Color
} impl Object {
    pub fn new(x: i32, y: i32, char: char, color: Color) -> Self {
        Object { x, y, char, color }
    }

    pub fn move_by(&mut self, dx: i32, dy: i32, game: &Game) {
        if !game.map[(self.x + dx) as usize][(self.y + dy) as usize].blocked {
            self.x += dx;
            self.y += dy;
        }
    }

    pub fn draw(&self, console: &mut Offscreen) {
        console.set_default_foreground(self.color);
        console.put_char(self.x, self.y, self.char, BackgroundFlag::None);
    }
}

#[derive(Clone, Copy)]
struct Tile {
    blocked: bool,
    block_sight: bool,
} impl Tile {
    pub fn empty() -> Self {
        Tile {blocked: false, block_sight: false}
    }
    pub fn wall() -> Self {
        Tile {blocked: true, block_sight: true}
    }
}

struct Game {
    map: Map
}

#[derive(Debug, Clone, Copy)]
struct Rect {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
} impl Rect {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        Rect { x1: x, y1: y, x2: x + w, y2: y + h }
    }
    pub fn center(&self) -> (i32, i32) {
        let center_x = (self.x1 + self.x2) / 2;
        let center_y = (self.y1 + self.y2) / 2;
        (center_x, center_y)
    }
    pub fn intersects_with(&self, other: &Rect) -> bool {
        (self.x1 <= other.x2)
            && (self.x2 >= other.x1)
            && (self.y1 <= other.y2)
            && (self.y2 >= other.y1)
    }
}

type Map = Vec<Vec<Tile>>;

fn main() {
    let mut ctx = Tcod {
        root: Root::initializer()
            .font("arial10x10.png", FontLayout::Tcod)
            .font_type(FontType::Greyscale)
            .size(WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32)
            .title("rl7d playground")
            .init(),
        con: Offscreen::new(MAP_WIDTH, MAP_HEIGHT)
    };
    tcod::system::set_fps(FPS_LIMIT as i32);

    let game = Game { map: make_map() };
    let player = Object::new(25, 23, '@', WHITE);
    let mut objects = [player];

    while !ctx.root.window_closed() {
        ctx.con.clear();
        render_all(&mut ctx, &game, &objects);

        ctx.root.flush();

        let player = &mut objects[0];
        let exit = handle_keys(&mut ctx, &game, player);
        if exit {
            return;
        }
    }
}

fn handle_keys(ctx: &mut Tcod, game: &Game, player: &mut Object) -> bool {
    match ctx.root.wait_for_keypress(true) {
        Key { code: KeyCode::Up, .. } => { player.move_by(0, -1, game) }
        Key { code: KeyCode::Down, .. } => { player.move_by(0, 1, game) }
        Key { code: KeyCode::Left, .. } => { player.move_by(-1, 0, game) }
        Key { code: KeyCode::Right, .. } => { player.move_by(1, 0, game) }
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

fn make_map() -> Map {
    let mut map = vec![vec![Tile::empty(); MAP_HEIGHT as usize]; MAP_WIDTH as usize];
    let mut rooms = vec![];

    for _ in 0..MAX_ROOMS {
        let w = rand::thread_rng().gen_range(ROOM_MIN_SIZE..ROOM_MAX_SIZE + 1);
        let h = rand::thread_rng().gen_range(ROOM_MIN_SIZE..ROOM_MAX_SIZE + 1);

        let x = rand::thread_rng().gen_range(0..MAP_WIDTH - w);
        let y = rand::thread_rng().gen_range(0..MAP_HEIGHT - h);

        let new_room = Rect::new(x, y, w, h);
        if rooms.iter().any(|room| new_room.intersects_with(room)) {
            continue;
        }

        create_room(new_room, &mut map);
        let (new_x, new_y) = new_room.center();

        if !rooms.is_empty() {
            let (prev_x, prev_y) = rooms[rooms.len() - 1].center();
            if rand::random() {
                create_h_tunnel(prev_x, new_x, prev_y, &mut map);
                create_v_tunnel(prev_y, new_y, new_x, &mut map);
            } else {
                create_h_tunnel(prev_x, new_x, new_y, &mut map);
                create_v_tunnel(prev_y, new_y, prev_x, &mut map);
            }
        }

        rooms.push(new_room)
    }

    map
}

fn create_map(room: Rect, map: &mut Map) {
    for x in (room.x1 + 1)..room.x2 {
        for y in (room.y1 + 1)..room.y2 {
            map[x as usize][y as usize] = Tile::empty()
        }
    }
}

fn render_all(ctx: &mut Tcod, game: &Game, objects: &[Object]) {
    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            let wall = game.map[x as usize][y as usize].block_sight;
            if wall {
                ctx.con
                    .set_char_background(x, y, WALL_DARK_COLOR, BackgroundFlag::Set);
            } else {
                ctx.con
                    .set_char_background(x, y, GROUND_DARK_COLOR, BackgroundFlag::Set);
            }
        }
    }

    for obj in objects {
        obj.draw(&mut ctx.con);
    }

    blit(&ctx.con, (0,0), (MAP_HEIGHT, MAP_WIDTH), &mut ctx.root, (0,0), 1.0, 1.0);
}

fn create_h_tunnel(x1: i32, x2: i32, y: i32, map: &mut Map) {
    for x in std::cmp::min(x1, x2)..(std::cmp::max(x1, x2) + 1) {
        map[x as usize][y as usize] = Tile::empty();
    }
}

fn create_v_tunnel(y1: i32, y2: i32, x: i32, map: &mut Map) {
    for y in std::cmp::min(y1, y2)..(std::cmp::max(y1, y2) + 1) {
        map[x as usize][y as usize] = Tile::empty();
    }
}

fn create_room(room: Rect, map: &mut Map) {
    for x in (room.x1 + 1)..room.x2 {
        for y in (room.y1 + 1)..room.y2 {
            map[x as usize][y as usize] = Tile::empty();
        }
    }
}