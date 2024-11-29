use circle::Circle;
use raylib::consts::KeyboardKey::*;
use raylib::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
mod circle;

const WINDOW_WIDTH: i32 = 640;
const WINDOW_HEIGHT: i32 = 480;
const FPS: u32 = 60;

pub struct Game {
    rl: Rc<RefCell<RaylibHandle>>,
    thread: RaylibThread,
    circles: Vec<Circle>,
}

impl Game {
    pub fn new() -> Self {
        let (rh, thread) = raylib::init().build();

        let circle1 = Circle::new(100, 100, 50);
        let circle2 = Circle::new(200, 150, 30);

        let game = Game {
            rl: Rc::new(RefCell::new(rh)),
            thread,
            circles: vec![circle1, circle2],
        };

        let rh = game.rl.clone();
        let mut rh = rh.borrow_mut();

        rh.set_target_fps(FPS);
        rh.set_window_size(WINDOW_WIDTH, WINDOW_HEIGHT);
        rh.set_exit_key(Some(KEY_ESCAPE));

        game
    }

    pub fn update(&self) {}

    pub fn draw(&mut self) {
        let mut rh = self.rl.borrow_mut();
        let mut d = rh.begin_drawing(&self.thread);
        d.clear_background(Color::BLACK);

        self.circles[0].draw(&mut d);
        self.circles[1].draw(&mut d);

        d.draw_text(
            "Press ESC to exit",
            20,
            WINDOW_HEIGHT - 30,
            24,
            Color::WHITE,
        );
    }

    pub fn has_finished(&self) -> bool {
        let rh = self.rl.borrow();
        rh.window_should_close()
    }
}
