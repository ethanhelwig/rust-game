mod character;
mod class;
use std::cmp::max;

use crate::character::*;
use crate::class::*;
fn main() {
    let mut a: Character = Character::new("Ethan", Class::Rogue);
    a.set_level(100);
    let b: Character = Character::new("Eric", Class::Warrior);
    let mut c: Character = Character::new("Ezak", Class::Archer);
    c.is_damaged(40);
    let mut d: Character = Character::new("Liz", Class::Mage);
    d.is_damaged(20);
    d.is_damaged(90);
    let e: Character = Character::new("Winston", Class::Healer);

    let mut characters: Vec<Character> = Vec::new();
    characters.push(a);
    characters.push(b);
    characters.push(c);
    characters.push(d);
    characters.push(e);

    let mut formatted_values: String = "".to_string();
    for character in &characters {
        formatted_values += &character.values();
    }

    println!("{}", make_table(&formatted_values, characters.len()));
}



fn make_table (formatted_values: &String, num_items: usize) -> String {
    let mut table: String = "".to_string();
    let mut length: usize;
    let mut curr: usize = 0;
    let left_border: &str = "| ";
    let separator: &str = " | ";
    let right_border: &str = " |";
    let edge = "+";
    let mut total_values:usize = 0;
    for _ in formatted_values.split(";") {
        total_values += 1;
    }
    let num_columns = (total_values / 2) / num_items;
    let mut max_length: Vec<usize> = vec![0; num_columns];

    // parse values using delimeters
    let mut headers: Vec<&str> = Vec::new();
    let mut values: Vec<Vec<&str>> = vec!(Vec::new(); num_items);
    let mut col = 0;
    for (i, value) in formatted_values.split(";").enumerate() {
        if i % 2 == 0 {
            if headers.len() < num_columns {
                headers.push(value);
            }
        } else {
            values[curr].push(value);
            col += 1;
            if col == num_columns {
                curr += 1;
                col = 0;
            }
        }
    }
    
    // calculate the max length for each column
    for row in 0..values.len() {
        for col in 0..values[row].len() {
            max_length[col] = max(max_length[col], headers[col].chars().count());
            max_length[col] = max(max_length[col], values[row][col].chars().count());
        }
    }

    // print top border
    for col in 0..num_columns {
        table += edge;
        length = 0;
        while length < max_length[col]+2 {
            table += "-";
            length += 1;
        }
        if col == num_columns-1 {
            table += edge;
        }
    }

    // print column headers
    table += "\n";
    table += left_border;
    for (col, header) in headers.iter().enumerate() {
        table += header;
        length = header.chars().count();
        while length < max_length[col] {
            table += " ";
            length += 1;
        }
        if col != num_columns-1 {
            table += separator;
        }
    }
    table += right_border;
    table += "\n";

    // print middle
    for col in 0..num_columns {
        table += edge;
        length = 0;
        while length < max_length[col]+2 {
            table += "-";
            length += 1;
        }
        if col == num_columns-1 {
            table += edge;
        }
    }

    // print values
    for row in 0..values.len() {
        table += "\n";
        table += left_border;
        for (col, value) in values[row].iter().enumerate() {
            table += value;
            length = value.chars().count();
            while length < max_length[col] {
                table += " ";
                length += 1;
            }
            if col != num_columns-1 {
                table += separator;
            }
        }
        table += right_border;
    }
    table += "\n";

    // print bottom border
    for col in 0..num_columns {
        table += edge;
        length = 0;
        while length < max_length[col]+2 {
            table += "-";
            length += 1;
        }
        if col == num_columns-1 {
            table += edge;
        }
    }

    return table;
}
