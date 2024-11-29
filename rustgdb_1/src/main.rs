use std::cmp::max;

#[derive(Debug)]
struct Character {
    first_name: String,
    last_name: String,
    class: String,
    life: i32,
    armor: i32,
    attack: i32,
}

impl Character {
    fn get_name(&self) -> String {
        format!("[{}] {} {}", self.class, self.first_name, self.last_name)
    }

    // method that change Character internals (=> &mut self)

    // This method has a logical bug
    fn take_damages(&mut self, damage: i32) {
        self.life -= max(damage - self.armor, 0);
    }

    fn attacked(&mut self, other: &Character) {
        self.take_damages(other.attack);
    }

    // 'class' method to be called like: Character::create_wizard(...)
    fn create_wizard(first_name: String, last_name: String, _attack: i32) -> Character {
        Character {
            first_name,
            last_name,
            class: String::from("wizard"),
            life: 25,
            armor: 2,
            attack: 6,
        }
    }

    fn is_dead(&self) -> bool {
        self.life == 0
    }
}

fn create_wizard() -> Character {
    Character::create_wizard("Merlin".to_string(), "The Wizard".to_string(), 500)
}

fn main() {
    println!("Hello, world!");

    let mut hero1 = Character {
        first_name: String::from("Conan"),
        last_name: String::from("The Barbarian"),
        class: String::from("Barbarian"),
        life: 105,
        armor: 5,
        attack: 86,
    };

    let bad_guy_1 = Character {
        first_name: String::from("Dark Conan"),
        last_name: String::from("The Dark Barbarian"),
        class: String::from("Barbarian"),
        life: 165,
        armor: 4,
        attack: 96,
    };

    hero1.attacked(&bad_guy_1);
    println!("Is hero1 {} dead?: {}", hero1.get_name(), hero1.is_dead());
    hero1.attacked(&bad_guy_1);
    println!("Is hero1 {} dead?: {}", hero1.get_name(), hero1.is_dead());
    hero1.attacked(&bad_guy_1);
    println!("Is hero1 {} dead?: {}", hero1.get_name(), hero1.is_dead());

    let wz1 = create_wizard();
    println!("wz1: {:?}", wz1);
}

#[cfg(test)]
mod tests {

    // allow access to sub() in current file
    use super::*;

    #[test]
    fn attack_1() {
        let mut c1 = Character {
            first_name: String::from("f1"),
            last_name: String::from("l1"),
            class: String::from("c1"),
            life: 100,
            armor: 0,
            attack: 1,
        };

        let c2 = Character {
            first_name: String::from("f2"),
            last_name: String::from("l2"),
            class: String::from("c2"),
            life: 200,
            armor: 0,
            attack: 100,
        };

        c1.attacked(&c2);
        assert!(c1.is_dead());
    }
}
