extern crate rand;
extern crate log;
extern crate env_logger;
extern crate serde;
extern crate serde_json;

use rand::Rng;
use std::thread;
use std::collections::HashMap;
use log::{debug, info, error};
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use serde::Serialize;


const TRANS_THRES: u32 = 10000;
const NTHREAD: u32 = 10000;
const PoA:i32 = 30;
const PoB:i32 = 30;
const PoC:i32 = 30;

type State = [i32; 3];
type Result = Option<State>;

#[derive(Eq, PartialEq)]
enum Participant {
    A,
    B,
    C,
    Ghost,
}

fn gamble() -> Result {
    /* initial state of a, b, c */
    let mut state = [PoA, PoB, PoC];
    /* transition matrix */
    let a_give_b = [-1, 1, 0];
    let a_give_c = [-1, 0, 1];
    let b_give_a = [1, -1, 0];
    let b_give_c = [0, -1, 1];
    let c_give_a = [1, 0, -1];
    let c_give_b = [0, 1, -1];

    let mut trans_counter = 0;

    let mut rng = rand::thread_rng();

    while trans_counter <= TRANS_THRES {
        /* state transition with possibility */
        let possiblity:u8 = rng.gen();

        match possiblity % 6 {
            0 => {
                state = add(state, a_give_b.clone());
            },
            1 => {
                state = add(state, a_give_c.clone());
            },
            2 => {
                state = add(state, b_give_a.clone());
            },
            3 => {
                state = add(state, b_give_c.clone());
            },
            4 => {
                state = add(state, c_give_a.clone());
            },
            5 => {
                state = add(state, c_give_b.clone());
            },
            _ => {
                error!("Module operation error");
            },
        }
        /* see if it reaches the terminating state */
        {
            if end(&state) {
                info!("Result: {}, {}, {}", state[0], state[1], state[2]);
                /* Game Finished */
                return Some(state);
            }
        }

        trans_counter += 1;
    }

    info!("Reach the end of gamble");
    return None;
}

fn end(state: &State) -> bool {
    for i in state.iter() {
        if *i == 0 {
            return true;
        }
    }
    return false;
}

fn start_gamble() -> Result {
    let handle = thread::spawn(move || {
        gamble()
    });
    handle.join().unwrap()
}

/* do it with a multi-thread manner */
fn main() {

    env_logger::init();

    let mut results_with_a_out: HashMap<String, u32> = HashMap::new();
    let mut results_with_b_out: HashMap<String, u32> = HashMap::new();
    let mut results_with_c_out: HashMap<String, u32> = HashMap::new();


    /* dump config */
    println!("==============================");
    println!("Sample Point Number: {}", NTHREAD);
    println!("Starting point: {}, {}, {}", PoA, PoB, PoC);
    println!("==============================");

    for _ in 0..NTHREAD {
        let mut res_op = start_gamble();

        /* check if the result is recorded */
        let mut append = Participant::Ghost;

        let mut bankbroke_man = Participant::Ghost;
        {
            if let res = res_op.clone().unwrap() {
                for (i, item) in res.iter().enumerate() {
                    if *item == 0 {
                        match i {
                            0 => {
                                bankbroke_man = Participant::A;
                                break;
                            },
                            1 => {
                                bankbroke_man = Participant::B;
                                break;
                            },
                            2 => {
                                bankbroke_man = Participant::C;
                                break;
                            },
                            _ => {},
                        }
                    }
                }
            }            
        }

        match bankbroke_man {
            Participant::A => {
                let count = results_with_a_out.entry(to_string(res_op)).or_insert(0);
                *count += 1;
            },
            Participant::B => {
                let count = results_with_b_out.entry(to_string(res_op)).or_insert(0);
                *count += 1;
            },
            Participant::C => {
                let count = results_with_c_out.entry(to_string(res_op)).or_insert(0);
                *count += 1;
            },
            Participant::Ghost => {
                debug!("Reach Maximum Gamble Times");
            },
        }
    }

    /* output hashmap when one of three is bankrupt */ 
    log_file(Path::new("out/a_out.txt"), results_with_a_out);
    log_file(Path::new("out/b_out.txt"), results_with_b_out);
    log_file(Path::new("out/c_out.txt"), results_with_c_out);

}

/* array addition */
fn add(a: State, b: State) -> State {
    let mut z: State = [0, 0, 0];
    for (i, (aval, bval)) in a.iter().zip(&b).enumerate() {
        z[i] = aval + bval;
    }
    z
}

/* output test result to txt file */
fn log_file(file_name: &Path, map: HashMap<String, u32>) {
    let display = file_name.display();

    let mut file = match File::create(&file_name) {
        Err(why) => panic!("couldn't create A's {}: {}", display, why.description()),
        Ok(file) => file,
    };

    let serialized_res = serde_json::to_string(&map);
    match serialized_res {
        Err(why) => panic!("couldn't serialize A's hashmap since {}", why),
        Ok(serialized) => {
            match file.write_all(&mut serialized.as_bytes()) {
                Err(why) => panic!("couldn't print out A's hashmap since {}", why),
                Ok(_) => { },
            }
        },
    }
}

fn to_string(result_op: Result) -> String {
    match result_op {
        Some(result) => {
            return format!("{}, {}, {}", result[0], result[1], result[2])
        },
        None => {
            return "NULL".to_owned()
        }
    }
}

