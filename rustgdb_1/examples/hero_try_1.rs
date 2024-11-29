fn main() {
    let _hero_1 = "Conan, the Barbarian";
    let _attack_1 = 70;
    let mut life_1: i32 = 100;

    let _bad_guy_1 = "Dark Conan, the Dark Barbarian";
    let attack_2 = 100;
    let _life_2: i32 = 70;

    // attack on hero 1
    life_1 -= attack_2;
    life_1 -= attack_2;
    life_1 -= attack_2;

    dbg!(life_1);
}
