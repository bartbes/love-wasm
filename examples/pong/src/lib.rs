wit_bindgen::generate!({
    world: "game-world",
    generate_all,
});

use crate::exports::bartbes::love_wasm::game;
use crate::bartbes::love_wasm as love;

struct LoveEventIterator;

impl std::iter::Iterator for LoveEventIterator {
    type Item = love::event::Message;

    fn next(&mut self) -> Option<Self::Item> {
        love::event::poll()
    }
}

struct Paddle
{
    x: f32,
    y: f32,
    w: f32,
    h: f32,
}

struct Ball
{
    x: f32,
    y: f32,
    r: f32,
    vx: f32,
    vy: f32,
}

enum Direction
{
    Left,
    Right,
}

struct Pong
{
    score: (u8, u8),
    player1: Paddle,
    player2: Paddle,
    ball: Ball,
}

impl Pong
{
    const WIDTH: f32 = 800.0;
    const HEIGHT: f32 = 600.0;
    const MOVEMENT_SPEED: f32 = Self::WIDTH / 4.0;

    fn new() -> Pong {
        const PADDLE_GAP: f32 = 10.0;
        const PADDLE_WIDTH: f32 = 10.0;
        const PADDLE_HEIGHT: f32 = 40.0;
        let mut pong = Self{
            score: (0, 0),
            player1: Paddle{x: PADDLE_GAP, y: (Self::HEIGHT-PADDLE_HEIGHT)/2.0, w: PADDLE_WIDTH, h: PADDLE_HEIGHT},
            player2: Paddle{x: Self::WIDTH - PADDLE_GAP - PADDLE_WIDTH, y: (Self::HEIGHT-PADDLE_HEIGHT)/2.0, w: PADDLE_WIDTH, h: PADDLE_HEIGHT},
            ball: Ball{x: 0.0, y: 0.0, r: 0.0, vx: 0.0, vy: 0.0}, // Replaced by respawn_ball
        };
        pong.respawn_ball(Direction::Left);
        pong
    }

    fn respawn_ball(&mut self, direction: Direction) {
        let vx = match direction {
            Direction::Left => -120.0,
            Direction::Right => 120.0,
        };
        self.ball = Ball{x: Self::WIDTH / 2.0, y: Self::HEIGHT / 2.0, r: 4.0, vx, vy: 0.0}
    }

    fn collide_with_paddle(ball: &Ball, paddle: &Paddle) -> bool {
        let dx = if ball.x < paddle.x { ball.x - paddle. x}
            else if ball.x > paddle.x + paddle.w { paddle.x + paddle.w - ball.x }
            else { 0.0 };
        let dy = if ball.y < paddle.y { ball.y - paddle. x}
            else if ball.y > paddle.y + paddle.h { paddle.y + paddle.h - ball.y }
            else { 0.0 };
        let dist = dx.powi(2) + dy.powi(2);
        dist <= ball.r.powi(2)
    }

    fn apply_bounce(ball: &mut Ball, paddle: &Paddle) {
        let current_velocity = (ball.vx.powi(2) + ball.vy.powi(2)).sqrt();
        let new_velocity = current_velocity * 1.2;
        let angle_to_middle = (
            (ball.x - paddle.x - (paddle.w / 2.0)),
            (ball.y - paddle.y - (paddle.h / 2.0)));
        let length = (angle_to_middle.0.powi(2) + angle_to_middle.1.powi(2)).sqrt();
        ball.vx = (angle_to_middle.0 / length) * new_velocity;
        ball.vy = (angle_to_middle.1 / length) * new_velocity;
        ball.x += (angle_to_middle.0 / length) * ball.r;
        ball.y += (angle_to_middle.1 / length) * ball.r;
    }

