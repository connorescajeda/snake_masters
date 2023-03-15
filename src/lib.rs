#![cfg_attr(not(test), no_std)]

use bare_metal_modulo::{ModNumC, MNum, ModNumIterator};
use num::ToPrimitive;
use pluggable_interrupt_os::vga_buffer::{BUFFER_WIDTH, BUFFER_HEIGHT, plot, ColorCode, Color, is_drawable, plot_num};
use pc_keyboard::{DecodedKey, KeyCode};
use num::traits::SaturatingAdd;
use rand::{Rng, SeedableRng};
use rand::RngCore;
use rand::rngs::SmallRng;
use core::default::Default;




pub struct Game {
    player1: Player,
    player2: Player,
    food: Food,
}

impl Game {
    pub fn new() -> Self {
        Self {player1: Player::new(), player2: Player::new(), food: Food::new(25)}
        
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
        //self.walls.draw();
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

        plot('1', self.player1.x, self.player1.y, ColorCode::new(Color::Green, Color::Black));
        plot('2', self.player2.x, self.player2.y, ColorCode::new(Color::Blue, Color::Black));
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
        
        
        
    }
}

#[derive(Copy, Clone)]
pub struct Duple {
    x: usize,
    y: usize,
}
impl Duple{
    pub fn new(xt: usize, yt: usize) -> Self{
        Self{x:xt, y: yt}
    }
}
#[derive(Copy, Clone)]
pub struct Player {
    x: usize,
    y: usize,
    food_ate: usize,
    body: [Duple; 8000]
}

impl Player {
    pub fn new() -> Self {
        let mut small_rng = SmallRng::seed_from_u64(100000);
        let rand_x = small_rng.gen_range(5..BUFFER_WIDTH - 5);
        let rand_y = small_rng.gen_range(5..BUFFER_HEIGHT - 5);
        let mut body: [Duple; 8000] = [Duple::new(0, 0); 8000];
        for i in 0..8000{
            body[i] = Duple::new(rand_x, rand_y);
        }
        Self {x: rand_x.to_usize().unwrap(), y: rand_y.to_usize().unwrap(), food_ate: 0, body}
    }

    // pub fn is_colliding(&self, walls: &Walls) -> bool {
    //     // set up someway for this to check other players tail
    //     false
    // }
    
    

    pub fn eat(&mut self){
        self.food_ate +=1;
        }

    
    pub fn down(&mut self) {
        if self.y + 1 < BUFFER_HEIGHT {
            self.y += 1
        }
    }

    pub fn up(&mut self) {
        if self.y > 1 {
            self.y -= 1
        }
    }   

    pub fn left(&mut self) {
        if self.x > 0 {
            self.x -= 1
        }
    }

    pub fn right(&mut self) {
        if self.x + 1 < BUFFER_WIDTH {
            self.x += 1
        }
    }
}






// #[derive(Copy,Debug,Clone,Eq,PartialEq)]
// pub struct LetterMover {
//     letters: [char; BUFFER_WIDTH],
//     num_letters: ModNumC<usize, BUFFER_WIDTH>,
//     next_letter: ModNumC<usize, BUFFER_WIDTH>,
//     col: ModNumC<usize, BUFFER_WIDTH>,
//     row: ModNumC<usize, BUFFER_HEIGHT>,
//     dx: ModNumC<usize, BUFFER_WIDTH>,
//     dy: ModNumC<usize, BUFFER_HEIGHT>
// }



// impl LetterMover {
//     pub fn new() -> Self {
//         LetterMover {
//             letters: ['A'; BUFFER_WIDTH],
//             num_letters: ModNumC::new(1),
//             next_letter: ModNumC::new(1),
//             col: ModNumC::new(BUFFER_WIDTH / 2),
//             row: ModNumC::new(BUFFER_HEIGHT / 2),
//             dx: ModNumC::new(0),
//             dy: ModNumC::new(0)
//         }
//     }

//     fn letter_columns(&self) -> impl Iterator<Item=usize> {
//         ModNumIterator::new(self.col)
//             .take(self.num_letters.a())
//             .map(|m| m.a())
//     }

//     pub fn tick(&mut self) {
//         self.clear_current();
//         self.update_location();
//         self.draw_current();
//     }

//     fn clear_current(&self) {
//         for x in self.letter_columns() {
//             plot(' ', x, self.row.a(), ColorCode::new(Color::Black, Color::Black));
//         }
//     }

//     fn update_location(&mut self) {
//         self.col += self.dx;
//         self.row += self.dy;
//     }

//     fn draw_current(&self) {
//         for (i, x) in self.letter_columns().enumerate() {
//             plot(self.letters[i], x, self.row.a(), ColorCode::new(Color::Cyan, Color::Black));
//         }
//     }

//     pub fn key(&mut self, key: DecodedKey) {
//         match key {
//             DecodedKey::RawKey(code) => self.handle_raw(code),
//             DecodedKey::Unicode(c) => self.handle_unicode(c)
//         }
//     }

//     fn handle_raw(&mut self, key: KeyCode) {
//         match key {
//             KeyCode::ArrowLeft => {
//                 self.dx -= 1;
//             }
//             KeyCode::ArrowRight => {
//                 self.dx += 1;
//             }
//             KeyCode::ArrowUp => {
//                 self.dy -= 1;
//             }
//             KeyCode::ArrowDown => {
//                 self.dy += 1;
//             }
//             _ => {}
//         }
//     }

    // fn handle_unicode(&mut self, key: char) {
    //     if is_drawable(key) {
    //         self.letters[self.next_letter.a()] = key;
    //         self.next_letter += 1;
    //         self.num_letters = self.num_letters.saturating_add(&ModNumC::new(1));
    //     }
    // }
// }

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
