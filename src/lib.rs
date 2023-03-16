#![cfg_attr(not(test), no_std)]
#![feature(const_trait_impl)]


use bare_metal_modulo::{ModNumC, MNum, ModNumIterator};
use num::ToPrimitive;
use pluggable_interrupt_os::vga_buffer::{BUFFER_WIDTH, BUFFER_HEIGHT, plot, ColorCode, Color, is_drawable, plot_num, plot_str};
use pc_keyboard::{DecodedKey, KeyCode};
use num::traits::SaturatingAdd;
use rand::{Rng, SeedableRng};
use rand::RngCore;
use rand::rngs::SmallRng;
use core::default::Default;
use core::clone::Clone;
use core::marker::Copy;
use core::iter::Iterator;




pub struct Game {
    player1: Player,
    player2: Player,
    food: Food,
    grid: refresh,
    tick_count: isize
}

impl Game {
    pub fn new() -> Self {
        Self {player1: Player::new(50), player2: Player::new(100), food: Food::new(25), grid: refresh::new(), tick_count: 0}
    }

    pub fn key(&mut self, key: DecodedKey) {
        match key {
            DecodedKey::RawKey(key) => {
                match key {
                    KeyCode::ArrowDown => {
                        self.player1.down()
                    }
                    KeyCode::ArrowUp => {
                        self.player1.up()
                    } 
                    KeyCode::ArrowLeft => {
                        self.player1.left()
                    }
                    KeyCode::ArrowRight => {
                        self.player1.right()
                    }
                    
                    _ => {}
                }     
            }
            DecodedKey::Unicode(key) => {
                match key {
                    's' => {
                        self.player2.down()
                        }
                    'w' => {
                        self.player2.up()
                        }
                    'a' => {
                        self.player2.left()
                        }
                    'd' => {
                        self.player2.right()
                        }
                    _ => {}
                } 
            }
        }
    }
    
    pub fn tick(&mut self) {
        self.grid.draw();

        if self.food.total_food < self.food.max_food{
            self.food.add_food();
        }
        for col in 0..BUFFER_WIDTH-1{
            for row in 0..BUFFER_HEIGHT-1{
                
                if self.food.food_map[col][row]{
                    plot('f', col, row, ColorCode::new(self.food.color, Color::Black));
                }
            }
        }
        if self.player1.x == self.player2.x && self.player2.y == self.player1.y {
            plot_str("FREAKING LOSER", 30, 0, ColorCode::new(Color::LightRed, Color::Black));
        }
        self.player1.update_location(self.tick_count);
        self.player2.update_location(self.tick_count);
        
        

        plot('1', self.player1.x, self.player1.y, ColorCode::new(Color::Green, Color::Black));
        plot('2', self.player2.x, self.player2.y, ColorCode::new(Color::Blue, Color::Black));
        for i in 0..self.player1.food_ate+1{
            
            plot('1', self.player1.body[i].x, self.player1.body[i].y, ColorCode::new(Color::Green, Color::Black));
        }
        for i in 0..self.player2.food_ate+1{
            
            plot('2', self.player2.body[i].x, self.player2.body[i].y, ColorCode::new(Color::Blue, Color::Black));
        }
        
        
        if (self.food.food_map[self.player1.x][self.player1.y]){
            self.food.food_map[self.player1.x][self.player1.y] = false;
            self.food.add_food();
            self.player1.eat();
        }
        if (self.food.food_map[self.player2.x][self.player2.y]){
            self.food.food_map[self.player2.x][self.player2.y] = false;
            self.food.add_food();
            self.player2.eat();
        }
        if (self.player1.has_moved || self.player2.has_moved){
            if self.player1.check_collisions(self.player2) && self.player2.check_collisions(self.player1) {
                plot_str("YOU BOTHER ARE FREAKIN LOSERS", 30, 0, ColorCode::new(Color::LightRed, Color::Black));
            }
            else if self.player1.check_collisions(self.player2) {
                plot_str("PLAYER 1 IS A LOOOOSER", 30, 0, ColorCode::new(Color::LightRed, Color::Black));
            }
            else if self.player2.check_collisions(self.player1) {
                plot_str("PLAYER 2 IS A LOOOOSER", 30, 0, ColorCode::new(Color::LightRed, Color::Black));
            }
        }

        
        
        self.tick_count += 1
    }
    

    
}

pub struct refresh {
    grid : [[bool; BUFFER_WIDTH]; BUFFER_HEIGHT],
    color: Color
}

impl refresh {
    pub fn new() -> Self {
        let mut grid = [[false; BUFFER_WIDTH]; BUFFER_HEIGHT];
        
        Self {grid, color: Color::Black}
    }

    fn char_at(&self, row: usize, col: usize) -> char {
        if self.grid[row][col] {
            '#'
        } else {
            ' '
        }
    }
    fn draw(&self) {
        for row in 0..self.grid.len() {
            for col in 0..self.grid[row].len(){
                plot(self.char_at(row, col,), col, row, ColorCode::new(Color::Red, Color::Black));
            }
        }
    }
}

pub struct Duple {
    x: usize,
    y: usize,
}
impl Copy for Duple {}
impl Clone for Duple{
    fn clone(&self)-> Duple {
        *self
    }