    fn update(&mut self, dt: f64) {
        let dt = dt as f32;

        self.ball.x += self.ball.vx * dt;
        self.ball.y += self.ball.vy * dt;

        if love::keyboard::is_down(love::keyboard::Key::Down) {
            self.player1.y = (self.player1.y + Self::MOVEMENT_SPEED * dt).min(Self::HEIGHT - self.player1.h);
        }
        else if love::keyboard::is_down(love::keyboard::Key::Up) {
            self.player1.y = (self.player1.y - Self::MOVEMENT_SPEED * dt).max(self.player1.h);
        }
        if self.ball.y < self.player2.y {
            self.player2.y = (self.player2.y - Self::MOVEMENT_SPEED * dt).max(self.player2.h);
        }
        else if self.ball.y > self.player2.y + self.player2.h {
            self.player2.y = (self.player2.y + Self::MOVEMENT_SPEED * dt).min(Self::HEIGHT - self.player2.h);
        }

        if Self::collide_with_paddle(&self.ball, &self.player1) {
            Self::apply_bounce(&mut self.ball, &self.player1);
        }
        else if Self::collide_with_paddle(&self.ball, &self.player2) {
            Self::apply_bounce(&mut self.ball, &self.player2);
        }
        else if self.ball.x < self.ball.r {
            self.score.1 += 1;
            self.respawn_ball(Direction::Right);
        }
        else if self.ball.x > Self::WIDTH-self.ball.r {
            self.score.0 += 1;
            self.respawn_ball(Direction::Left);
        }
        else if self.ball.y < self.ball.r {
            self.ball.vy = -self.ball.vy
        }
        else if self.ball.y > Self::HEIGHT-self.ball.r {
            self.ball.vy = -self.ball.vy
        }
    }

    fn draw(&self) {
        love::graphics::rectangle(love::graphics::DrawMode::Fill, self.player1.x, self.player1.y, self.player1.w, self.player1.h);
        love::graphics::rectangle(love::graphics::DrawMode::Fill, self.player2.x, self.player2.y, self.player2.w, self.player2.h);
        love::graphics::circle(love::graphics::DrawMode::Fill, self.ball.x, self.ball.y, self.ball.r, None);

        love::graphics::print(&format!("{0} - {1}", self.score.0, self.score.1), Self::WIDTH/2.0-20.0, 20.0);
    }

    fn quit(&self) -> bool {
        false
    }

    fn keypressed(&self, _key: &str, _scancode: &str, _is_repeat: bool) {
    }

    fn unknown_event(&self, _event: &love::event::Message) {
    }
}

enum DispatchResult
{
    Dispatched,
    Unknown,
    Quit,
}

fn dispatch_event(pong: &mut Pong, event: &love::event::Message) -> DispatchResult
{
    match &event.name as &str {
        "quit" => {
            if pong.quit() {
                DispatchResult::Dispatched
            }
            else {
                DispatchResult::Quit
            }
        },
        "keypressed" => {
             if let [love::common::Variant::String(key), love::common::Variant::String(scancode), love::common::Variant::Boolean(is_repeat)] = &event.args[..] {
                 pong.keypressed(key, scancode, *is_repeat);
                 DispatchResult::Dispatched
             }
             else {
                 DispatchResult::Unknown
             }
        },
        _ => DispatchResult::Unknown
    }
}

impl game::Guest for Pong
{
    fn main() -> () {
        love::window::set_window_title("Pong");
        love::window::set_mode(Pong::WIDTH as u32, Pong::HEIGHT as u32);

        love::event::pump();
        love::event::pump();

        let mut pong = Pong::new();
        loop {
            love::timer::step();

            love::event::pump();
            for e in LoveEventIterator {
                match dispatch_event(&mut pong, &e) {
                    DispatchResult::Quit => return,
                    DispatchResult::Dispatched => (),
                    DispatchResult::Unknown => pong.unknown_event(&e),
                }
            }

            pong.update(love::timer::get_delta());

            love::graphics::origin();
            love::graphics::clear();

            pong.draw();

            love::graphics::present();

            love::timer::sleep(1.0/60.0);
        }
    }
}

export!(Pong);
