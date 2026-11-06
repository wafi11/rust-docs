#[derive(Debug, Clone, Copy)]
pub enum Op {
    Plus,
    Minus,
    Mul,
    Div,
}

pub fn calc(op: Op, a: i32, b: i32) -> Result<i32, String> {
    match op {
        Op::Plus => Ok(a + b),
        Op::Minus => Ok(a - b),
        Op::Mul => Ok(a * b),
        Op::Div => a
            .checked_div(b)
            .ok_or_else(|| "division by zero".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds() {
        assert_eq!(calc(Op::Plus, 1, 2), Ok(3));
    }

    #[test]
    fn subtracts() {
        assert_eq!(calc(Op::Minus, 1, 2), Ok(-1));
    }

    #[test]
    fn multiplies() {
        assert_eq!(calc(Op::Mul, 3, 4), Ok(12));
    }

    #[test]
    fn divides() {
        assert_eq!(calc(Op::Div, 6, 2), Ok(3));
    }

    #[test]
    fn division_by_zero_is_err() {
        assert!(calc(Op::Div, 1, 0).is_err());
    }
}
