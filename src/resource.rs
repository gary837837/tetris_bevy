use bevy::prelude::*;
use bevy::utils::HashMap;
use rand::Rng;

const ROTATION_TABLE: [[[(i8, i8); 4]; 4]; 7] = [
    [
        [(0, 2), (1, 2), (2, 2), (3, 2)],
        [(2, 0), (2, 1), (2, 2), (2, 3)],
        [(0, 1), (1, 1), (2, 1), (3, 1)],
        [(1, 0), (1, 1), (1, 2), (1, 3)],
    ], //I
    [
        [(0, 1), (0, 2), (1, 1), (2, 1)],
        [(1, 0), (1, 1), (1, 2), (2, 2)],
        [(0, 1), (1, 1), (2, 0), (2, 1)],
        [(0, 0), (1, 0), (1, 1), (1, 2)],
    ], //J
    [
        [(0, 1), (1, 1), (2, 1), (2, 2)],
        [(1, 0), (1, 1), (1, 2), (2, 0)],
        [(0, 0), (0, 1), (1, 1), (2, 1)],
        [(0, 2), (1, 0), (1, 1), (1, 2)],
    ], //L
    [
        [(1, 1), (1, 2), (2, 1), (2, 2)],
        [(0, 2), (1, 2), (2, 2), (3, 2)],
        [(0, 2), (1, 2), (2, 2), (3, 2)],
        [(0, 2), (1, 2), (2, 2), (3, 2)],
    ], //O
    [
        [(0, 1), (1, 1), (1, 2), (2, 2)],
        [(1, 1), (1, 2), (2, 0), (2, 1)],
        [(0, 0), (1, 0), (1, 1), (2, 1)],
        [(0, 1), (0, 2), (1, 0), (1, 1)],
    ], //S
    [
        [(0, 1), (1, 1), (1, 2), (2, 1)],
        [(1, 0), (1, 1), (1, 2), (2, 1)],
        [(0, 1), (1, 0), (1, 1), (2, 1)],
        [(0, 1), (1, 0), (1, 1), (1, 2)],
    ], //T
    [
        [(0, 2), (1, 1), (1, 2), (2, 1)],
        [(1, 0), (1, 1), (2, 1), (2, 2)],
        [(0, 1), (1, 0), (1, 1), (2, 0)],
        [(0, 0), (0, 1), (1, 1), (1, 2)],
    ], //Z
];

const WALL_KICK_TABLE_JLSTZ: [[(i8, i8); 5]; 8] = [
    [(0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2)],
    [(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)],
    [(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)],
    [(0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2)],
    [(0, 0), (1, 0), (1, 1), (0, -2), (1, -2)],
    [(0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2)],
    [(0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2)],
    [(0, 0), (1, 0), (1, 1), (0, -2), (1, -2)],
];

const WALL_KICK_TABLE_I: [[(i8, i8); 5]; 8] = [
    [(0, 0), (-2, 0), (1, 0), (-2, -1), (1, 2)],
    [(0, 0), (2, 0), (-1, 0), (2, 1), (-1, -2)],
    [(0, 0), (-1, 0), (2, 0), (-1, 2), (2, -1)],
    [(0, 0), (1, 0), (-2, 0), (1, -2), (-2, 1)],
    [(0, 0), (2, 0), (-1, 0), (2, 1), (-1, -2)],
    [(0, 0), (-2, 0), (1, 0), (-2, -1), (1, 2)],
    [(0, 0), (1, 0), (-2, 0), (1, -2), (-2, 1)],
    [(0, 0), (-1, 0), (2, 0), (-1, 2), (2, -1)],
];

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub enum Tetromino {
    I = 0,
    J,
    L,
    O,
    S,
    T,
    Z,
    G, // Ghost
}

impl Tetromino {
    pub fn gen<R: Rng + ?Sized>(rng: &mut R) -> Tetromino {
        let x: u8 = rng.gen_range(0 .. 7);
        match x {
            0 => Tetromino::I,
            1 => Tetromino::J,
            2 => Tetromino::L,
            3 => Tetromino::O,
            4 => Tetromino::S,
            5 => Tetromino::T,
            _ => Tetromino::Z,
        }
    }
}

#[derive(Clone, Copy)]
pub enum MoveDirection {
    Down,
    Left,
    Right,
    // Up,
}

#[derive(Clone, Copy)]
pub enum RotateDirection {
    Left,
    Right,
}

#[derive(Clone, Copy)]
pub enum BoardEvent {
    LineCompleted = 0,
    TetrominoPut,
    TetrominoMoved,
    GameOver,
}

#[derive(Resource)]
pub struct TileAssets {
    map: HashMap<Tetromino, Handle<Image>>,
}

