use std::thread;
use std::io;
use std::time::Duration;
use clearscreen;
use device_query::{DeviceQuery, DeviceState, Keycode};
use std::sync::{Arc, Mutex};
use rand::Rng;
use termios::{Termios, ECHO, ICANON, TCSANOW};

const WIDTH: usize = 11; // don't use WIDTH smallest 5 
const HEIGHT: usize = 24;
const FILL: u8 = 0;

const TOY: u8 = 2;
const STATIC_TOY: u8 = 1;

const START_PST: u8 = 3;

const TIME_UPD: u64 = 700;
const TIME_KEY: u64 = 95;
const TRUE_CHANGE: usize = 6;

const LEFT:Keycode = Keycode::Left;
const RIGHT: Keycode = Keycode::Right;
const UP: Keycode = Keycode::Up;
const DOWN: Keycode = Keycode::Down;
const LE: i8 = -1;
const RI: i8 = 1;
const ROT: i8 = 5;
const DO: i8 = 3;

const SCORE_LINE: i64 = 100;
const FREE_SPACE: &str = "                    ";

static GLOBAL_FLAG: Mutex<bool> = Mutex::new(false);
static LOOP_FLAG: Mutex<bool> = Mutex::new(false);
static NEW_TOY_FLAG: Mutex<bool> = Mutex::new(true);
static GLOBAL_TIME_UPD: Mutex<u64> = Mutex::new(TIME_UPD); //time screen update and level of hard game
static SCORE_STAT: Mutex<i64> = Mutex::new(0);

//pole[HEIGHT-1][0] - have info about last toy height position
//pole[HEIGHT-1][1] - have info about last toy width position
//pole[HEIGHT-1][2] - have info about count true, left or rigth click
//pole[HEIGHT-1][3] - have info about speed
//pole[HEIGHT-1][4] - have info about game over 

fn main() {
    global_flag_change(true);
    loop_flag_change(true);
    toy_flag_change(true);

    let mut termios = Termios::from_fd(0).unwrap();
    termios.c_lflag &= (ECHO | ICANON);
    termios::tcsetattr(0, TCSANOW, &termios).unwrap(); //off symbol stdin
    
    let mutex_toy = Arc::new(Mutex::new(vec![vec![FILL;2];3]));
    let mutex_pole = Arc::new(Mutex::new(vec![vec![FILL;WIDTH];HEIGHT]));

    let mutex_toy_2 = Arc::clone(&mutex_toy);
    let mutex_arc_2 = Arc::clone(&mutex_pole);
    let handle_two = thread::spawn(move || {
        let mut count = 0;
        let mut toy_now: Vec<Vec<u8>> = vec![vec![FILL;2];3]; 

        'l: loop {
            thread::sleep(Duration::from_millis(global_time_status()));

            let mut toy_now = mutex_toy_2.lock().unwrap(); 
            let mut pole_p = mutex_arc_2.lock().unwrap();
            let mut received_key: i8 = 0;
            
            if pole_p[HEIGHT-1][4] == 1 {
                clearscreen::clear().expect("Tetris failed");
                println!("{}     GAME OVER!", FREE_SPACE);
                println!();
                println!("{}  Your score: {}", FREE_SPACE, score_change_status());
                println!();
                break 'l;
            }

            if  toy_flag_status() {
                change_toy_to_static(&mut pole_p);
                count = 0;
                pole_p[HEIGHT-1][0] = 2;
                pole_p[HEIGHT-1][1] = START_PST; 
                rand_toy(&mut toy_now);
                toy_flag_change(false);
            }
            
            if loop_flag_status() { 
                clearscreen::clear().expect("Tetris failed");
                pole_print(update_screen(&mut pole_p, &mut received_key, &mut toy_now));
                
            }
            if count != HEIGHT-1 {
                count += 1;
            } else {count = 0}
            //println!("{}", &count);
        }
    });

    let mutex_toy = Arc::clone(&mutex_toy);
    let mutex_arc = Arc::clone(&mutex_pole);
    let handle_one = thread::spawn(move || {
    let mut toy_k: Vec<Vec<u8>> = vec![vec![FILL;2];3];

        loop {
            thread::sleep(Duration::from_millis(TIME_KEY));
            let mut toy_k = mutex_toy.lock().unwrap();
            let mut pole_k = mutex_arc.lock().unwrap();
            let mut num_key: i8 = key_status();   //читаем состояние клавиши
            if toy_flag_status() != true {
                match num_key {
                    LE | RI | ROT  => {
                        clearscreen::clear().expect("Tetris failed");
                        if num_key == ROT {
                            thread::sleep(Duration::from_millis(TIME_KEY));
                        }
                        pole_print(update_screen(&mut pole_k, &mut num_key, &mut toy_k)); //меняем поле после нажатия клавиши сразу      
                    },
                    DO => {
                        clearscreen::clear().expect("Tetris failed");
                        pole_print(update_screen(&mut pole_k, &mut num_key, &mut toy_k)); 
                    }
                    _=> {
                        loop_flag_change(true); 
                    },
                }                                    
            }
        }
    });

     handle_two.join().unwrap(); 
}

