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

#[derive(Eq, PartialEq)]
enum Participant {
    A,
    B,
    C,
    Ghost,
}

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

    let mut results_with_a_out: HashMap<Result, u32> = HashMap::new();
    let mut results_with_b_out: HashMap<Result, u32> = HashMap::new();
    let mut results_with_c_out: HashMap<Result, u32> = HashMap::new();


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
                results_with_a_out.insert(res_op, 1);
            },
            Participant::B => {
                results_with_b_out.insert(res_op, 1);
            },
            Participant::C => {
                results_with_c_out.insert(res_op, 1);
            },
            Participant::Ghost => {
                debug!("Reach Maximum Gamble Times");
            },
        }
    }
}


