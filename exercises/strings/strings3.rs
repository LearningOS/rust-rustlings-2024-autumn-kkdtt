// strings3.rs
//
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a
// hint.


use std::char;

fn trim_me(input: &str) -> String {
    // TODO: Remove whitespace from both ends of a string!
    
//     let binding = input.to_string();
//     let bytes=binding.as_bytes();
//    for i in (0..bytes.len()).rev(){


//     if bytes[i] != b' ' {
//         let a=input.to_string();
//         let b=&a[..i];
//         let c=b.to_string();
//         return c;
//     }
    
//    }
//    return input.to_string();

      let s=input.to_string();
      let str=s.trim();
      str.to_string()

}

fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There's multiple ways to do this!
    let mut s=input.to_string();
    s.push_str(" world!");
    s

}

fn replace_me(input: &str) -> String {
    // TODO: Replace "cars" in the string with "balloons"!
    let s=input.to_string();
    s.replace("cars", "balloons")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(replace_me("I think cars are cool"), "I think balloons are cool");
        assert_eq!(replace_me("I love to look at cars"), "I love to look at balloons");
    }
}
