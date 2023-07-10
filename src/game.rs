use piston_window::types::Color;
use piston_window::*;

use rand::{thread_rng, Rng};

use crate::draw::{draw_block, draw_rectangle};
use crate::snake::{Direction, Snake};

const FOOD_COLOR: Color = [1.00, 0.00, 0.00, 1.0]; //Red Colored Food
const BORDER_COLOR: Color = [0.00, 0.00, 0.00, 1.0]; //Black Colored Walls
const GAMEOVER_COLOR: Color = [0.00, 0.00, 1.00, 0.5]; //Blue Gameover Screen with 50% Opacity

const MOVING_PERIOD: f64 = 0.1;
const RESTART_TIME: f64 = 1.0;

pub struct Game
{
    //Player
    snake: Snake,
    //Food
    food_exists: bool,
    food_x: i32,
    food_y: i32,
    //Dimensions
    width: i32,
    height: i32,
    //Status Parameters
    game_over: bool,
    waiting_time: f64,
}

impl Game //Implementation of the Game Struct (basic OOP (sorta))
{
    pub fn new(width: i32, height: i32) -> Game
    {
        Game
        {
            snake: Snake::new(2, 2),
            waiting_time: 0.0,
            food_exists: true,
            food_x: 6,
            food_y: 4,
            width,
            height,
            game_over: false,
        }
    }

    pub fn key_pressed(&mut self, key: Key)
    {
        if (self.game_over)
        {
            return;
        }

        let dir = match key
        {
            Key::Up => Some(Direction::Up),
            Key::Down => Some(Direction::Down),
            Key::Left => Some(Direction::Left),
            Key::Right => Some(Direction::Right),
            _ => Some(self.snake.head_direction()), //Looks like "space key" doesn't do anything
        };

        if let Some(dir) = dir
        {
            if (dir == self.snake.head_direction().opposite()) //Can't go opposite of the head direction inward the snake
            {
                return;
            }
        }

        //Update Position with new direction
        self.update_snake(dir);
    }

    pub fn draw(&self, context: &Context, G: &mut G2d)
    {
        self.snake.draw(context, G);

        if (self.food_exists)
        {
            draw_block(FOOD_COLOR, self.food_x, self.food_y, context, G);
        }

        //Draw black walls around corners
        draw_rectangle(BORDER_COLOR, 0, 0, self.width, 1, context, G);
        draw_rectangle(BORDER_COLOR, 0, self.height - 1, self.width, 1, context, G);
        draw_rectangle(BORDER_COLOR, 0, 0, 1, self.height, context, G);
        draw_rectangle(BORDER_COLOR, self.width - 1, 0, 1, self.height, context, G);

        //Draw Gameover Block
        if (self.game_over)
        {
            draw_rectangle(GAMEOVER_COLOR, 0, 0, self.width, self.height, context, G);
        }
    }

    pub fn update(&mut self, delta_time: f64)
    {
        self.waiting_time += delta_time;

        if (self.game_over)
        {
            if (self.waiting_time > RESTART_TIME)
            {
                self.restart();
            }
            
            return;
        }

        if !(self.food_exists)
        {
            self.add_food();
        }

        if (self.waiting_time > MOVING_PERIOD)
        {
            self.update_snake(None);
        }
    }

    fn check_eating(&mut self)
    {
        let (head_x, head_y): (i32, i32) = self.snake.head_position();
        if (self.food_exists && (self.food_x == head_x) && (self.food_y == head_y))
        {
            self.food_exists = false;
            self.snake.add_tail();
        }
    }

    fn check_if_snake_alive(&self, dir: Option<Direction>) -> bool
    {
        let (next_x, next_y) = self.snake.next_head(dir);

        if (self.snake.overlap_tail(next_x, next_y)) //If the snake collides into itself, it dies
        {
            return false;
        }

        return ((next_x > 0) && (next_y > 0) && (next_x < (self.width - 1)) && (next_y < (self.height - 1))); //Is the snake's next position within bounds?
    }

    fn add_food(&mut self)
    {
        let mut rng = thread_rng();

        let mut new_x = rng.gen_range(1..self.width - 1);
        let mut new_y = rng.gen_range(1..self.height - 1);

        while self.snake.overlap_tail(new_x, new_y)
        {
            new_x = rng.gen_range(1..self.width - 1);
            new_y = rng.gen_range(1..self.height - 1);
        }

        self.food_x = new_x;
        self.food_y = new_y;
        self.food_exists = true;
    }

    fn update_snake(&mut self, dir: Option<Direction>)
    {
        if self.check_if_snake_alive(dir)
        {
            self.snake.move_forward(dir);
            self.check_eating();
        }
        else 
        {
            self.game_over = true;
        }

        self.waiting_time = 0.0;
    }

    fn restart(&mut self)
    {
        self.snake = Snake::new(2, 2);
        self.waiting_time = 0.0;
        self.food_exists = true;
        self.food_x = 6;
        self.food_y = 4;
        self.game_over = false;
    }
}