fn pole_print(pole_step:  Vec<Vec<u8>> ) {
    let space: String = String::from(FREE_SPACE);;

    let score_print: String = String::from("SCORE: ");
    let line_print: String = String::from("LINE: ");
    let speed_print: String = String::from("SPEED: ");

    let score: String = score_change_status();
    let line: String = String::from("?");
    let speed: String =  pole_step[HEIGHT-1][3].to_string();

    fn space_math(info_print: &String, space: &String, value: &String) -> String {
        let math = space.len() - info_print.len() - value.len();
        let mut out_string: String = String::new();

        for c in 0..math {
            out_string.push(' ');            
        }
        out_string
    }

    for string_cnt in 4..HEIGHT-1 {
        
        match string_cnt {
            4 => print!("{}{}{}",&score_print, &score, space_math(&score_print, &space, &score)),
            //5 => print!("{}{}{}", &line_print, &line, space_math(&line_print, &space, &line)),
            5 => print!("{}{}{}", &speed_print, &speed, space_math(&speed_print, &space, &speed)),
            _=> print!("{}", &space),
        }

        let end_arr_cnt = WIDTH - 1;

        for y in 0..WIDTH {
            let fill: String = String::from("' ");
            let toy: String = String::from("[]");
            match pole_step[string_cnt][y] {
                FILL => {
                    if y == 0 || y == WIDTH-1 {
                        match y {
                            0 => print!("<!"),
                            end_arr_cnt => print!("!>"),
                        }
                    } else {
                    print!("{}", fill)
                    }
                },
                STATIC_TOY => print!("{}", toy),
                TOY => print!("{}", toy),
                _=> {},
            }
        }
    println!();
    }
    print!("{}", &space);
    for n in 0..WIDTH {
        print!("==");
    }
    println!();
    print!("{}", &space);
    for n in 0..WIDTH {
        print!(r"\/");
    }
    println!();
    for _n in 1..5  {
         println!();
    }
    global_flag_change(true);
    
}

