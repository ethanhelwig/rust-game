mod character;
mod class;
use cli_tables::Table;

use crate::character::*;
use crate::class::*;
fn main() {
    // make some characters and change their stats
    let mut a: Character = Character::new("Ethan", Class::Rogue);
    a.set_level(100);
    let b: Character = Character::new("Eric", Class::Warrior);
    let mut c: Character = Character::new("Ezak", Class::Archer);
    c.is_damaged(40);
    let mut d: Character = Character::new("Liz", Class::Mage);
    d.is_damaged(20);
    d.is_damaged(90);
    d.is_healed(1);
    let e: Character = Character::new("Winston", Class::Healer);

    // define the table
    let data: Vec<Vec<String>> = vec![
        Character::headers(),
        a.values(),
        b.values(),
        c.values(),
        d.values(),
        e.values()
    ];
    let mut test_table = Table::new(&data);

    // print in column format using my new cli-table library
    println!("{}", test_table.to_string());
}