extern crate ndarray;
extern crate rand;
extern crate log;
extern crate env_logger;

use ndarray::{arr1, Array1};
use rand::Rng;
use std::thread;
use std::collections::HashMap;
use log::{debug, info, error};

const TRANS_THRES: u32 = 10000;
const NTHREAD: u32 = 10000;
const PoA:i32 = 30;
const PoB:i32 = 30;
const PoC:i32 = 30;

type Result = Option<Array1<i32>>;

fn gamble() -> Result {
    /* initial state of a, b, c */
    let mut state = arr1(&[PoA, PoB, PoC]);
    /* transition matrix */
    let a_give_b = arr1(&[-1, 1, 0]);
    let a_give_c = arr1(&[-1, 0, 1]);
    let b_give_a = arr1(&[1, -1, 0]);
    let b_give_c = arr1(&[0, -1, 1]);
    let c_give_a = arr1(&[1, 0, -1]);
    let c_give_b = arr1(&[0, 1, -1]);

    let mut trans_counter = 0;

    let mut rng = rand::thread_rng();

    while trans_counter <= TRANS_THRES {
        /* state transition with possibility */
        let possiblity:u8 = rng.gen();

        match possiblity % 6 {
            0 => {
                state = state + &a_give_b;
            },
            1 => {
                state = state + &a_give_c;
            },
            2 => {
                state = state + &b_give_a;
            },
            3 => {
                state = state + &b_give_c;
            },
            4 => {
                state = state + &c_give_a;
            },
            5 => {
                state = state + &c_give_b;
            },
            _ => {
                error!("Module operation error");
            },
        }
        /* see if it reaches the terminating state */
        {
            if end(&state) {
                info!("Result: {}", state);
                /* Game Finished */
                return Some(state);
            }
        }

        trans_counter += 1;
    }

    info!("Reach the end of gamble");
    return None;
}

fn end(state: &Array1<i32>) -> bool {
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

    let mut results: HashMap<Result, u32> = HashMap::new();

    /* dump config */
    println!("==============================");
    println!("Sample Point Number: {}", NTHREAD);
    println!("Starting point: {}, {}, {}", PoA, PoB, PoC);
    println!("==============================");

    for _ in 0..NTHREAD {
        let mut res = start_gamble();

        /* check if the result is recorded */
        let mut append = false;
        {
            if results.contains_key(&res) {
                append = true;
            }
        }
        if append {
            let val_op = results.get_mut(&mut res);
            match val_op {
                Some(val) => {
                    *val += 1;
                },
                None => {
                    error!("Modifing not-existed value");
                },
            }
        } else {
            results.insert(res, 0);
        }
    }
}
