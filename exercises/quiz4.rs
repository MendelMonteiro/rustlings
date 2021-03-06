// quiz4.rs
// This quiz covers the sections:
// - Modules
// - Macros

// Write a macro that passes the quiz! No hints this time, you can do it!

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_macro_world() {
        assert_eq!(my_macro!("world!"), "Hello world!");
    }

    #[test]
    fn test_my_macro_goodbye() {
        assert_eq!(my_macro!("goodbye!"), "Hello goodbye!");
    }
}
