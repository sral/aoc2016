// --- Day 6: Signals and Noise ---
//
// Something is jamming your communications with Santa. Fortunately, your signal
// is only partially jammed, and protocol in situations like this is to switch
// to a simple repetition code to get the message through.
//
// In this model, the same message is sent repeatedly. You've recorded the repeating
// message signal (your puzzle input), but the data seems quite corrupted - almost too
// badly to recover. Almost.
//
// All you need to do is figure out which character is most frequent for each position.
// For example, suppose you had recorded the following messages:
//
// eedadn
// drvtee
// eandsr
// raavrd
// atevrs
// tsrnev
// sdttsa
// rasrtv
// nssdts
// ntnada
// svetve
// tesnvt
// vntsnd
// vrdear
// dvrsen
// enarar
//
// The most common character in the first column is e; in the second, a; in the third, s, and
// so on. Combining these characters returns the error-corrected message, easter.
//
// Given the recording in your puzzle input, what is the error-corrected version of the message
// being sent?
//
// --- Part Two ---
//
// Of course, that would be the message - if you hadn't agreed to use a modified repetition
// code instead.
//
// In this modified code, the sender instead transmits what looks like random data, but for each
// character, the character they actually want to send is slightly less likely than the others.
// Even after signal-jamming noise, you can look at the letter distributions in each column and
// choose the least common letter to reconstruct the original message.
//
// In the above example, the least common character in the first column is a; in the second, d, and
// so on. Repeating this process for the remaining characters produces the original message,
// advent.

// Given the recording in your puzzle input and this new decoding methodology, what is the original
// message that Santa is trying to send?

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

const COLUMNS: usize = 8;

fn find_max(v: &[u32]) -> Option<usize> {
    let mut max_index = None;
    let mut max = 0;

    for (i, n) in v.iter().enumerate() {
        if n > &max {
            max = *n;
            max_index = Some(i);
        }
    }
    max_index
}

fn find_min(v: &[u32]) -> Option<usize> {
    let mut min_index = None;
    // The range we're dealing with is [0, 25]
    let mut min = u32::max_value();

    for (i, n) in v.iter().enumerate() {
        if n > &0 && n < &min {
            min = *n;
            min_index = Some(i);
        }
    }
    min_index
}

fn main() {
    let path = Path::new("input");
    let file = BufReader::new(File::open(&path).expect("Couldn't open file."));

    // 8 x 26 matrix for frequency tables
    let mut freq: Vec<Vec<u32>> = vec![vec![0; 26]; COLUMNS];

    for line in file.lines() {
        match line {
            Ok(l) => {
                for (col, c) in l.chars()
                    // Assume all lower-case ascii, convert to range [0, 25]
                    // for frequency tables.
                    .map(|n| n as usize - 97)
                    .enumerate() {
                        freq[col][c] += 1;
                }
            },
            Err(_) => panic!("Bork. Bork.")
        }
    }

    // Convert range back to [97, 122] (lower-case ascii) to allow converting to string.
    let most_common: Vec<u8> = freq.iter()
        .map(|v| (find_max(&v).unwrap() + 97) as u8)
        .collect();
    let least_common: Vec<u8> = freq.iter()
        .map(|v| (find_min(&v).unwrap() + 97) as u8)
        .collect();
    let error_corrected_message = String::from_utf8(most_common).unwrap();
    let original_message = String::from_utf8(least_common).unwrap();
    println!("Error-corrected message: {}", error_corrected_message);
    println!("Original message: {}", original_message);
}
