use std::{io, thread};
use std::time::Duration;
use clearscreen;

const WIDTH: usize = 7;
const HEIGHT: usize = 7;
const FILL: i8 = 0;
const TOY: i8 = 1;
const TIME_UPD: u64 = 300; //time screen update and level of hard game
const TIME_KEY: u64 = 300;

fn main () {

    let mut pole: Vec<Vec<i8>> = vec![vec![FILL;WIDTH];HEIGHT];
    let mut pole_variable: Vec<Vec<i8>>; 
   // let mut string_cnt: usize = 0; //not use 
    
    
    loop {
        pole[0][3] = TOY;

        for string_cnt in 0..HEIGHT { //print vector on terminal
                pole_variable = update_screen(&mut pole); 

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

    //let vc_to: &Vec<Vec<char>> = &vc.clone();
    //let mut width = vc_to.len() - 1;
    //let mut height = vc_to[0].len() - 1;
    //let block: char = TOY;
    let mut status_key: String = String::new();
   
    'count: for h in 0..(HEIGHT - 1) {

                for w in 0..(WIDTH - 1) {
                    //println!("{}", &w);
                    match *&vc[h][w] {
                        FILL => vc[h][w] = 0,
                        TOY => {
                          //  thread::sleep(Duration::from_millis(TIME_KEY));
                            
                          //  io::stdin().read_line(&mut status_key);
                          //  println!("{}", &status_key);
                            //if status_key[0] == 'a' {
                             //   w -= 1;
                            //}

                            if vc[h][w] != vc[h+1][w] {
                                vc[h][w] = FILL;
                                vc[h+1][w] = TOY;
                            } else {vc[h][w] = TOY}
                          break 'count;
                        },
                        _ =>{
                           // println!("Tetris Failed_update");
                        }
                    }
                }
            }
    //println!("{}", height_clone);
    return vc.to_vec()
}

fn key_status() -> String {

    //thread::sleep(Duration::from_millis(TIME_KEY));
    let mut status_key: String = String::new();

    match io::stdin().read_line(&mut status_key) {
        Ok(_) => {},
        _ => {
          println!("push key < or >");      
        },
    }
    status_key
}