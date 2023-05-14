use std::thread;
use std::time::Duration;
use clearscreen;
use device_query::{DeviceQuery, DeviceState, Keycode};
use std::sync::{Arc, Mutex};
use std::sync::mpsc;

const WIDTH: usize = 13;
const HEIGHT: usize = 12;
const FILL: u8 = 0;

const TOY: u8 = 2;
const STATIC_TOY: u8 = 1;

const START_PST: u8 = 3;

const TIME_UPD: u64 = 1000;
const TIME_KEY: u64 = 80;

const LEFT:Keycode = Keycode::Left;
const RIGHT: Keycode = Keycode::Right;
const UP: Keycode = Keycode::Up;
const DOWN: Keycode = Keycode::Down;
const LE: i8 = -1;
const RI: i8 = 1;
const ROT: i8 = 5;
const DO: i8 = 3;

static GLOBAL_FLAG: Mutex<bool> = Mutex::new(false);
static LOOP_FLAG: Mutex<bool> = Mutex::new(false);
static NEW_TOY_FLAG: Mutex<bool> = Mutex::new(true);
static GLOBAL_TIME_UPD: Mutex<u64> = Mutex::new(TIME_UPD); //time screen update and level of hard game

fn main() {
    global_flag_change(true);
    loop_flag_change(true);
    toy_flag_change(true);
    //let (tx, rx) = mpsc::channel();                                //create channel thread_1 tread_2
    let mutex_pole = Arc::new(Mutex::new(vec![vec![FILL;WIDTH];HEIGHT]));
    let mut toy11: Vec<Vec<u8>> = vec![vec![FILL;2];3];
    toy11[0][1] = 2; toy11[1][0] = 2; toy11[1][1] = 2; toy11 [2][1] = 2;

    let toy1 = Arc::new(Mutex::new(toy11));
   

    let toy_arc = Arc::clone(&toy1);
    let mutex_arc = Arc::clone(&mutex_pole);
    let handle_one = thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_millis(TIME_KEY));
            let mut toy_k = toy_arc.lock().unwrap();
            let mut pole_k = mutex_arc.lock().unwrap();
            let mut num_key: i8 = key_status();     //читаем состояние клавиши
                                                           
            if global_flag_status() {     // разрешение на віполнение в момент обновления поля
                match num_key {
                    LE | RI  => {
                        loop_flag_change(false);
                        clearscreen::clear().expect("Tetris failed");
                        pole_print(update_screen(&mut pole_k, &mut num_key, &mut toy_k)); //меняем поле после нажатия клавиши сразу
                        loop_flag_change(true);      
                    },
                    ROT => {
                        loop_flag_change(false);
                        rotate_toy(&mut toy_k);
                        pole_print(update_screen(&mut pole_k, &mut num_key, &mut toy_k)); 
                        loop_flag_change(true);  
                    },
                    _=> {global_time_change(1)},
                }                                    
            }
        }
    });

    let toy_arc_2 = Arc::clone(&toy1);
    let mutex_arc_2 = Arc::clone(&mutex_pole);
    let handle_two = thread::spawn(move || {
        let mut count = 0;

        
        loop {
            thread::sleep(Duration::from_millis(global_time_status()));
            let mut toy_p = toy_arc_2.lock().unwrap();
            let mut pole_p = mutex_arc_2.lock().unwrap();
            let mut received_key: i8 = 0;

            if  toy_flag_status() {
                    count = 0;
                    pole_p[HEIGHT-1][0] = 1;
                    pole_p[HEIGHT-1][1] = START_PST;
                    add_toy_in_pole(&mut pole_p, &mut toy_p, 0, START_PST as usize);
                toy_flag_change(false);
            }
            if loop_flag_status() { 
                clearscreen::clear().expect("Tetris failed");
                pole_print(update_screen(&mut pole_p, &mut received_key, &mut toy_p));
            }
            if count != HEIGHT {
                count += 1;
            } else {count = 0}
            println!("{}", &count);
        }
    });

    handle_one.join().unwrap(); 
    handle_two.join().unwrap(); 
}

