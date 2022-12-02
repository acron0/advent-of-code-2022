use crate::util;
use std::vec::Vec;

pub fn run1() -> u32{
    if let Ok(lines) = util::read_lines("./inputs/day1") {
        // Consumes the iterator, returns an (Optional) String
        let mut block: Vec<u32> = Vec::new();
        let mut largest: u32 = 0;
        for line in lines {
            if let Ok(calories_line) = line {
                //println!("calories_line {}", calories_line);
                if let Ok(c) = calories_line.parse::<u32>() {
                    // we got a value; add it to the block
                    block.push(c)
                } else {
                    // no value; sum the block, clear the block and save if
                    // it's the largest sum so far
                    let s = block.iter().sum();
                    block.clear();
                    //println!("total {}\n", s);
                    if s > largest {
                        largest = s
                    }
                }
            }
        }
        // FIXME: what if we have an unsummed block when we finish parsing lines?
        // AW: I will add empty line at end of file for now but it's not safe
        return largest
    } else {
        return 0
    }
}
pub const EXPECTED_RESULT1: u32 = 69177;

////////////////////////////////////////////////////////////////////////////////

pub fn run2() -> u32{
    if let Ok(lines) = util::read_lines("./inputs/day1") {
        // Consumes the iterator, returns an (Optional) String
        let mut block: Vec<u32> = Vec::new();
        let mut largest: Vec<u32> = vec![0,0,0];
        for line in lines {
            if let Ok(calories_line) = line {
                //println!("calories_line {}", calories_line);
                if let Ok(c) = calories_line.parse::<u32>() {
                    // we got a value; add it to the block
                    block.push(c)
                } else {
                    // no value; sum the block, clear the block and save if
                    // it's the largest sum so far
                    let s = block.iter().sum();
                    block.clear();
                    //println!("total {}\n", s);
                    for l in largest.iter_mut() {
                        if s > *l {
                            *l = s;
                            break;
                        }
                    }
                    // we use this to make sure the lowest value is always first
                    // in the array, which is critical to the for loop logic
                    // above to work
                    largest.sort();
                    //println!("largest {}, {}, {}\n", largest[0], largest[1], largest[2]);
                }
            }
        }
        // FIXME: what if we have an unsummed block when we finish parsing lines?
        // AW: I will add empty line at end of file for now but it's not safe
        return largest.iter().sum()
    } else {
        return 0
    }
}
pub const EXPECTED_RESULT2: u32 = 69177;