fn update_screen(vc: &mut Vec<Vec<u8>>,key: &mut i8, toy: &mut Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    global_flag_change(false);

    let mut key_stat: i8 = *key;
    let mut last_toy_height = vc[HEIGHT-1][0] as usize;
    let last_toy_width = vc[HEIGHT-1][1] as usize;
    let mut last_true_change = vc[HEIGHT-1][2] as usize; 

    let mut flag_undo_add_toy_width: bool = false;
    let mut flag_undo_add_toy_height: bool = false;
   
    fn clear_toy(vc: &mut  Vec<Vec<u8>>) {
        for h in (0..(HEIGHT-1)).rev() {
            for w in 1..WIDTH-1 {
                if vc[h][w] == TOY {
                    vc[h][w] = FILL 
                }
            }
        }
    }

    'c: for hg in (0..(HEIGHT-1)).rev() {
        for wd in 1..WIDTH-1 {
            if vc[hg][wd] == TOY && hg == (HEIGHT-2) {
                flag_undo_add_toy_height = true;
            } 
            if vc[hg][wd] == STATIC_TOY && vc[hg-1][wd] == TOY {
                flag_undo_add_toy_height = true;
            } 
            if vc[hg][wd] == TOY && wd - 1 == 0 && key_stat == LE {
                vc[HEIGHT-1][2] = TRUE_CHANGE as u8;
                last_true_change = TRUE_CHANGE;
            }
            if  vc[hg][wd] == TOY  && wd + 1 == (WIDTH-1) && key_stat == RI   {
                vc[HEIGHT-1][2] = TRUE_CHANGE as u8;
                last_true_change = TRUE_CHANGE;
            }
            if  vc[hg][wd] == TOY && vc[hg][wd+1] == STATIC_TOY && key_stat == RI {
                vc[HEIGHT-1][2] = TRUE_CHANGE as u8;
                last_true_change = TRUE_CHANGE;
            }
            if  vc[hg][wd] == TOY && vc[hg][wd-1] == STATIC_TOY && key_stat == LE {
                vc[HEIGHT-1][2] = TRUE_CHANGE as u8;
                last_true_change = TRUE_CHANGE;
            }
        }
    }
    match key_stat {
        LE | RI => {
            if last_true_change != TRUE_CHANGE {
                clear_toy(vc);
                let result = ((last_toy_width as i8) + key_stat) as usize;
                add_toy_in_pole(vc, toy, last_toy_height-1, result);
                vc[HEIGHT-1][1] = result as u8;
                vc[HEIGHT-1][2] += 1; 
            } 
        },
        ROT => {
            clear_toy(vc);
            if key_stat == ROT {
                rotate_toy(toy);
            }
            let result: bool = add_toy_in_pole(vc, toy, last_toy_height-1, last_toy_width);
            if result {
                for n in 0..3 {
                    rotate_toy(toy);
                }
            }
        },
        0 | DO => {
            if flag_undo_add_toy_height != true {
                vc[HEIGHT-1][0] = (last_toy_height as u8) + 1;
            } else {
                toy_flag_change(true);
            }

            'h: for h in (0..(HEIGHT-1)).rev() {
                for w in 1..WIDTH-1 {
                    if vc[HEIGHT-2][w] == TOY {
                        last_toy_height -= 1;
                        break 'h;
                    }
                    if vc[h][w] == STATIC_TOY && vc[h-1][w] == TOY {
                        last_toy_height -= 1;
                        break 'h;
                    }
                    if vc[h][w] == STATIC_TOY && h == 4 {
                        vc[HEIGHT-1][4] = 1;
                    } 
                }
            }

            vc[HEIGHT-1][2] = 0;
            clear_toy(vc);
            add_toy_in_pole(vc, toy, last_toy_height, last_toy_width);
        }, 
        _=> {},
    }

    ////////////////////////////////////SCORE AND SPEED CHANGE////////////////////////////////////

    let score_stat = SCORE_STAT.lock().unwrap();
    let score_stat_clone = score_stat.clone();
    drop(score_stat);
    let score_count = clear_string(vc);
    if score_count != 0 {
        for c in 0..score_count {
            score_change(SCORE_LINE);
        }
    }
    let score_stat = SCORE_STAT.lock().unwrap();
    
    if score_count > 0 {
        if *score_stat >= 1000 {
            let score_digit: Vec<char> = score_stat.to_string().chars().collect();
            let mut score_digit_clone: Vec<char> = score_stat_clone.to_string().chars().collect();

            let score_char: u8 = score_digit[0] as u8;
            if score_digit_clone.len() == 3 {
                score_digit_clone.insert(0, 0 as char);  // insert 0 to 0 element for good work 
            }
            let score_char_clone: u8 = score_digit_clone[0] as u8;

            if score_char > score_char_clone {
                global_time_change(20);
                vc[HEIGHT-1][3] += 1; 
            }
        }
    } 
  
    vc.to_vec()
}