fn pole_print(pole_step:  Vec<Vec<u8>> ) {
        for element_cnt in &pole_step[0..pole_step.len()] {
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

fn update_screen(vc: &mut Vec<Vec<u8>>,key: &mut i8, toy: &mut Vec<Vec<u8>>) -> Vec<Vec<u8>> {

    global_flag_change(false);

    let mut key_stat: i8 = *key;
    let mut flag: bool = false;
    let mut last_toy_height = vc[HEIGHT-1][0] as usize;
    let mut last_toy_width = vc[HEIGHT-1][1] as usize;
   
    fn clear_toy(vc: &mut  Vec<Vec<u8>>) {
        for h in (0..(HEIGHT-1)).rev() {
            for w in 0..WIDTH {
                if vc[h][w] == TOY {
                    vc[h][w] = FILL 
                }
            }
        }
    }
    
    for hg in (0..(HEIGHT-1)).rev() {
        for wd in 0..WIDTH {

            if vc[hg][wd] == STATIC_TOY && vc[hg-1][wd] == TOY && (key_stat != LE || key_stat != RI) {
                flag = true; 
                toy_flag_change(true);
                loop_flag_change(false);
            } else if vc[hg][wd] == TOY && hg == (HEIGHT-2) {
                flag = true; 
                global_flag_change(false);
                loop_flag_change(false); 
            } else if vc[hg][wd] == TOY && wd - 1 == 0 && key_stat == LE {
                key_stat = 0;
            } else if  vc[hg][wd] == TOY  && wd + 1 == (WIDTH-1) && key_stat == RI {
                key_stat = 0;
            } else if  vc[hg][wd] == TOY && vc[hg][wd+1] == STATIC_TOY && key_stat == RI {
                key_stat = 0;
            } else if  vc[hg][wd] == TOY && vc[hg][wd-1] == STATIC_TOY && key_stat == LE {
                key_stat = 0;
            }
        }
    }
                    match key_stat {
                        LE | RI => {
                                clear_toy(vc);
                                let result = ((last_toy_width as i8) + key_stat) as usize;
                                add_toy_in_pole(vc, toy, last_toy_height, result);
                                vc[HEIGHT-1][1] = result as u8; 
                        },
                        _=> { 
                            if flag {
                                for h in (0..(HEIGHT-1)).rev() {
                                    for w in 0..WIDTH {
                                        if vc[h][w] == TOY {
                                            vc[h][w] = STATIC_TOY;
                                        }
                                    }
                                }
                                toy_flag_change(true);
                            } else {
                                clear_toy(vc);
                                add_toy_in_pole(vc, toy, last_toy_height, last_toy_width);
                                vc[HEIGHT-1][0] = (last_toy_height as u8) + 1; 
                            }
                        },
                    }

    vc.to_vec()
}

fn add_toy_in_pole(vc_upd: &mut Vec<Vec<u8>>, toy_to_add: &mut Vec<Vec<u8>>, count_iter: usize, start_width: usize) {
    
    let mut vc_height = count_iter.clone();
    let mut vc_width = start_width.clone();
    //thread::sleep(Duration::from_millis(5000));
    let toy_height: usize = toy_to_add.len() as usize;
    let toy_width: usize = toy_to_add[0].len() as usize;

    for h in 0..toy_height {
        for w in 0..toy_width {
            if toy_to_add[h][w] != FILL {
                vc_upd[vc_height][vc_width] = toy_to_add[h][w];
            }
            vc_width += 1;
        }
        vc_width = vc_width - toy_width;
        vc_height += 1;
    }
}

fn rotate_toy(rot_toy12: &mut Vec<Vec<u8>>) {

    let mut rot_toy = rot_toy12.clone(); 
    let height: usize = rot_toy.len() as usize;
    let width: usize = rot_toy[0].len() as usize;
    let mut arr_toy: Vec<Vec<u8>> = vec![vec![0; height]; width];

    for h in 0..height {
        for w in 0..width {
            arr_toy [width - w - 1][height - h - 1] = rot_toy[h][w];
        } 
    } 
    *rot_toy12 = arr_toy;
}

fn key_status() -> i8 {

    let device_state = DeviceState::new(); //read key status

    let keys: Vec<Keycode> = device_state.get_keys(); //get keys
    let mut position: i8 = 0;
    
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

fn global_time_change(value: i8) {
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