    fn clone_from(&mut self, source: &Self)
    where
        Self: ~const core::marker::Destruct,
    {
        *self = source.clone()
    }
}
impl Default for Duple{
    fn default() -> Self {
        Self{x:0, y: 0}
    }
}
impl Duple{
    pub fn new(xt: usize, yt: usize) -> Self{
        Self{x: xt, y: yt}
    }
}
#[derive(Copy, Clone)]
pub struct Player {
    x: usize,
    y: usize,
    direction: char,
    food_ate: usize,
    body: [Duple; 8000],
    has_moved: bool,
}

impl Player {
    pub fn new(state : u64) -> Self {
        let mut small_rng = SmallRng::seed_from_u64(state);
        let x = small_rng.next_u64() as usize % BUFFER_WIDTH ; 
        let y = small_rng.next_u64() as usize % BUFFER_HEIGHT;
        let mut body: [Duple; 8000] = [Duple::new(0, 0); 8000];

        Self {x, y , food_ate: 0, body, has_moved: false}
        
    }

    // pub fn is_colliding(&self, walls: &Walls) -> bool {
    //     // set up someway for this to check other players tail
    //     false
    // }
    
    pub fn eat(&mut self){
        self.food_ate +=1;
    }

    pub fn check_collisions(&mut self, op: Player) -> bool{
        for (spot,dup) in op.body.iter().enumerate(){
            if spot <= op.food_ate{
                if self.body[0].x == dup.x && self.body[0].y == dup.y{
                    return true;
                }
            }   
        }
        return false;
    }
    pub fn down(&mut self) {
        self.has_moved = true;
        if self.y + 1 < BUFFER_HEIGHT {
            self.direction = 'd';
            self.y += 1;
            let mut temp: &Duple = &Duple::new(0,0);
            let mut temp2: &Duple = &Duple::new(0,0);

            let mut tempbod = self.body.clone();
            for (spot, dup) in tempbod.iter().enumerate(){
                if spot==0{
                    temp = dup;
                    self.body[0] = Duple::new(self.x, self.y).clone();
                }
                else{
                    temp2 = dup;
                    self.body[spot] = *temp;
                    temp = temp2;
                }
            }
            
        }
    }

    pub fn up(&mut self) {
        self.has_moved = true;
        if self.y > 1 {
            self.direction = 'u';
            self.y -= 1;
            let mut temp: &Duple = &Duple::new(0,0);
            let mut temp2: &Duple = &Duple::new(0,0);

            let mut tempbod = self.body.clone();
            for (spot, dup) in tempbod.iter().enumerate(){
                if spot==0{
                    temp = dup;
                    self.body[0] = Duple::new(self.x, self.y).clone();
                }
                else{
                    temp2 = dup;
                    self.body[spot] = *temp;
                    temp = temp2;
                }
            }
        }
    }   

    pub fn left(&mut self) {
        self.has_moved = true;
        if self.x > 0 {
            self.direction = 'l';
            self.x -= 1;
            let mut temp: &Duple = &Duple::new(0,0);
            let mut temp2: &Duple = &Duple::new(0,0);

            let mut tempbod = self.body.clone();
            for (spot, dup) in tempbod.iter().enumerate(){
                if spot==0{
                    temp = dup;
                    self.body[0] = Duple::new(self.x, self.y).clone();
                }
                else{
                    temp2 = dup;
                    self.body[spot] = *temp;
                    temp = temp2;
                }
            }
        }
    }

    pub fn right(&mut self) {
        self.has_moved = true;
        if self.x + 1 < BUFFER_WIDTH {
            self.direction = 'r';
            self.x += 1;
            let mut temp: &Duple = &Duple::new(0,0);
            let mut temp2: &Duple = &Duple::new(0,0);

            let mut tempbod = self.body.clone();
            for (spot, dup) in tempbod.iter().enumerate(){
                if spot==0{
                    temp = dup;
                    self.body[0] = Duple::new(self.x, self.y).clone();
                }
                else{
                    temp2 = dup;
                    self.body[spot] = *temp;
                    temp = temp2;
                }
            }
        }
    }

    fn update_location(&mut self, tick_count : isize) {
        //if tick_count % 3 == 0 {
            if self.direction == 'r' {
                self.right();
            } else if self.direction == 'l' {
                self.left();
            } else if self.direction == 'd' {
                self.down()
            }  else if  self.direction == 'u' {
                self.up();
            }  
       // }
        
    }
    
    
}


pub struct Food{
    food_map: [[bool; BUFFER_HEIGHT]; BUFFER_WIDTH],
    color: Color,
    total_food: usize,
    max_food: usize,
    rng: SmallRng,
}

impl Food {
    pub fn new(max: usize) -> Self{
        let mut temp = [[false; BUFFER_HEIGHT];BUFFER_WIDTH];
        let c = Color::White;
        Self {food_map: temp, color: c, total_food: 0, max_food: max, rng: SmallRng::seed_from_u64(42)}
        
    }

    pub fn add_food(&mut self){
        let col: usize = 1+ self.rng.next_u32() as usize % (BUFFER_WIDTH - 1);
        let row: usize = 1 + self.rng.next_u32() as usize % (BUFFER_HEIGHT -1);
        while true{
            if !self.food_map[col][row]{
                self.food_map[col][row] = true;
                self.total_food +=1;
                break;
            }
        }
    }
}
