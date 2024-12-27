/*************************************************************************************************************
File : main.rs

Author : Sandeep Chikkapla Siddappa

Description  : Flappy ball game(endless highscore making game).

Usage: SPACE : jump
       P     : pause/resume game
       R     : restart game

***************************************************************************************************************/

use ggez::{
    event::{self, EventHandler, KeyCode, KeyMods},
    graphics::{self, Color, DrawMode, Mesh, Text},
    Context, GameResult,
};
use rand::Rng;

const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 600.0;
const GRAVITY: f32 = 0.5;
const ANTI_GRAVITY: f32 = -10.0;
const PIPE_WIDTH: f32 = 50.0;
const PIPE_GAP: f32 = 150.0;
const PIPE_SPEED: f32 = 2.0;

#[derive(Debug, PartialEq)]
enum GameReqType {
    Start,
    Playing,
    Paused,
    GameOver,
}

// Represents the ball in the game
struct Ball {
    x_axis: f32,
    y_axis: f32,
    velocity: f32,
}

// Represents a pipe
struct Pipe {
    x_axis: f32,
    height: f32,
    scored: bool,
}

// Main game state
struct GameReq {
    ball: Ball,
    pipes: Vec<Pipe>,
    score: i32,
    high_score: i32,
    state: GameReqType,
}

impl GameReq {
    // Initializes the game
    fn game_start() -> Self {
//        println!("Game initialized!");
        GameReq {
            ball: Ball {
                x_axis: SCREEN_WIDTH / 4.0,
                y_axis: SCREEN_HEIGHT / 2.0,
                velocity: 0.0,
            },
            pipes: vec![],
            score: 0,
            high_score: 0,
            state: GameReqType::Start,
        }
    }

    // Refresh the game state
    fn refresh_game(&mut self) {
//        println!("Game reset!");
        self.ball.y_axis = SCREEN_HEIGHT / 2.0;
        self.ball.velocity = 0.0;
        self.pipes.clear();
        self.high_score = self.high_score.max(self.score);
        self.score = 0;
        self.state = GameReqType::Start;
//        println!("High score: {}", self.high_score);
    }

    // Add or remove pipes accordingly
    fn add_or_remove_pipes(&mut self) {
        // This is like making all pipes slide towards the player
        for pipe in &mut self.pipes {
            pipe.x_axis -= PIPE_SPEED;  // Just sliding left by our speed value
        }
    
        // Keep only pipes that are visible on screen
        self.pipes.retain(|pipe| pipe.x_axis + PIPE_WIDTH > 0.0);
    
        let need_new_pipe = self.pipes.is_empty() || 
                           self.pipes.last().unwrap().x_axis < SCREEN_WIDTH - 300.0;
    
        if need_new_pipe {
            // Make a new random height for our pipe
            let mut rng = rand::thread_rng();
            let min_height = 50.0;
            let max_height = SCREEN_HEIGHT - PIPE_GAP - 50.0;
            let height = min_height + rng.gen::<f32>() * (max_height - min_height);
    
            // Create and add the new pipe
            let new_pipe = Pipe {
                x_axis: SCREEN_WIDTH,  
                height: height,        
                scored: false,         
            };
            
            self.pipes.push(new_pipe);
//            println!("Added new pipe! Height: {}", height);
        }
    
       
        // Look at each pipe to see if player passed it and score increments by 1 if it has passed.
        for pipe in &mut self.pipes {
            let passed_pipe = pipe.x_axis + PIPE_WIDTH < self.ball.x_axis;
            let not_scored_yet = !pipe.scored;
            
            if passed_pipe && not_scored_yet {
                pipe.scored = true;         
                self.score += 1;            // Add a point
 //               println!("Score is now: {}", self.score);
            }
        }
    }

