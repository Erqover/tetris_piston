// piston
use piston_window::{WindowSettings, PistonWindow, Event, RenderEvent, PressEvent};
use piston_window::{Button, Key};
use piston_window::*;

use std::collections::HashSet; // hashset

use std::time::{Duration, Instant}; // time
use rand::Rng; // random

const BOARD_X: u8 = 10;
const BOARD_Y: u8 = 20;
const SHIFT: f64 = 50.0;
const SCALE: f64 = 40.0;


#[derive(Debug, Clone, Copy)]
enum Color {
    Red,
    Purple,
    Aqua,
    Yellow,
    Orange,
    Blue,
    Green,
    White,
}

struct Game {
    board: Board,
    tetr: Tetromin,
    points: u128,
    window: PistonWindow,
}

impl Game {
    fn new() -> Self {
        Self {
            board: Board::new(),
            points: 0,
            tetr: Tetromin::new(4, 0, 0),
            window: WindowSettings::new("Tetris", [600, 1000])
                .exit_on_esc(true).build().unwrap(),
        }
    }

    fn game(&mut self) {
        while let Some(e) = self.window.next() {
            
            // game progression
            if self.tetr.fall(&self.board) {
                self.tetr = self.new_tetro();
                self.board.del_lines();
            }

            //event handlers
            if let Some(args) = e.press_args() {
                self.press_handler(args);
            }
            if let Some(_) = e.render_args() {
                self.draw(e);
            }
        }
    }

    // fn game_over(&mut self) {
    //     panic!("Game over! \nTwój wynik: {}",self.points);
    // }

    fn draw(&mut self, e: Event) {
            let mut font = self.window.load_font("./HackNerdFont-Regular.ttf").unwrap();

            self.window.draw_2d(&e, |c, g, _device| {
                clear([1.0; 4], g);

                let border_color = [0.0, 0.0, 0.0, 1.0]; // Czarny kolor
                let border_thickness = 2.0;

                let x = SHIFT - 2.0;
                let y = SHIFT - 2.0;
                let width:f64 = BOARD_X as f64 * SCALE + 44.0;
                let height = BOARD_Y as f64 * SCALE + 44.0;
                line(
                    border_color,
                    border_thickness,
                    [x, y, x + width, y],
                    c.transform,
                    g,
                );
                line(
                    border_color,
                    border_thickness,
                    [x, y + height, x + width, y + height],
                    c.transform,
                    g,
                );
                line(
                    border_color,
                    border_thickness,
                    [x, y, x, y + height],
                    c.transform,
                    g,
                );
                line(
                    border_color,
                    border_thickness,
                    [x + width, y, x + width, y + height],
                    c.transform,
                    g,
                );

                let col = match &self.tetr.color {
                    Color::Red     => [1.0, 0.0, 0.0, 1.0],
                    Color::Green   => [0.0, 1.0, 0.0, 1.0],
                    Color::Blue    => [0.5, 0.5, 1.0, 1.0],
                    Color::Purple  => [1.0, 0.0, 1.0, 1.0],
                    Color::Aqua    => [0.0, 1.0, 1.0, 1.0],
                    Color::Yellow  => [1.0, 1.0, 0.0, 1.0],
                    Color::Orange  => [1.0, 0.5, 0.0, 1.0],
                    Color::White   => [1.0, 1.0, 1.0, 1.0],
                };
                for (y, x) in &self.tetr.pos {
                    rectangle(col, // red
                        [(x.to_owned() as f64) * SCALE + SHIFT, (y.to_owned() as f64) * SCALE + SHIFT, SCALE, SCALE],
                        c.transform, g);
                }
                for (y, x, cl) in &self.board.brd {
                    let col = match cl {
                        Color::Red     => [1.0, 0.0, 0.0, 1.0],
                        Color::Green   => [0.0, 1.0, 0.0, 1.0],
                        Color::Blue    => [0.5, 0.5, 1.0, 1.0],
                        Color::Purple  => [1.0, 0.0, 1.0, 1.0],
                        Color::Aqua    => [0.0, 1.0, 1.0, 1.0],
                        Color::Yellow  => [1.0, 1.0, 0.0, 1.0],
                        Color::Orange  => [1.0, 0.5, 0.0, 1.0],
                        Color::White   => [1.0, 1.0, 1.0, 1.0],
                    };
                    rectangle(col, // red
                        [(x.to_owned() as f64) * SCALE + SHIFT, (y.to_owned() as f64) * SCALE + SHIFT, SCALE, SCALE],
                        c.transform, g);
                }

                text::Text::new_color([0.0, 0.0, 0.0, 1.0], 32)
                .draw(
                    &format!("Points: {}", self.points),
                    &mut font,
                    &c.draw_state,
                    c.transform.trans(10.0, 29.0), g
                ).unwrap();

                font.factory.encoder.flush(_device);
                
            });
        
        }

