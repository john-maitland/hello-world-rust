fn add_two_number(first : i32, second : i32) -> i32 {
    first + second
}

fn main() {
    println!("Adding: {} + {} = {}", 1, 2, add_two_number(1, 2));
}


#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_add() {
        
        let first = 1;
        let second = 2;

        assert_eq!(3, add_two_number(first, second))
    }
}