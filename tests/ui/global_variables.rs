#![warn(clippy::global_variables)]

use std::sync::Mutex;
use std::sync::atomic::AtomicU32;

macro_rules! define_global_variable_with_macro {
    () => {
        static GLOBAL_VARIABLE_0: AtomicU32 = AtomicU32::new(2);
        //~^ global_variables

        static GLOBAL_VARIABLE_1: Mutex<u32> = Mutex::new(3);
        //~^ global_variables
    };
}

fn main() {
    define_global_variable_with_macro!();

    static GLOBAL_VARIABLE_2: AtomicU32 = AtomicU32::new(2);
    //~^ global_variables

    static GLOBAL_VARIABLE_3: Mutex<u32> = Mutex::new(3);
    //~^ global_variables

    static NOT_GLOBAL_VARIABLE_0: u32 = 1;
    static NOT_GLOBAL_VARIABLE_1: Vec<AtomicU32> = Vec::new();
    static NOT_GLOBAL_VARIABLE_2: Vec<Mutex<u32>> = Vec::new();
}