    fn points_act(&mut self ,up: u128) {
        self.points += up;
    }

    fn new_tetro(&mut self) -> Tetromin {
        let col: Color = self.tetr.color;
        for (y, x) in &self.tetr.pos {
            self.board.brd.push((y.to_owned(), x.to_owned(), col));
        }
        let mut rng = rand::thread_rng();
        let idx = rng.gen_range(1..8);
        if self.board.brd.iter().any(|&(y, _, _)| y == 0) { 
            todo!("Game over");
        }
        self.points_act(100);
        return Tetromin::new(idx, 0, 5);
    }

    fn press_handler(&mut self, btn: Button) {
        match btn {
            Button::Keyboard(key) => { self.key_handler(key); }
            _ => {},
        }
    }

    fn key_handler(&mut self, key: Key) {
            match key {
                Key::A => self.tetr.move_left(&self.board),
                Key::D => self.tetr.move_right(&self.board),
                Key::Q => self.tetr.rotate_270(&self.board),
                Key::E => self.tetr.rotate_90(&self.board),
                Key::S => {if self.tetr.move_down(&self.board) {}},
                Key::Space => {if self.tetr.super_move_down(&self.board) {}},
                _ => {},
            }
    }
}


struct Board {
    brd: Vec<(u8, u8, Color)>,
}

impl Board {
    fn new() -> Self {
        Self {
            brd: Vec::new(),
        }
    }

    fn check_lines(&self) -> Vec<u8> {
        let set: HashSet<u8> = self.brd.iter()
            .map(|&(y, _, _)| y)
            .collect();

        let sett = set.into_iter().collect::<Vec<u8>>();
        let mut full_lines:Vec<u8> = Vec::new();
        for i in sett {
            if (&self.brd.iter().map(|&(y, x, _)|(y, x)).filter(|&(y, _)| y == i).count()).to_owned() == (BOARD_X+1) as usize {
                full_lines.push(i);
            }
        }
        return full_lines;
    }

    fn del_lines(&mut self) -> u128 {
        let flines:Vec<u8> = self.check_lines();
        if !flines.is_empty() {
            self.brd.retain(|&(y, _x, _c)| !flines.contains(&y));
            self.drop_down(flines.iter().min().unwrap().to_owned(), &(flines.len() as u8));
            if flines.len() < 4 {
                return flines.len() as u128 * 1000;
            }
            else {
                return flines.len() as u128 * 2000;
            }
        }
        else {
            return 0;
        }
    }

    fn drop_down(&mut self, dline: u8, nlines: &u8) {
        for i in &mut self.brd {
            if i.0 < dline {
                i.0 += nlines;
            }
        }
    }
}

struct Tetromin {
    color: Color,
    pos: Vec<(u8,u8)>,
    time_since_fall: Instant,
}

