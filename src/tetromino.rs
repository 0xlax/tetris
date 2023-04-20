// this crate has the tetromino shapes initialisationl

use rand::Rng;
use rand::thread_rng;



// 7 shapes
pub enum Tetromino {
    I,
    J,
    L,
    O,
    S,
    T,
    Z
}

impl Tetromino {

    // Get random shapes using rand crate - Thread safe Random number ganerater
    pub fn random() -> Tetromino {
        match rand::thread_rng().gen_range(0,7) {
            0 => return Self::I, 
            1 => return Self::J, 
            2 => return Self::L, 
            3 => return Self::O, 
            4 => return Self::S, 
            5 => return Self::T, 
            6 => return Self::Z, 
        }

    } //random


}