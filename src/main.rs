use std::{io, thread};
use std::time::Duration;
use clearscreen;
use device_query::{DeviceQuery, DeviceState, Keycode};

const WIDTH: usize = 7;
const HEIGHT: usize = 10;
const FILL: i8 = 0;

const TOY: i8 = 1;
const START_PST: usize = 3;

const TIME_UPD: u64 = 800; //time screen update and level of hard game
const TIME_KEY: u64 = 500;

const LEFT:Keycode = Keycode::A;
const RIGHT: Keycode = Keycode::D;
const UP: Keycode = Keycode::W;
const DOWN: Keycode = Keycode::S;
//const LEFT_B: u8 = 01000001;

fn main () {

    let mut pole: Vec<Vec<i8>> = vec![vec![FILL;WIDTH];HEIGHT];
    let mut pole_variable: Vec<Vec<i8>>; 
/*
    let handle = thread::spawn(move || {
        loop {
        let m: u8 = key_status();
        println!("{}", m);
        }
    });
*/
    loop {
        pole[0][START_PST] = TOY;
        
        for string_cnt in 0..HEIGHT { //print vector on terminal
                pole_variable = update_screen(&mut pole); 
                
                for x in &pole_variable {
                    for y in x {
                    print!("{} ", y);
                    }
                println!();
                }
                
                pole = pole_variable;    //update pole on last iteration
                thread::sleep(Duration::from_millis(TIME_UPD)); // sleep to update pole
                clearscreen::clear().expect("Tetris failed");
                
        }
    }
    //handle.join().unwrap(); 
}

fn pole() {

}

fn update_screen(vc: &mut Vec<Vec<i8>>) -> Vec<Vec<i8>> {
   
    'count: for h in 0..(HEIGHT - 1) { //update_pixel height
            
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



fn key_status() -> u8 {

        let device_state = DeviceState::new(); //read key status

        thread::sleep(Duration::from_millis(300));

        let keys: Vec<Keycode> = device_state.get_keys(); //get keys
        let mut position: u8 = 0;
        
        if device_state.get_keys().len() > 0 { // get position toy
            match keys[0] {
                LEFT => position += 1,
                RIGHT => position += 2,
                UP => position += 3,
                DOWN => position += 4,
                _ => {},
            }
        }
    position
}
