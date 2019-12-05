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
use std::fs;
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
type Nstep = u32;

#[derive(Eq, PartialEq)]
enum Participant {
    A,
    B,
    C,
    Ghost,
}

fn gamble() -> (Result, Nstep) {
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
                return (Some(state), trans_counter);
            }
        }

        trans_counter += 1;
    }

    info!("Reach the end of gamble");
    return (None, 0);
}

fn end(state: &State) -> bool {
    for i in state.iter() {
        if *i == 0 {
            return true;
        }
    }
    return false;
}

fn start_gamble() -> (Result, Nstep) {
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

    /* calculate consumed steps */
    let mut nsteps: HashMap<u32, u32> = HashMap::new();

    /* dump config */
    println!("==============================");
    println!("Sample Point Number: {}", NTHREAD);
    println!("Starting point: {}, {}, {}", PoA, PoB, PoC);
    println!("==============================");

    for _ in 0..NTHREAD {
        let (res_op, nstep) = start_gamble();

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

        /* update  hashmap shows how many step it took */
        let nc_step = nsteps.entry(nstep).or_insert(0);
        *nc_step += 1;
        

        /* determine how end the game */
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
    log_pfile(Path::new("out/nsteps.txt"), nsteps);

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

    /* try to create the directory */
    fs::create_dir_all("/out");

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

/* output test result to txt file, but diffent map format */
fn log_pfile(file_name: &Path, map: HashMap<u32, u32>) {
    let display = file_name.display();

    /* try to create the directory */
    fs::create_dir_all("/out");

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
            if result[0] == 0 {
                return format!("{}", result[1])
            } else if result[1] == 0 {
                return format!("{}", result[2])
            } else if result[2] == 0 {
                return format!("{}", result[0])
            } else {
                return "NULL".to_owned()
            }
        },
        None => {
            return "NULL".to_owned()
        },
    }
}

