use std::{io, thread};
use std::time::Duration;
use clearscreen;

const WIDTH: usize = 7;
const HEIGHT: usize = 7;
const FILL: i8 = 0;
const TOY: i8 = 1;
const TIME_UPD: u64 = 3300; //time screen update and level of hard game
const TIME_KEY: u64 = 300;
const LEFT: char = 'a';
const RIGHT: char = 'd';
const UP: char = 'w';
const DOWN: char = 's';

fn main () {

    let mut pole: Vec<Vec<i8>> = vec![vec![FILL;WIDTH];HEIGHT];
    let mut pole_variable: Vec<Vec<i8>>; 

    loop {
        pole[0][3] = TOY;  //function for generate TOY (update next...)

        for string_cnt in 0..HEIGHT { //print vector on terminal
                //pole_variable = update_screen(&mut pole); 

                for x in &pole_variable {
                    for y in x {
                    print!("{} ", y);
                    }
                println!();
                }
                
                pole = pole_variable; //update pole on last iteration
                thread::sleep(Duration::from_millis(TIME_UPD)); // sleep to update pole
                clearscreen::clear().expect("Tetris failed");
        }
    }
}

fn update_screen(vc: &mut Vec<Vec<i8>>) -> Vec<Vec<i8>> {

    let mut status_key: String = String::new();
   
    'count: for h in 0..(HEIGHT - 1) { //update_pixel height
            let m: i32 = key_status();
            
                for w in 0..(WIDTH - 1) { //update_pixel width
                    match *&vc[h][w] {
                        FILL => vc[h][w] = 0,
                        TOY => {
                            if vc[h][w] != vc[h+1][w] {
                                vc[h][w] = FILL;
                                vc[h+1][w] = TOY;
                            } else {vc[h][w] = TOY}
                          break 'count;
                        },
                        _ => println!("Tetris Failed_update_screen"),
                        }
                    }
                }

    return vc.to_vec()
}

fn key_status() -> i32 {

    let mut status_key: String = String::new();
    io::stdin().read_line(&mut status_key); //{

    let key: Vec<char> = status_key.chars().collect();
    let mut position: i32 = 0;

    match key[0] {
        'a' => position += 1,
        'd' => position += 2,
        'w' => position += 3,
        's' => position += 4,
        _ => println!{"Err read key"},
    };

    position
}