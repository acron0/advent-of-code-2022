use crate::util;

const UPPERCASE_PRIORITY_OFFSET: u8 = 38;
const LOWERCASE_PRIORITY_OFFSET: u8 = 96;

fn compartment_to_priorities(size: usize, compartment: &str) -> Vec<u8> {
    let mut vec = Vec::with_capacity(size);
    for c in compartment.chars() {
        let mut n = c as u8;
        // is_upper?
        if n >= 65 && n <= 90 {
            n -= UPPERCASE_PRIORITY_OFFSET;
        } else { // must be lower
            n -= LOWERCASE_PRIORITY_OFFSET;
        }
        //println!("-- {} {}", c, n);
        vec.push(n);
    }
    return vec
}

fn cheap_intersect(a: Vec<u8>, b: Vec<u8>) -> u8 {
    let difference: Vec<_> = a.into_iter().filter(|item| b.contains(item)).collect();
    return difference[0];
}

pub fn run1() -> u32 {
    if let Ok(lines) = util::read_lines("./inputs/day3") {
        let mut accumulated_priority: u32 = 0;
        for line in lines {
            if let Ok(rucksack) = line {
                let compartment_size = rucksack.chars().count() / 2;
                let first_compartment  = &rucksack[..compartment_size];
                let second_compartment = &rucksack[compartment_size..];
                let first_compartment_priorities  = compartment_to_priorities(compartment_size, first_compartment);
                let second_compartment_priorities = compartment_to_priorities(compartment_size, second_compartment);
                let intersection = cheap_intersect(first_compartment_priorities, second_compartment_priorities);

                accumulated_priority += intersection as u32;
                //println!("1: {} 2: {} | {}", first_compartment, second_compartment, intersection);
            }
        }
        return accumulated_priority
    } else {
        return 0
    }
}
pub const EXPECTED_RESULT1: u32 = 7716;

////////////////////////////////////////////////////////////////////////////////

pub fn run2() -> usize {
    0
}
pub const EXPECTED_RESULT2: usize = 123;
