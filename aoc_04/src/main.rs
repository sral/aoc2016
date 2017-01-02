use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::cmp::Ordering;
use std::collections::HashMap;

// --- Day 4: Security Through Obscurity ---
//
// Finally, you come across an information kiosk with a list of rooms.
// Of course, the list is encrypted and full of decoy data, but the instructions
// to decode the list are barely hidden nearby. Better remove the decoy data first.
//
// Each room consists of an encrypted name (lowercase letters separated by dashes)
// followed by a dash, a sector ID, and a checksum in square brackets.
//
// A room is real (not a decoy) if the checksum is the five most common letters in
// the encrypted name, in order, with ties broken by alphabetization. For example:
//
// aaaaa-bbb-z-y-x-123[abxyz] is a real room because the most common letters
// are a (5), b (3), and then a tie between x, y, and z, which are listed alphabetically.
//
// a-b-c-d-e-f-g-h-987[abcde] is a real room because although the letters are all tied (1 of each),
// the first five are listed alphabetically.
//
// not-a-real-room-404[oarel] is a real room.
//
// totally-real-room-200[decoy] is not.
//
// Of the real rooms from the list above, the sum of their sector IDs is 1514.
//
// What is the sum of the sector IDs of the real rooms?
//
//--- Part Two ---
//
// With all the decoy data out of the way, it's time to decrypt this list and get moving.
//
// The room names are encrypted by a state-of-the-art shift cipher, which is nearly
// unbreakable without the right software. However, the information kiosk designers at
// Easter Bunny HQ were not expecting to deal with a master cryptographer like yourself.
//
// To decrypt a room name, rotate each letter forward through the alphabet a number of times
// equal to the room's sector ID. A becomes B, B becomes C, Z becomes A, and so on. Dashes
// become spaces.
//
// For example, the real name for qzmt-zixmtkozy-ivhz-343 is very encrypted name.
//
// What is the sector ID of the room where North Pole objects are stored?

const A: u8 = b'a';
const Z: u8 = b'z';
const DASH: char = '-';
const SPACE: char = ' ';
const L_SQUARE_BRACKET: char = '[';

fn get_most_frequent(char_freq: &HashMap<char, i32>, n: usize) -> Vec<(&char, &i32)> {
    let mut count_vec: Vec<(&char, &i32)> = char_freq.iter().collect();
    count_vec.sort_by(|a, b| {
        match b.1.cmp(a.1) {
            Ordering::Equal => b.0.cmp(a.0).reverse(),
            other => other,
        }
    });

    count_vec[0..n].to_vec()
}

fn build_frequency_table(encrypted_name: &str) -> HashMap<char, i32> {
    let mut char_freq: HashMap<char, i32> = HashMap::new();
    for c in encrypted_name.chars() {
        if c == DASH {
            continue;
        }
        let counter = char_freq.entry(c).or_insert(0);
        *counter += 1;
    }

    char_freq
}

fn is_real_room(char_freq: &HashMap<char, i32>, checksum: &str) -> bool {
    let most_frequent = get_most_frequent(char_freq, 5);
    let parsed_checksum: String = most_frequent
        .iter()
        .map(|a| *a.0)
        .collect();

    parsed_checksum == checksum
}

fn increment(mut c: u8, count: i32) -> u8 {
    if c == DASH as u8 {
        return SPACE as u8;
    }
    for _ in 0..count {
        c += 1;
        if c > Z {
            c = A
        }
    }

    c
}

fn decrypt(secret: &str, count: i32) -> String {
    let secret: String = secret
        .as_bytes()
        .iter()
        .map(|c| increment(*c, count) as char)
        .collect();

    secret
}

fn main() {
    let path = Path::new("input");
    let file = BufReader::new(File::open(&path).expect("Couldn't open file."));

    let mut sum_room_ids = 0;
    for room in file.lines() {
        match room {
            Ok(r) => {
                match r.rfind(DASH) {
                    Some(i) => {
                        let encrypted_name = &r[0..i];
                        let id_and_checksum = &r[i+1..];
                        let v: Vec<&str> = id_and_checksum
                            .split(L_SQUARE_BRACKET)
                            .collect();
                        let room_id: i32 = v[0].parse().unwrap();
                        let mut checksum = String::from(v[1]);
                        checksum.pop();

                        let char_freq = build_frequency_table(encrypted_name);
                        if is_real_room(&char_freq, &checksum) {
                            sum_room_ids += room_id;
                        };

                        println!("id: {}, decrypted: {}",
                            room_id, decrypt(encrypted_name, room_id));
                    },
                    None => panic!("Bork. Bork.")
                };
            }
            Err(_) => panic!("Bork. Bork."),
        }
    }
    println!("Sum of room IDs: {}", sum_room_ids);
}