    // Checks for ball if crashed with pipes or screen boundaries
    fn is_crashed(&mut self) {
        if self.ball.y_axis > SCREEN_HEIGHT || self.ball.y_axis < 0.0 {
//            println!("Ball is out of bounds! Game over.");
            self.state = GameReqType::GameOver;
        }

        for pipe in &self.pipes {
            let horizontal_overlap = self.ball.x_axis + 10.0 > pipe.x_axis
                && self.ball.x_axis - 10.0 < pipe.x_axis + PIPE_WIDTH;
            let vertical_overlap = self.ball.y_axis - 10.0 < pipe.height
                || self.ball.y_axis + 10.0 > pipe.height + PIPE_GAP;

            if horizontal_overlap && vertical_overlap {
 //               println!("Collision detected! Pipe at x: {}", pipe.x_axis);
                self.state = GameReqType::GameOver;
                return; 
            }
        }
    }
}

impl EventHandler for GameReq {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        
        if self.state != GameReqType::Playing {
            return Ok(());
        }

        self.ball.velocity += GRAVITY;
        self.ball.y_axis += self.ball.velocity;
 //       println!("Ball position updated: y = {}, velocity = {}", self.ball.y_axis, self.ball.velocity);

        self.add_or_remove_pipes();
        self.is_crashed();

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        //sky blue color
        graphics::clear(ctx, Color::from_rgb(135, 206, 235));

        // Ball with yellow color
        let ball = Mesh::new_circle(
            ctx,
            DrawMode::fill(),
            [self.ball.x_axis, self.ball.y_axis],
            10.0,
            0.1,
            Color::from_rgb(255, 255, 0), // Yellow color
        )?;
        graphics::draw(ctx, &ball, graphics::DrawParam::default())?;

        // Draw pipes
        for pipe in &self.pipes {
            let top_pipe = Mesh::new_rectangle(
                ctx,
                DrawMode::fill(),
                [pipe.x_axis, 0.0, PIPE_WIDTH, pipe.height].into(),
                Color::from_rgb(0, 255, 0), // Green color
            )?;
            let bottom_pipe = Mesh::new_rectangle(
                ctx,
                DrawMode::fill(),
                [
                    pipe.x_axis,
                    pipe.height + PIPE_GAP,
                    PIPE_WIDTH,
                    SCREEN_HEIGHT - pipe.height - PIPE_GAP,
                ]
                .into(),
                Color::from_rgb(0, 255, 0), // Green color
            )?;
            graphics::draw(ctx, &top_pipe, graphics::DrawParam::default())?;
            graphics::draw(ctx, &bottom_pipe, graphics::DrawParam::default())?;
        }

        // Put the score and high score on window
        let score_text = Text::new(format!("Score: {}", self.score));
        graphics::draw(ctx, &score_text, ([10.0, 10.0],))?;
        let high_score_text = Text::new(format!("High Score: {}", self.high_score));
        graphics::draw(ctx, &high_score_text, ([10.0, 40.0],))?;

        // Put messages based on the game state
        match self.state {
            GameReqType::Start => {
                let start_text = Text::new("Press Space to Start!");
                graphics::draw(ctx, &start_text, ([SCREEN_WIDTH / 2.0 - 100.0, SCREEN_HEIGHT / 2.0],))?;
            }
            GameReqType::Paused => {
                let pause_text = Text::new("Paused. Press P to Resume.");
                graphics::draw(ctx, &pause_text, ([SCREEN_WIDTH / 2.0 - 100.0, SCREEN_HEIGHT / 2.0],))?;
            }
            GameReqType::GameOver => {
                let game_over_text = Text::new("Game Over! Press R to Reset.");
                graphics::draw(ctx, &game_over_text, ([SCREEN_WIDTH / 2.0 - 100.0, SCREEN_HEIGHT / 2.0],))?;
            }
            _ => {}
        }

