use std::thread;
use std::time::Duration;
use clearscreen;
use device_query::{DeviceQuery, DeviceState, Keycode};
use std::sync::{Arc, Mutex};
use std::sync::mpsc;

const WIDTH: usize = 12;
const HEIGHT: usize = 11;
const FILL: i8 = 0;

const STATIC_TOY: i8 = 1;
const TOY: i8 = 2;
const START_PST: usize = 3;

const TIME_UPD: u64 = 400;
const TIME_KEY: u64 = 80;

const LEFT:Keycode = Keycode::A;
const RIGHT: Keycode = Keycode::D;
const UP: Keycode = Keycode::W;
const DOWN: Keycode = Keycode::S;

static GLOBAL_FLAG: Mutex<bool> = Mutex::new(false);
static NEW_TOY_FLAG: Mutex<bool> = Mutex::new(true);
static GLOBAL_TIME_UPD: Mutex<u64> = Mutex::new(TIME_UPD); //time screen update and level of hard game

//const LEFT_B: u8 = 01000001;

fn main() {
        //let mut pole_buffer = pole.clone();
    let (tx, rx) = mpsc::channel(); //create channel thread_1 tread_2
    let mut mutex_pole = Arc::new(Mutex::new(vec![vec![FILL;WIDTH];HEIGHT]));
    
    let mutex_arc = Arc::clone(&mutex_pole);
    let handle_one = thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_millis(TIME_KEY));
            

            let mut pole_k = mutex_arc.lock().unwrap();
            let mut m: u8 = key_status();
            let n = m.clone();

            tx.send(n).unwrap(); //send data for thread_2
            
            if m > 0 {
                global_flag_change(true);
                clearscreen::clear().expect("Tetris failed");
                pole_print(update_screen(&mut pole_k, &mut m));
            }
        }
    });

    let mutex_arc_2 = Arc::clone(&mutex_pole);
    let handle_two = thread::spawn(move || {
        loop {
            if global_flag_status() {
                thread::sleep(Duration::from_millis(500));
                global_flag_change(false);
            } 

            let force_key: u8 = rx.recv().unwrap();
            if force_key == 4 {
                global_time_change(force_key);
            } else {global_time_change(1)}

            thread::sleep(Duration::from_millis(global_time_status()));
            let mut pole_p = mutex_arc_2.lock().unwrap();
            let mut received_key: u8 = 0;
            if  toy_flag_status() {
                *&mut pole_p[0][START_PST] = TOY; // it s change to random TOY after code update
                toy_flag_change(false);
            }
            clearscreen::clear().expect("Tetris failed");
            pole_print(update_screen(&mut pole_p, &mut received_key));
            thread::sleep(Duration::from_millis(global_time_status()));
            
        }
    });
   
    handle_one.join().unwrap(); 
    handle_two.join().unwrap(); 
}

fn pole_print(pole_step:  Vec<Vec<i8>> ) {
        for element_cnt in &pole_step[0..(HEIGHT-1)] {
            for y in element_cnt {
                print!("{:?}", y);
            }
        println!();
        }
    for _n in 1..5  {
         println!();
    }
}

fn update_screen(vc: &mut Vec<Vec<i8>>,key: &mut u8) -> Vec<Vec<i8>> {

    //global_flag_change(false);
    

    'count: for h in 0..(HEIGHT - 1) { //update_pixel height
            
        for w in 0..(WIDTH - 1) { //update_pixel width
            match vc[h][w] {
                FILL => vc[h][w] = 0,
                STATIC_TOY => vc[h][w] = 1,
                TOY => {
                    //println!{"{}", &key};
                    match key {
                        1 => {
                            if vc[h][w-1] != STATIC_TOY {
                                vc[h][w] = FILL; vc[h][w-1] = TOY;
                            }
                        },
                        2 => {
                            if vc[h][w+1] != STATIC_TOY {
                                vc[h][w] = FILL; vc[h][w+1] = TOY;
                            }
                        },
                        _=> {
                            if STATIC_TOY != vc[h+1][w] && h != (HEIGHT-2) {
                                vc[h][w] = FILL;
                                vc[h+1][w] = TOY;
                            } else {
                                vc[h][w] = STATIC_TOY;
                                toy_flag_change(true);
                            }
                        },
                    }
                break 'count;
                },
                _ => println!("Tetris Failed_update_screen"),
                }
        }
    }
    //global_flag_change(true);
    vc.to_vec()
}

fn key_status() -> u8 {

        let device_state = DeviceState::new(); //read key status

        let keys: Vec<Keycode> = device_state.get_keys(); //get keys
        let mut position: u8 = 0;
        
        if device_state.get_keys().len() > 0 { // get position toy
            match keys[0] {
                LEFT => position += 1,
                RIGHT => position += 2,
                UP => position += 3,
                DOWN => position += 5,
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
    let global_flag = {
        let global_flag_status = GLOBAL_FLAG.lock().unwrap();
        *global_flag_status
    };
    global_flag
}

fn global_time_change(value: u8) {
    let mut time_flag = GLOBAL_TIME_UPD.lock().unwrap();
    *time_flag = TIME_UPD / value as u64;
}

fn global_time_status() -> u64 {
    let time_flag = {
        let time_flag_status = GLOBAL_TIME_UPD.lock().unwrap();
        *time_flag_status
    };
    time_flag
}