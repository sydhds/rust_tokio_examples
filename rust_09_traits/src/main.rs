use std::cmp::max;

// Define a trait, we will have to implement it for Character (see below)
trait Describe {
    fn describe(&self) -> String;

    fn describe_with_default(&self) -> String {
        String::from("Dunno!")
    }
}

#[derive(Debug)]
struct Character {
    first_name: String,
    last_name: String,
    class: String,
    life: i32,
    armor: i32,
    attack: i32,
}

#[allow(dead_code)]
impl Character {
    fn get_name(&self) -> String {
        format!("[{}] {} {}", self.class, self.first_name, self.last_name)
    }

    // method that change Character internals (=> &mut self)

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
}

impl Describe for Character {
    fn describe(&self) -> String {
        format!(
            "{}, class: {}, attack: {}, armor: {}, life: {}",
            self.get_name(),
            self.class,
            self.attack,
            self.armor,
            self.life
        )
    }
}

#[derive(Debug)]
struct Animal {
    name: String,
    class: String,
}

impl Describe for Animal {
    fn describe(&self) -> String {
        format!("Animal: {}, class: {}", self.name, self.class,)
    }
}

// function that require an 'object' that implement the trait: Describe
fn omni_describe(entity: &impl Describe) -> String {
    entity.describe()
}

// longer form of declaration
fn omni_describe_2<T: Describe>(entity: &T) -> String {
    entity.describe()
}

// longer form of declaration: multiple traits required
fn omni_describe_3<T: Describe + std::fmt::Debug>(entity: &T) -> String {
    entity.describe()
}

// longer form of declaration: multiple traits required + where clause
fn omni_describe_4<T, U>(entity: &T, entity2: &U) -> String
where
    T: Describe + std::fmt::Debug,
    U: Describe,
{
    format!(
        "entity: {}\nentity2: {}",
        entity.describe(),
        entity2.describe()
    )
}

//

// a function that return an obj with a Trait

fn make_lazy_cat() -> impl Describe {
    Animal {
        name: "garfield".to_string(),
        class: "cat".to_string(),
    }
}

// Note: this code will not compile
/*
fn make_animal(is_animal: bool) -> impl Describe {
    if is_animal {
        Animal {
            name: "meow meow".to_string(),
            class: "cat".to_string(),
        }
    } else {
        Character {
            first_name: "".to_string(),
            last_name: "".to_string(),
            class: "".to_string(),
            life: 0,
            armor: 0,
            attack: 0
        }
    }
}
*/

//

fn main() {
    // println!("Hello, world!");

    let conan = Character {
        first_name: "Conan".to_string(),
        last_name: "The Barbarian".to_string(),
        class: "Barbarian".to_string(),
        life: 50,
        armor: 5,
        attack: 25,
    };

    println!("Here comes our hero: {}!!", conan.get_name());
    println!("[Game Master] Hero, describe yourself!");
    println!("{}", conan.describe());
    println!("{}", conan.describe_with_default());

    let dog1 = Animal {
        name: "Woofy".to_string(),
        class: "dog".to_string(),
    };

    println!("Here comes an animal: {}!!", dog1.name);
    println!("[Game Master] Animal, describe yourself!");
    println!("...");
    println!("[Game Master] Ok, let me find out your description...");
    omni_describe(&dog1);
    println!("{}", omni_describe(&dog1));
    println!("[Game Master] and now use it for our pseudo hero too...");
    println!("{}", omni_describe_2(&conan));
    println!("{}", omni_describe_3(&conan));
    println!("[Game Master] and now I want to describe both of you!!");
    println!("{}", omni_describe_4(&conan, &dog1));

    let cat1 = make_lazy_cat();
    println!("[cat] meow! (from: {})", cat1.describe());
}
