use std::thread;
use std::time::Duration;
use clearscreen;
use device_query::{DeviceQuery, DeviceState, Keycode};
use std::sync::{Arc, Mutex};
use std::sync::mpsc;

const WIDTH: usize = 13;
const HEIGHT: usize = 14;
const FILL: u8 = 0;

const TOY: u8 = 2;
const STATIC_TOY: u8 = 1;

const START_PST: usize = 4;

const TIME_UPD: u64 = 500;
const TIME_KEY: u64 = 80;

const LEFT:Keycode = Keycode::Left;
const RIGHT: Keycode = Keycode::Right;
const UP: Keycode = Keycode::Up;
const DOWN: Keycode = Keycode::Down;
const LE: u8 = 1;
const RI: u8 = 2;
const ROT: u8 = 5;
const DO: u8 = 10;

static GLOBAL_FLAG: Mutex<bool> = Mutex::new(false);
static LOOP_FLAG: Mutex<bool> = Mutex::new(false);
static NEW_TOY_FLAG: Mutex<bool> = Mutex::new(true);
static GLOBAL_TIME_UPD: Mutex<u64> = Mutex::new(TIME_UPD); //time screen update and level of hard game

fn main() {
    global_flag_change(true);
    loop_flag_change(true);
    //let (tx, rx) = mpsc::channel();                                //create channel thread_1 tread_2

    let mut mutex_pole = Arc::new(Mutex::new(vec![vec![FILL;WIDTH];HEIGHT]));
    
    let mut mutex_arc = Arc::clone(&mutex_pole);
    let handle_one = thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_millis(TIME_KEY));
            let mut pole_k = mutex_arc.lock().unwrap();

            let mut m: u8 = key_status();     //читаем состояние клавиши
                               
                                                           
            if global_flag_status() {     // разрешение на віполнение в момент обновления поля
                match m {
                    LE | RI => {
                        loop_flag_change(false);
                        clearscreen::clear().expect("Tetris failed");
                        pole_print(update_screen(&mut pole_k, &mut m)); //меняем поле после нажатия клавиши сразу
                        loop_flag_change(true);      
                    },
                    ROT => {},
                    DO => global_time_change(m),
                    _=> {global_time_change(1)},
                }                                    
            }
        }
    });

    let mutex_arc_2 = Arc::clone(&mutex_pole);
    let handle_two = thread::spawn(move || {
        let mut count = 0;
        
        loop {
            thread::sleep(Duration::from_millis(global_time_status()));
            let mut pole_p = mutex_arc_2.lock().unwrap();
            let mut received_key: u8 = 0;

            if  toy_flag_status() {
                *&mut pole_p[0][START_PST] = TOY;
                *&mut pole_p[0][START_PST+1] = TOY;// it s change to random TOY after code update
                *&mut pole_p[0][START_PST+2] = TOY;
                //*&mut pole_p[1][START_PST+2] = TOY;
                toy_flag_change(false);
            }
            if loop_flag_status() { 
                clearscreen::clear().expect("Tetris failed");
                pole_print(update_screen(&mut pole_p, &mut received_key));
            }
            count += 1;
            println!("{}", &count);
        }
    });

    handle_one.join().unwrap(); 
    handle_two.join().unwrap(); 
}

fn pole_print(pole_step:  Vec<Vec<u8>> ) {
        for element_cnt in &pole_step[0..HEIGHT] {
            for y in element_cnt {
                if *y == 0 {
                    let ch: char = '-';
                    print!("{}", ch);
                } else {print!("{:?}", y)}
            }
        println!();
        }
    for _n in 1..5  {
         println!();
    }
    global_flag_change(true);
    loop_flag_change(true);
}

fn update_screen(vc: &mut Vec<Vec<u8>>,key: &mut u8) -> Vec<Vec<u8>> {
    //global_flag_change(false);
    let mut key_stat: u8 = *key;
    let mut flag: bool = false;
     
    for hg in 0..HEIGHT {
        for wd in 0..WIDTH {
            if vc[hg][wd] == STATIC_TOY && vc[hg-1][wd] == TOY {
                flag = true; 
                global_flag_change(false);
                loop_flag_change(false);
            }
        }
    }

    'count_h: for h in (0..HEIGHT).rev() { //update_pixel height
            
        'count_w: for w in 0..WIDTH { //update_pixel width
            match vc[h][w] {
                FILL => vc[h][w] = 0,
                STATIC_TOY => {
                    if vc[h][w+1] == TOY && key_stat == 1 {
                        key_stat = 0;
                    }
                    if vc[h][w-1] == TOY && key_stat == 2 {
                        key_stat = 0;
                    }
                    if vc[h-1][w] == TOY && key_stat != LE && key_stat != RI {
                        vc[h-1][w] = STATIC_TOY;
                        toy_flag_change(true);
                    }
                        
                },
                TOY => {
                    if flag && key_stat != LE && key_stat != RI  {
                        vc[h][w] = STATIC_TOY;
                        global_flag_change(false);
                    } 
                    match key_stat {
                        LE => {
                            if w != 1 && vc[h][w-1] != TOY {
                                vc[h][w] = FILL;
                                vc[h][w-1] = TOY;
                            }
                        },
                        RI => {
                            for d in (0..WIDTH).rev() {
                                if vc[h][d] == TOY && d != WIDTH-2 && vc[h][d+1] != TOY {
                                    if vc[h][d+1] == STATIC_TOY { 
                                        break 'count_w;
                                    } else {
                                        vc[h][d] = FILL;
                                        vc[h][d+1] = TOY;
                                    } 
                                } 
                            }
                            break 'count_w;
                        },
                        _=> {
                            if h != (HEIGHT-1) && flag != true {
                                vc[h][w] = FILL;
                                vc[h+1][w] = TOY;
                            } else {
                                vc[h][w] = STATIC_TOY;
                                toy_flag_change(true);
                            }
                        },
                    }
                },
                _ => println!("Tetris Failed_update_screen"),
            }
        }
    }
    vc.to_vec()
}

fn key_status() -> u8 {

        let device_state = DeviceState::new(); //read key status

        let keys: Vec<Keycode> = device_state.get_keys(); //get keys
        let mut position: u8 = 0;
        
        if device_state.get_keys().len() > 0 { // get position toy
            match keys[0] {
                LEFT => position += LE,
                RIGHT => position += RI,
                UP => position += ROT,
                DOWN => position += DO,
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

fn loop_flag_change(value: bool) {                      //изминения флага на общий луп
    let mut flag = LOOP_FLAG.lock().unwrap();
    *flag = value;
}

fn loop_flag_status() -> bool {                         //статус разрешения на общий луп
    let global_flag = {
        let global_flag_status = LOOP_FLAG.lock().unwrap();
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