fn change_toy_to_static(vc: &mut Vec<Vec<u8>>) {
    for h in (0..(HEIGHT-1)).rev() {
        for w in 0..WIDTH {
            if vc[h][w] == TOY {
                vc[h][w] = STATIC_TOY;
            }
        }
    }
    toy_flag_change(true);
}

fn add_toy_in_pole(vc_upd: &mut Vec<Vec<u8>>, toy_to_add: &mut Vec<Vec<u8>>, count_iter: usize, start_width: usize) -> bool {
    
    let mut vc_height = count_iter.clone();
    let mut vc_width = start_width.clone();
    let vc_upd_reserved = vc_upd.clone();     // clone if pole have STATIC TOY under TOY
    let toy_height: usize = toy_to_add.len() as usize;
    let toy_width: usize = toy_to_add[0].len() as usize;

    let mut result: bool = false;

    's:for h in 0..toy_height {
        for w in 0..toy_width {
            if toy_to_add[h][w] != FILL {
                if vc_upd[vc_height][vc_width] == STATIC_TOY || vc_width == WIDTH - 1 {
                    *vc_upd = vc_upd_reserved;
                    result = true;
                    break 's;
                }
                vc_upd[vc_height][vc_width] = toy_to_add[h][w];
            }
            vc_width += 1;
        }
        vc_width = start_width;
        vc_height += 1;
    }
    result 
}

fn rotate_toy(rot_toy12: &mut Vec<Vec<u8>>) {

    let rot_toy = rot_toy12.clone(); 
    let height: usize = rot_toy.len() as usize;
    let width: usize = rot_toy[0].len() as usize;

    /////////////////////////////////////новій размер для фигурі/////////////////////////////////////
    if width < height {
        for h in (width..height).rev() {
            rot_toy12.remove(h);
        }
        for h in 0..width {
            for w in width..height {
                rot_toy12[h].push(FILL);
            }
        }
    } 
    if width > height {
        for h in height..width {
            rot_toy12.push(vec![0; width]);
        }
        for h in 0..width {
            for w in (height..width).rev() {
                rot_toy12[h].remove(w);
            }
        }
    }

    ///////////////////////////////////////////////////крутим фигуру///////////////////////////////////////

    for h in 0..height {
        for w in 0..width {
            rot_toy12[w][height - h - 1] = rot_toy[h][w];
        } 
    } 
}

fn clear_string(vc_upd: &mut Vec<Vec<u8>>) -> u8 {
    let mut result: u8 = 0;
    'c: loop {
        't: for mut hg in (0..(HEIGHT-1)).rev() { // проверить все поле
            's: for wd in 1..WIDTH-1 {
                if vc_upd[hg][wd] == FILL || vc_upd[hg][wd] ==  TOY {
                    break 's;  // завершить проверку если наткнулись на 0
                } else if vc_upd[hg][wd] == STATIC_TOY { 
                    if wd == WIDTH - 2 {   // если проверка дошла до конца строки и там Статик_той
                        result += 1;
                        for w in 1..WIDTH - 1 {
                            vc_upd[hg][w] = FILL;
                        }
                        for h in (0..hg).rev() { //от текущей строки и до начала массива сместить все статик той на одну позицию вниз
                            for w in 1..WIDTH-1 {
                                if vc_upd[h][w] == STATIC_TOY {
                                    vc_upd[h+1][w] = STATIC_TOY;
                                    vc_upd[h][w] = FILL;
                                }
                            }
                        }
                        break 't;
                    }
                }
            } 
            if hg == 4 {
                break 'c;
            }
        }
    }
    result
}

