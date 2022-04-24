use std::cmp::max;

fn main() {

    // println!("Hello, world!");

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
            format!("{} {}", self.first_name, self.last_name)
        }

        // method that change Character internals (=> &mut self)

        fn take_damages(&mut self, damage: i32) {
            self.life -= max(damage - self.armor, 0);
        }

        fn attacked(&mut self, other: &Character) {
            self.take_damages(other.attack);
        }

        // 'class' method to be called like: Character::create_wizard(...)
        fn create_wizard(first_name: String, last_name: String, attack: i32) -> Character {

            return Character {
                first_name,
                last_name,
                class: String::from("wizard"),
                life: 25,
                armor: 2,
                attack: 6,
            };
        }

    }

    let mut conan = Character {
        first_name: String::from("Conan"),
        last_name: String::from("The Barbarian"),
        class: String::from("Barbarian"),
        life: 65,
        armor: 5,
        attack: 86,
    };

    // requires, the: #[derive(Debug)]
    println!("Conan: {:?}", conan);
    conan.attack = 82;
    println!("Conan: {:?}", conan);

    println!("Conan says its name: {}!!", conan.get_name());
    println!("Conan is taking damages... :-/");
    conan.take_damages(15);
    println!("Conan: {:?}", conan);

    let evil_wizard = Character::create_wizard(String::from("wizzy"), String::from("bar"), 32);
    println!("And here comes the legendary evil: {:?}", evil_wizard);
    println!("Conan is being attacked by {}...", evil_wizard.get_name());
    conan.attacked(&evil_wizard);
    println!("Conan: {:?}", conan);
}
