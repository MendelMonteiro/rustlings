// test4.rs
// This test covers the sections:
// - Modules
// - Macros

// Write a macro that passes the test! No hints this time, you can do it!

macro_rules! my_macro { 
    ($val:expr) => {
        {
            let mut temp_str = String::from("Hello ");
            temp_str.push_str($val);
            temp_str 
        }
    };
    // or 
    // "Hello $val"
}

fn main() {
    if my_macro!("world!") != "Hello world!" {
        panic!("Oh no! Wrong output!");
    }
}
