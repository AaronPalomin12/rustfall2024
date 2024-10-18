fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn multiply(x: i32, y: i32) -> i32 {
    let mut total: i32 = 0;

    for _ in 0..y.abs() {
        total = add(total, x);
    }

    if y < 0 {
        total = -total;
    }
    total
}

fn exponentiation(base: i32, power: i32) -> f32 {
    if power < 0 {
        return 1.0 / exponentiation(base, -power) as f32;
    }

    let mut total: i32 = 1;
    for _ in 0..power {
        total = multiply(total, base);
    }
    total as f32
}

fn main() {
    println!("2 + 2 = {}", add(2, 2));
    println!("5 * 2 = {}", multiply(5, 2));
    println!("2^2 = {}", exponentiation(2, 2)); 
    println!("2^-2 = {}", exponentiation(2, -2)); 
    let mut x: i32 = 5.0;
    let dangerous_value:f32 = 1.0/3.0;
    x = x + dangerous_value;
    println!("5 + 1/3 = {}", x);
    x = x - dangerous_value;
    println!("x = 1/3 = 5 {}", x);



    println!("5 + 1/3 = {}", add(2, exponentiation(3, -1));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 2), 4);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(multiply(5, 2), 10);
    }

    #[test]
    fn test_exponentiation() {
        assert_eq!(exponentiation(5, 2), 25.0);
        assert_eq!(exponentiation(5, -2), 0.04);
        assert!((0.3333 - exponentiation(3, -1)).abs() < 0.005);
    }

    #[test]
    fn test_add_multiple() {
        let test_cases = vec![
            (1, 1, 2),
            (0, 0, 0),
            (-1, 1, 0),
            (100, -50, 50)
        ];

        for (a, b, expected) in test_cases {
            assert_eq!(add(a, b), expected, "Failed on input ({}, {})", a, b);
        }
    }
}