impl TileAssets {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn setup(&mut self, asset_server: &Res<AssetServer>) {
        self.map
            .insert(Tetromino::I, asset_server.load("blocks/LightBlue.png"));
        self.map
            .insert(Tetromino::J, asset_server.load("blocks/Blue.png"));
        self.map
            .insert(Tetromino::L, asset_server.load("blocks/Orange.png"));
        self.map
            .insert(Tetromino::O, asset_server.load("blocks/Yellow.png"));
        self.map
            .insert(Tetromino::S, asset_server.load("blocks/Green.png"));
        self.map
            .insert(Tetromino::T, asset_server.load("blocks/Purple.png"));
        self.map
            .insert(Tetromino::Z, asset_server.load("blocks/Red.png"));
        self.map
            .insert(Tetromino::G, asset_server.load("blocks/Ghost.png"));
    }

    pub fn get(&self, k: Tetromino) -> Handle<Image> {
        self.map[&k].clone()
    }
}

#[derive(Resource)]
pub struct TetrominoAssets {
    map: HashMap<Tetromino, Handle<Image>>,
}

impl TetrominoAssets {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn setup(&mut self, asset_server: &Res<AssetServer>) {
        self.map
            .insert(Tetromino::I, asset_server.load("tetrominos/I.png"));
        self.map
            .insert(Tetromino::J, asset_server.load("tetrominos/J.png"));
        self.map
            .insert(Tetromino::L, asset_server.load("tetrominos/L.png"));
        self.map
            .insert(Tetromino::O, asset_server.load("tetrominos/O.png"));
        self.map
            .insert(Tetromino::S, asset_server.load("tetrominos/S.png"));
        self.map
            .insert(Tetromino::T, asset_server.load("tetrominos/T.png"));
        self.map
            .insert(Tetromino::Z, asset_server.load("tetrominos/Z.png"));
    }

    pub fn get(&self, k: Tetromino) -> Handle<Image> {
        self.map[&k].clone()
    }
}

#[derive(Clone)]
struct PlayerTetromino {
    tetromion: Tetromino,
    position: (i8, i8),
    rotation: u8,
}

impl PlayerTetromino {
    fn transform(&mut self, dir: MoveDirection) {
        match dir {
            MoveDirection::Down => self.position = (self.position.0, self.position.1 - 1),
            MoveDirection::Left => self.position = (self.position.0 - 1, self.position.1),
            MoveDirection::Right => self.position = (self.position.0 + 1, self.position.1),
            // MoveDirection::Up => self.position = (self.position.0, self.position.1 + 1),
        }
    }

    fn transform_xy(&mut self, x: i8, y: i8) {
        self.position = (self.position.0 as i8 + x, self.position.1 as i8 + y);
    }
}

#[derive(Resource)]
pub struct BoardMap {
    rows: [u16; 20],
    tiles: [[Tetromino; 10]; 20],
    player: Option<PlayerTetromino>,
    score: u32,
    eventflag: u8,
}

impl BoardMap {
    pub fn new() -> Self {
        Self {
            rows: [0; 20],
            tiles: [[Tetromino::I; 10]; 20],
            player: None,
            score: 0,
            eventflag: 0,
        }
    }

    pub fn tile_set(&mut self, row: u8, col: u8, tile: Option<Tetromino>) {
        let row = row as usize;
        let col = col as usize;
        if let Some(tile) = tile {
            self.rows[row] |= 1_u16 << (9 - col);
            self.tiles[row][col] = tile;
        } else {
            self.rows[row] &= !(1_u16 << (9 - col));
        }
    }

    pub fn tile_get(&self, row: u8, col: u8) -> Option<Tetromino> {
        let row = row as usize;
        let col = col as usize;
        if (self.rows[row] >> (9 - col)) & 1 == 1 {
            Some(self.tiles[row][col])
        } else {
            None
        }
    }

    fn player_draw(&mut self) {
        let Some(player) = &self.player else {
            return;
        };
        let (x, y) = player.position;
        let tetromion = Some(player.tetromion);
        for (offset_x, offset_y) in
            ROTATION_TABLE[player.tetromion as usize][player.rotation as usize]
        {
            let col = (x + offset_x) as u8;
            let row = (y + offset_y) as u8;
            self.tile_set(row, col, tetromion);
        }
    }

    fn player_erase(&mut self) {
        let Some(player) = &self.player else {
            return;
        };
        let (x, y) = player.position;
        for (offset_x, offset_y) in
            ROTATION_TABLE[player.tetromion as usize][player.rotation as usize]
        {
            let col = (x + offset_x) as u8;
            let row = (y + offset_y) as u8;
            self.tile_set(row, col, None);
        }
    }

    fn player_collision_check(&self, player: &PlayerTetromino) -> bool {
        let (x, y) = player.position;
        for (offset_x, offset_y) in
            ROTATION_TABLE[player.tetromion as usize][player.rotation as usize]
        {
            let col = x + offset_x;
            let row = y + offset_y;
            if col < 0 || col >= 10 || row < 0 || row >= 20 {
                return false;
            }
            let col = col as usize;
            let row = row as usize;
            if (self.rows[row] >> (9 - col)) & 1 == 1 {
                return false;
            }
        }
        return true;
    }