fn rand_toy(toy: &mut Vec<Vec<u8>>) {
    let mut rng = rand::thread_rng();                                  //запускаем поток перебора

    let mut toy11: Vec<Vec<u8>> = vec![vec![FILL;2];3];                          // 222
    toy11[0][1] = TOY; toy11[1][1] = TOY; toy11[2][1] = TOY; toy11[1][0] = TOY;  //  2

    let mut toy12: Vec<Vec<u8>> = vec![vec![FILL;2];3];                          //  22
    toy12[0][0] = TOY; toy12[1][0] = TOY; toy12[1][1] = TOY; toy12[2][1] = TOY;  //   22

    let mut toy13: Vec<Vec<u8>> = vec![vec![FILL;2];3];                          //  2
    toy13[0][0] = TOY; toy13[1][0] = TOY; toy13[2][0] = TOY; toy13[2][1] = TOY;  //  222

    let mut toy14: Vec<Vec<u8>> = vec![vec![FILL;1];4];                           
    toy14[0][0] = TOY; toy14[1][0] = TOY; toy14[2][0] = TOY; toy14[3][0] = TOY;  //  2222

    let mut toy15: Vec<Vec<u8>> = vec![vec![FILL;2];3];                          //    2
    toy15[0][0] = TOY; toy15[1][0] = TOY; toy15[2][0] = TOY; toy15[0][1] = TOY;  //  222

    let mut toy16: Vec<Vec<u8>> = vec![vec![FILL;2];3];                          //   22
    toy16[0][1] = TOY; toy16[1][1] = TOY; toy16[1][0] = TOY; toy16[2][0] = TOY;  //  22

    let mut toy17: Vec<Vec<u8>> = vec![vec![FILL;2];2];                          //  22
    toy17[0][0] = TOY; toy17[0][1] = TOY; toy17[1][0] = TOY; toy17[1][1] = TOY;  //  22

    let toy_arr = [toy13, toy14, toy12, toy11, toy15, toy16, toy17];
    let random_index = rng.gen_range(0..toy_arr.len());
    let toy_value: &Vec<Vec<u8>> = &toy_arr[random_index];

    let height_toy: usize = toy.len() as usize;
    let width_toy: usize = toy[0].len() as usize;
    let height: usize = toy_value.len() as usize;
    let width: usize = toy_value[0].len() as usize;

    if height_toy < height {
        for h in height_toy..height {
            toy.push(vec![FILL; width_toy]);
        }
    }
    if height_toy > height {
        for h in (height..height_toy).rev() {
            toy.remove(h);
        }
    }
    if width_toy < width {
        for h in 0..height {
            for w in width_toy..width {
                toy[h].push(FILL);
            }
        }
    }
    if width_toy > width  {
        for h in 0..height {
            for w in (width..width_toy).rev() {
                toy[h].remove(w);
            }
        }
    }

    for h in 0..height {
        for w in 0..width {
            toy[h][w] = toy_value[h][w];
        }
    }
}

fn key_status() -> i8 {

    let device_state = DeviceState::new(); //read key status

    let keys: Vec<Keycode> = device_state.get_keys(); //get keys
    let mut position: i8 = 0;
    
    if keys.len() > 0 { // get position toy
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
    let mut time_flag_clone = time_flag.clone();
    if time_flag_clone >= 50 {
        *time_flag = time_flag_clone - ((time_flag_clone/100) * value as u64);
    }
}

fn global_time_status() -> u64 {
    let time_flag = {
        let time_flag_status = GLOBAL_TIME_UPD.lock().unwrap();
        *time_flag_status
    };
    time_flag
}

fn score_change_status() -> String {
    let score: String = {
        let score_status = SCORE_STAT.lock().unwrap();
        format!("{}", *score_status)
    };
    score
}

fn score_change(value: i64) {
    let mut score = SCORE_STAT.lock().unwrap();
    let score_clone: i64 = score.clone() as i64;
    *score = score_clone + value;
}

