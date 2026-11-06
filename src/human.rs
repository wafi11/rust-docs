#[derive(Debug)]
pub struct Human {
    pub name: String,
    pub age: i32,
}

impl Human {
    pub fn birthday(&mut self) {
        self.age += 1;
    }
}

pub fn print_user(u: &Human) {
    println!("{} {}", u.name, u.age);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn birthday_increments_age() {
        let mut h = Human {
            name: "wafiuddin".into(),
            age: 10,
        };
        h.birthday();
        assert_eq!(h.age, 11);
    }

    #[test]
    fn print_user_borrows() {
        let h = Human {
            name: "test".into(),
            age: 1,
        };
        print_user(&h);
        assert_eq!(h.age, 1);
    }
}