impl Tetromin {
    fn new(typ: u8, sy: u8, sx: u8) -> Self {
        Self {
            time_since_fall: Instant::now(),
            color: match typ {
                1 => Color::Red,
                2 => Color::Blue,
                3 => Color::Green,
                4 => Color::Orange,
                5 => Color::Aqua,
                6 => Color::Purple,
                7 => Color::Yellow,
                _ => Color::White,
            },
            pos: match typ {
                1 => vec![(0 + sy, 0 + sx), (0 + sy, 1 + sx), (1 + sy, 1 + sx), (1 + sy, 2 + sx)],
                2 => vec![(2 + sy, 0 + sx), (2 + sy, 1 + sx), (1 + sy, 1 + sx), (0 + sy, 1 + sx)],
                3 => vec![(1 + sy, 0 + sx), (1 + sy, 1 + sx), (0 + sy, 1 + sx), (0 + sy, 2 + sx)],
                4 => vec![(0 + sy, 0 + sx), (1 + sy, 0 + sx), (2 + sy, 0 + sx), (2 + sy, 1 + sx)],
                5 => vec![(0 + sy, 0 + sx), (1 + sy, 0 + sx), (2 + sy, 0 + sx), (3 + sy, 0 + sx)],
                6 => vec![(0 + sy, 0 + sx), (0 + sy, 1 + sx), (0 + sy, 2 + sx), (1 + sy, 1 + sx)],
                7 => vec![(0 + sy, 0 + sx), (0 + sy, 1 + sx), (1 + sy, 0 + sx), (1 + sy, 1 + sx)],
                _ => vec![(0 + sy, 0 + sx)],
            }
        }
    }

    fn rotate_90(&mut self, board: &Board) {
        let sx  = (self.pos.iter()
            .map(|&(_, x)| x)
            .min()
            .unwrap()
            .to_owned())
            .to_owned() as i8;
        let sy = (self.pos.iter()
            .map(|&(y, _)| y)
            .max()
            .unwrap()
            .to_owned())
            .to_owned() as u8;
        
        let max_y = (&self.pos
            .iter()
            .map(|&(y, _)| y)
            .max()
            .unwrap()
            .to_owned())
            .to_owned() as i8;

        let rot: Vec<(u8, u8)> = self.pos
            .iter()
            .map(|&(y, x)| ((x - sx as u8 + sy) -1, (-(y as i8) + max_y + sx) as u8))
            .collect();


        if rot.iter().map(|&(y, _)| y).max().unwrap() > 20 {
            return;
        }
        else if rot.iter().map(|&(_, x)| x).max().unwrap() > 10 {
            self.pos = rot
                .iter()
                .map(|&(y, x)| (y, x - 1))
                .collect();
            return;
        }

        for (y, x, _) in &board.brd {
            if rot.contains(&(y.to_owned(), x.to_owned())) {
                return;
            }
        }

        self.pos = rot;

    }

    fn fall(&mut self, board: &Board) -> bool{

        if self.time_since_fall.elapsed() <= Duration::from_millis(1000) {
            return false;
        }
        self.time_since_fall = Instant::now();
        
        return self.move_down(&board);
    }

    fn collision_check(&mut self, dy: i8, dx: i8, board: &Board) -> bool{
        let tmp_collider:Vec<(u8,u8)> = (&self.pos
            .iter()
            .map(|&(y, x)| ((y as i8 + dy) as u8, (x  as i8+ dx) as u8))
            .collect::<Vec<(u8,u8)>>())
            .to_owned();
        for (y, x, _) in &board.brd {
            if tmp_collider.contains(&(y.to_owned(), x.to_owned())) {
                return true;
            }
        }

        return false;
    }

    fn rotate_270(&mut self, board: &Board) {
        for _ in 0..3 {
            self.rotate_90(board);
        }
    }

    fn move_down(&mut self, board: &Board) -> bool {
        if &self.pos.iter().map(|&(y, _)| y).max().unwrap() < &BOARD_Y && !self.collision_check(1, 0, &board){
            self.pos = self.pos.iter().map(|&(y, x)| (y+1, x)).collect();
            return false;
        }
        return true;
    }

    // niedopracowane
    fn super_move_down(&mut self, board: &Board) -> bool {
        loop {
            if self.move_down(board) {
                return true;
            }
        }
    }

    fn move_left(&mut self, board: &Board) {
        if &self.pos.iter().map(|&(_, x)| x).min().unwrap() > &0 && !self.collision_check(0, -1, &board) {
            self.pos = self.pos.iter().map(|&(y, x)| (y, x-1)).collect();
        }
    }

    fn move_right(&mut self, board: &Board) {
        if &self.pos.iter().map(|&(_, x)| x).max().unwrap() < &BOARD_X && !self.collision_check(0, 1, &board) {
            self.pos = self.pos.iter().map(|&(y, x)| (y, x+1)).collect();
        }
    }
}

fn main() {
    let mut game: Game = Game::new();
    game.game();
}