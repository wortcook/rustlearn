use lazy_static::lazy_static;

use num_bigint::BigUint;
use std::sync::{Mutex, RwLock};
use std::thread;

use std::io::{self, Write};


const LOOKUP_SIZE : usize = 1_000_000;
const STARTING_NUMBER : u64 = 100_000_000_000_000_000u64;


lazy_static!(    
    static ref PRIME_LIST : RwLock<Vec<BigUint>> = RwLock::new(vec![]);
    static ref ZERO : BigUint = BigUint::from(0u32);
    static ref counter : Mutex<u32> = Mutex::new(0u32);
);


//check if a number is prime
fn check_prime(ref number: &BigUint) -> bool{
    let max_check = number.sqrt();
    let prime_list_l = PRIME_LIST.read().unwrap();
    for prime in prime_list_l.iter() {
        let remainder = *number % prime;
        if remainder == *ZERO {
            return false;
        }

        if max_check < *prime {
            break;
        }
    }
    // *counter.lock().unwrap() += 1;
    // println!("{} : {} ", counter.lock().unwrap(), number);
    return true;
}

fn check_prime_large(ref number: &BigUint) -> bool{
    let max_check = number.sqrt();
    let prime_list_l = PRIME_LIST.read().unwrap();
    let mut broke_on_max = false;
    for prime in prime_list_l.iter() {
        let remainder = *number % prime;
        if remainder == *ZERO {
            return false;
        }
    }

    let ref val_next_check = prime_list_l.last().unwrap();
    let mut next_check = 10u32*(*val_next_check/10u32) + 1u32;

    let mut check_counter : u64 = 0u64;

    println!("Deep checking {}", number);
    while next_check < max_check {
        if check_counter % 10_000_000 == 0 {
            print!("#");
            io::stdout().flush().unwrap();
        }

        check_counter += 1;

        //check numbers ending in 1
        {
            let ref mod_next_check = next_check;
            let remainder = *number % mod_next_check;
            if remainder == *ZERO {
                println!();
                return false;
            }
        }
        next_check += 2u32;

        //check numbers ending in 3
        {
            let ref mod_next_check = next_check;
            let remainder = *number % mod_next_check;
            if remainder == *ZERO {
                println!();
                return false;
            }
        }
        next_check += 4u32;


        //check numbers ending in 7
        {
            let ref mod_next_check = next_check;
            let remainder = *number % mod_next_check;
            if remainder == *ZERO {
                println!();
                return false;
            }
        }
        next_check += 2u32;

        //check numbers ending in 9
        {
            let ref mod_next_check = next_check;
            let remainder = *number % mod_next_check;
            if remainder == *ZERO {
                println!();
                return false;
            }
        }
        next_check += 2u32;  //and back to 1
    }
    *counter.lock().unwrap() += 1;
    println!();
    println!("{} : {} ", counter.lock().unwrap(), number);
    io::stdout().flush().unwrap();

    return true;
}

//write a main method that finds and prints prime numbers
fn main() {
    // let mut PRIME_LIST : Vec<&BigUint> = Vec::new();

    let THREE : BigUint = BigUint::from(3u32);
    let FIVE  : BigUint = BigUint::from(5u32);
    let SEVEN : BigUint = BigUint::from(7u32);

    PRIME_LIST.write().unwrap().push(THREE);
    PRIME_LIST.write().unwrap().push(FIVE);
    PRIME_LIST.write().unwrap().push(SEVEN);

    let mut number = BigUint::from(10u32);
    
    for _ in 0..LOOKUP_SIZE {
    // loop{
        let candidate_one   : BigUint = &number + 1u32;
        let candidate_three : BigUint = &number + 3u32;
        let candidate_seven : BigUint = &number + 7u32;
        let candidate_nine  : BigUint = &number + 9u32;


        // let handle_one = thread::spawn(move || {
            if(check_prime(&candidate_one) ){
                PRIME_LIST.write().unwrap().push(candidate_one);
            }
        // });

        // let handle_three = thread::spawn(move || {
            if(check_prime(&candidate_three) ){
                PRIME_LIST.write().unwrap().push(candidate_three);
            }
        // });

        // let handle_seven = thread::spawn(move || {
            if(check_prime(&candidate_seven) ){
                PRIME_LIST.write().unwrap().push(candidate_seven);
            }
        // });

        // let handle_nine = thread::spawn(move || {
            if(check_prime(&candidate_nine) ){
                PRIME_LIST.write().unwrap().push(candidate_nine);
            }
        // });

        // handle_one.join().unwrap();
        // handle_three.join().unwrap();
        // handle_seven.join().unwrap();
        // handle_nine.join().unwrap();

        number += 10u32;
    }

    number = BigUint::from(STARTING_NUMBER);
    number *= 1_000_000u32;


    // for _ in 0..10000000 {
    loop{
        let candidate_one   : BigUint = &number + 1u32;
        let candidate_three : BigUint = &number + 3u32;
        let candidate_seven : BigUint = &number + 7u32;
        let candidate_nine  : BigUint = &number + 9u32;

        check_prime_large(&candidate_one);
        check_prime_large(&candidate_three);
        check_prime_large(&candidate_seven);
        check_prime_large(&candidate_nine);

        number += 10u32;
    }
    
}