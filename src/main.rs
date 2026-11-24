fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    println!("Hello from Jenkins CI v2! 2 + 3 = {}", add(2, 3));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_numbers() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn adds_negative_numbers() {
        assert_eq!(add(-2, -3), -5);
    }
}
