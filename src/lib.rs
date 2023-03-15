#![cfg_attr(not(test), no_std)]

use bare_metal_modulo::{ModNumC, MNum, ModNumIterator};
use pluggable_interrupt_os::vga_buffer::{BUFFER_WIDTH, BUFFER_HEIGHT, plot, ColorCode, Color, is_drawable};
use pc_keyboard::{DecodedKey, KeyCode};
use num::traits::SaturatingAdd;


pub struct Game {
    player1: Player,
    player2: Player,
}

impl Game {
    pub fn new() -> Self {
        Self {player1: Player::new(), player2: Player::new()}
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
            DecodedKey::Unicode(_) => {}
        }
    }

    pub fn tick(&mut self) {
        //self.walls.draw();
        plot('*', self.player1.x, self.player1.y, ColorCode::new(Color::Green, Color::Black));
    }
}

#[derive(Copy, Clone)]
pub struct Player {
    x: usize,
    y: usize,
}

impl Player {
    pub fn new() -> Self {
        Self {x: BUFFER_WIDTH / 2, y: BUFFER_HEIGHT / 2}
    }

    // pub fn is_colliding(&self, walls: &Walls) -> bool {
    //     // set up someway for this to check other players tail
    //     false
    // }

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

//     fn handle_unicode(&mut self, key: char) {
//         if is_drawable(key) {
//             self.letters[self.next_letter.a()] = key;
//             self.next_letter += 1;
//             self.num_letters = self.num_letters.saturating_add(&ModNumC::new(1));
//         }
//     }
// }