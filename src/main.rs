use std::thread;
use std::time::Duration;
use clearscreen;
use device_query::{DeviceQuery, DeviceState, Keycode};
use std::sync::{Arc, Mutex};

const WIDTH: usize = 5;
const HEIGHT: usize = 15;
const FILL: i8 = 0;

const TOY: i8 = 1;
const START_PST: usize = 3;

const TIME_UPD: u64 = 700; //time screen update and level of hard game
const TIME_KEY: u64 = 500;

const LEFT:Keycode = Keycode::A;
const RIGHT: Keycode = Keycode::D;
const UP: Keycode = Keycode::W;
const DOWN: Keycode = Keycode::S;

static GLOBAL_FLAG: Mutex<bool> = Mutex::new(false);
static NEW_TOY_FLAG: Mutex<bool> = Mutex::new(true);

//const LEFT_B: u8 = 01000001;

fn main() {
        //let mut pole_buffer = pole.clone();
    let mut mutex_pole = Mutex::new(vec![vec![FILL;WIDTH];HEIGHT]);
     
    let handle = thread::spawn(move || {
        loop {

            if global_flag_status() {
            let m: u8 = key_status();
            }
        }
    });

    let handle_two = thread::spawn(move||{
        let mut mutex = mutex_pole.lock().unwrap();
        loop {
            if  toy_flag_status() {
                mutex[0][START_PST] = TOY; // it s change to random TOY after code update
                toy_flag_change(false);
            }

            pole_print(update_screen(&mut mutex));
            //drop(mutex);
        }
    });
    handle.join().unwrap(); 
    handle_two.join().unwrap(); 
}

fn pole_print(pole_step:  Vec<Vec<i8>> ) {

        for element_cnt in &pole_step[0..(HEIGHT-1)] {
            for y in element_cnt {
            print!("{:?} ", y);
            }
        println!();
        }
    thread::sleep(Duration::from_millis(TIME_UPD)); // sleep to update pole
    clearscreen::clear().expect("Tetris failed");   // clear screen
}

fn update_screen(vc: &mut Vec<Vec<i8>>) -> Vec<Vec<i8>> {

    global_flag_change(false);

    'count: for h in 0..(HEIGHT - 1) { //update_pixel height
            
                for w in 0..(WIDTH - 1) { //update_pixel width
                    match vc[h][w] {
                        FILL => vc[h][w] = 0,
                        TOY => {
                            if vc[h][w] != vc[h+1][w] && h != (HEIGHT - 2) {
                                vc[h][w] = FILL;
                                vc[h+1][w] = TOY;
                            } else {
                                vc[h][w] = TOY;
                                toy_flag_change(true);
                            }
                          break 'count;
                        },
                        _ => println!("Tetris Failed_update_screen"),
                        }
                    }
                }
    global_flag_change(true);
    vc.to_vec()
}

fn key_status() -> u8 {

        let device_state = DeviceState::new(); //read key status

        thread::sleep(Duration::from_millis(TIME_KEY));

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

fn toy_flag_change(value: bool) {
    let mut flag = NEW_TOY_FLAG.lock().unwrap();
    *flag = value;
}

fn toy_flag_status() -> bool {
    let toys_flag = {
        let toys_flag_status = NEW_TOY_FLAG.lock().unwrap();
        *toys_flag_status
    };
    toys_flag
}

fn global_flag_change(value: bool) {
    let mut flag = GLOBAL_FLAG.lock().unwrap();
    *flag = value;
}

fn global_flag_status() -> bool {
    let toys_flag = {
        let toys_flag_status = GLOBAL_FLAG.lock().unwrap();
        *toys_flag_status
    };
    toys_flag
}