        graphics::present(ctx)?;
        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods, _: bool) {
        match keycode {
            KeyCode::Space => {
                if self.state == GameReqType::Start {
                    self.state = GameReqType::Playing;
               //     println!("Game started!");
                } else if self.state == GameReqType::Playing {
                    self.ball.velocity = ANTI_GRAVITY;
               //     println!("Ball jumped! New velocity: {}", self.ball.velocity);
                }
            }
            KeyCode::P => {
                if self.state == GameReqType::Playing {
                    self.state = GameReqType::Paused;
//                    println!("Game paused.");
                } else if self.state == GameReqType::Paused {
                    self.state = GameReqType::Playing;
//                   println!("Game resumed.");
                }
            }
            KeyCode::R => {
                self.refresh_game();
//                println!("Game reset.");
            }
            _ => {}
        }
    }
}

//Negative test cases.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_ball_out_of_bounds_test() {
        let mut game = GameReq::game_start();
        game.ball.y_axis = SCREEN_HEIGHT + 10.0; 

        game.is_crashed();

        assert_eq!(game.state, GameReqType::GameOver, "Game should end when ball goes out of bounds.");
    }
    

    #[test]
fn collision_with_pipe_test() {
    let mut game = GameReq::game_start();
    
    game.state = GameReqType::Playing;

    game.ball.x_axis = 200.0;  
    game.ball.y_axis = 100.0;  
    
    game.pipes.push(Pipe {
        x_axis: 195.0,         
        height: 120.0,         
        scored: false,
    });
    
    // Check collision
    game.is_crashed();
    
    assert_eq!(game.state,GameReqType::GameOver,"Game should end when ball collides with pipe at position ({}, {})",game.ball.x_axis,game.ball.y_axis);
}
    

    #[test]
    fn no_score_increment_without_pipe_pass_test() {

        let mut game = GameReq::game_start();
        game.pipes.push(Pipe {
            x_axis: game.ball.x_axis + 100.0, 
            height: SCREEN_HEIGHT / 2.0,
            scored: false,
        });
        game.add_or_remove_pipes();
        assert_eq!(game.score, 0, "Score should not increment when the ball hasn't passed a pipe.");
    }


//Positive test cases.
    #[test]
    fn ball_movement_test() {
    
        let mut game = GameReq::game_start();
        game.state = GameReqType::Playing;
        
        let initial_y = game.ball.y_axis;
        
        game.ball.velocity = ANTI_GRAVITY;
        
        game.ball.y_axis += game.ball.velocity;
        
        assert!(game.ball.y_axis < initial_y, "Ball should move upward when jumping");
    }

    #[test]
    fn pipe_generation_test() {
    
        let mut game = GameReq::game_start();
        game.state = GameReqType::Playing;
        
        assert_eq!(game.pipes.len(), 0, "Should start with no pipes");
        
        game.add_or_remove_pipes();
        
        assert_eq!(game.pipes.len(), 1, "Should generate one pipe");
        
        let pipe = &game.pipes[0];
        assert_eq!(pipe.x_axis, SCREEN_WIDTH, "New pipe should start at screen width");
    }

    #[test]
    fn score_tracking_test() {
        let mut game = GameReq::game_start();
        game.state = GameReqType::Playing;
        
        game.pipes.push(Pipe {
            x_axis: game.ball.x_axis - PIPE_WIDTH - 10.0, 
            height: 200.0,
            scored: false,
        });
    
        game.add_or_remove_pipes();
        
        assert_eq!(game.score, 1, "Should score point after passing pipe");
        assert!(game.pipes[0].scored, "Pipe should be marked as scored");
    }

}

pub fn main() -> GameResult {
    let contxt_bldr = ggez::ContextBuilder::new("flappy_ball", "Sandeep Chikkapla Siddappa")
        .window_setup(ggez::conf::WindowSetup::default().title("Flappy Ball"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_WIDTH, SCREEN_HEIGHT));
    let (ctx, event_loop) = contxt_bldr.build()?;
    let state = GameReq::game_start();
    event::run(ctx, event_loop, state)
}