    fn player_put(&mut self) {
        self.player_draw();
        self.player = None;
        self.line_check();
        self.event_set(BoardEvent::TetrominoPut);
    }

    fn line_check(&mut self) {
        let row_len = self.rows.len();
        let mut row_idx = 0;
        while row_idx < row_len {
            if self.rows[row_idx] == 0 {
                return;
            }
            if self.rows[row_idx] & 0x3ff != 0x3ff {
                row_idx += 1;
                continue;
            }
            for row_idx in row_idx .. row_len - 1 {
                self.rows[row_idx] = self.rows[row_idx + 1];
                self.tiles[row_idx] = self.tiles[row_idx + 1];
                if self.rows[row_idx + 1] == 0 {
                    self.rows[row_idx + 1] = 0;
                    break;
                }
            }
            self.rows[row_len - 1] = 0;
            self.score += 1;
            self.event_set(BoardEvent::LineCompleted);
        }
    }

    pub fn player_spawn(&mut self, tetromion: Tetromino) {
        if self.player.is_some() {
            return;
        };
        let player = PlayerTetromino {
            tetromion,
            position: (3, 17),
            rotation: 0,
        };
        if self.player_collision_check(&player) {
            self.player = Some(player);
            self.player_draw();
        } else {
            self.event_set(BoardEvent::GameOver);
        }
    }

    pub fn player_move(&mut self, dir: MoveDirection) {
        let Some(mut player) = self.player.clone() else {
            return;
        };
        self.player_erase();
        player.transform(dir);
        if self.player_collision_check(&player) {
            self.player = Some(player);
            self.event_set(BoardEvent::TetrominoMoved);
        } else {
            match dir {
                MoveDirection::Down => self.player_put(),
                _ => (),
            }
        }
        self.player_draw();
    }

    pub fn player_move_to_bottom(&mut self) {
        let Some(mut player) = self.player.clone() else {
            return;
        };
        self.player_erase();
        player.transform(MoveDirection::Down);
        while self.player_collision_check(&player) {
            self.player = Some(player.clone());
            player.transform(MoveDirection::Down);
        }
        self.player_put();
    }

    pub fn player_rotate(&mut self, dir: RotateDirection) {
        let Some(mut player) = self.player.clone() else {
            return;
        };
        let wall_kick_table = match player.tetromion {
            Tetromino::I => WALL_KICK_TABLE_I,
            Tetromino::J | Tetromino::L | Tetromino::S | Tetromino::T | Tetromino::Z => {
                WALL_KICK_TABLE_JLSTZ
            }
            _ => {
                return;
            } // no rotation for O, G
        };
        let table_idx = match dir {
            RotateDirection::Left => {
                player.rotation = (player.rotation + 3) % 4;
                ((player.rotation * 2) + 1) as usize
            }
            RotateDirection::Right => {
                player.rotation = (player.rotation + 1) % 4;
                (((player.rotation + 3) % 4) * 2) as usize
            }
        };
        self.player_erase();
        for (offset_x, offset_y) in wall_kick_table[table_idx] {
            let mut player = player.clone();
            player.transform_xy(offset_x, offset_y);
            if self.player_collision_check(&player) {
                self.player = Some(player);
                break;
            }
        }
        self.player_draw();
    }

    pub fn score_get(&self) -> u32 {
        self.score
    }

    fn event_set(&mut self, e: BoardEvent) {
        self.eventflag |= 1 << (e as usize);
    }

    pub fn event_get(&self, e: BoardEvent) -> bool {
        (self.eventflag >> (e as usize) & 1) == 1
    }

    // pub fn event_reset(&mut self, e: BoardEvent) {
    //     self.eventflag &= !(1 << (e as usize));
    // }

    pub fn event_reset(&mut self) {
        self.eventflag = 0;
    }
}

#[derive(Resource)]
pub struct TetrominoSupplier {
    idx: usize,
    list: [Tetromino; 5],
}

impl TetrominoSupplier {
    pub fn new() -> Self {
        Self { 
            idx: 0,
            list: [Tetromino::I; 5],
        }
    }

    pub fn get(&self, idx: usize) -> Tetromino {
        self.list[(self.idx + idx) % 5]
    }

    pub fn fill<R: Rng + ?Sized>(&mut self, rng: &mut R) {
        let list = &mut self.list;
        for i in 0 .. 5 {
            list[i] = Tetromino::gen(rng);
        }
    }

    pub fn pop<R: Rng + ?Sized>(&mut self, rng: &mut R) -> Tetromino {
        let current = self.list[self.idx];
        self.list[self.idx] = Tetromino::gen(rng);
        self.idx = (self.idx + 1) % 5;
        current
    }
}