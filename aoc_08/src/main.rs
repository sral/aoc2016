// --- Day 8: Two-Factor Authentication ---
//
// You come across a door implementing what you can only assume is an implementation of two-factor
// authentication after a long game of requirements telephone.
//
// To get past the door, you first swipe a keycard (no problem; there was one on a nearby desk).
// Then, it displays a code on a little screen, and you type that code on a keypad. Then,
// presumably, the door unlocks.
//
// Unfortunately, the screen has been smashed. After a few minutes, you've taken everything
// apart and figured out how it works. Now you just have to work out what the screen would have
// displayed.
//
// The magnetic strip on the card you swiped encodes a series of instructions for the screen; these
// instructions are your puzzle input. The screen is 50 pixels wide and 6 pixels tall, all of which
// start off, and is capable of three somewhat peculiar operations:
//
// - rect AxB turns on all of the pixels in a draw_rect at the top-left of the screen which is
//   A wide and B tall.
// - rotate row y=A by B shifts all of the pixels in row A (0 is the top row) right by B pixels.
//   Pixels that would fall off the right end appear at the left end of the row.
// - rotate column x=A by B shifts all of the pixels in column A (0 is the left column) down by B
//   pixels. Pixels that would fall off the bottom appear at the top of the column.
//
// For example, here is a simple sequence on a smaller screen:
//
// - rect 3x2 creates a small draw_rect in the top-left corner:
//
//     ###....
//     ###....
//     .......
//
// - rotate column x=1 by 1 rotates the second column down by one pixel:
//
//     #.#....
//     ###....
//     .#.....
//
// - rotate row y=0 by 4 rotates the top row right by four pixels:
//
//     ....#.#
//     ###....
//     .#.....
//
// - rotate column x=1 by 1 again rotates the second column down by one pixel, causing the bottom
//   pixel to wrap back to the top:
//
//     .#..#.#
//     #.#....
//     .#.....
//
// As you can see, this display technology is extremely powerful, and will soon dominate
// the tiny-code-displaying-screen market. That's what the advertisement on the back of the display
// tries to convince you, anyway.
//
// There seems to be an intermediate check of the voltage used by the display: after you swipe your
// card, if the screen did work, how many pixels should be lit?

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

fn draw_rect(screen: &mut [Vec<u32>], w: usize, h: usize) {
    for y in 0..h {
        for x in 0..w {
            screen[y][x] = 1;
        }
    }
}

fn rotate(row: &[u32], offset: usize) -> Vec<u32> {
    // if offset == 0 {
    //     return row.to_vec();
    // }
    let mut rotated = vec![0; row.len()];

    for (i, v) in row.iter().enumerate() {
        rotated[(i + offset) % row.len()] = *v;
    }
    rotated
}

fn get_column(screen: &[Vec<u32>], col: usize) -> Vec<u32> {
    let len = screen.len();
    let mut column = vec![0; len];

    for i in 0..len {
        column[i] = screen[i][col];
    }

    column
}

fn draw_screen(screen: &[Vec<u32>]) {
    for row in screen {
        for pixel in row {
            print!("{}", if pixel == &1 { '#' } else { ' ' });
        }
        println!("");
    }
}

fn main() {
    // 6 x 50
    let mut screen = vec![
        vec![0; 50],
        vec![0; 50],
        vec![0; 50],
        vec![0; 50],
        vec![0; 50],
        vec![0; 50]
        ];

    let path = Path::new("input");
    let file = BufReader::new(File::open(&path).expect("Couldn't open file."));

    for line in file.lines() {
        match line {
            Ok(l) => {
                // There are two numbers per line. Possible cases:
                // - rotate column x=27 by 1 => indices: 4, 6
                // - rotate row y=2 by 35 => indicies: 3, 5
                // - rect 9x1 => indicies: 1, 2
                let tokens: Vec<&str> = l.split(|c| c == ' ' || c == '=' || c == 'x').collect();
                if l.starts_with("rotate row") {
                    let row: usize = tokens[3].parse().unwrap();
                    let offset: usize = tokens[5].parse().unwrap();
                    screen[row] = rotate(&screen[row], offset);
                } else if l.starts_with("rotate column") {
                    let col: usize = tokens[4].parse().unwrap();
                    let offset: usize = tokens[6].parse().unwrap();
                    let column = rotate(&get_column(&screen, col), offset);
                    for (i, v) in column.iter().enumerate() {
                        screen[i][col] = *v;
                    }
                } else if l.starts_with("rect") {
                    let w: usize = tokens[1].parse().unwrap();
                    let h: usize = tokens[2].parse().unwrap();
                    draw_rect(&mut screen, w, h);
                }
            },
            Err(_) => panic!("Bork. Bork."),
        }
    }

    let mut sum_lit_pixels = 0;
    for v in &screen {
        sum_lit_pixels += v.iter().fold(0, |sum, p| sum + p);
    }
    println!("Lit pixels: {}", sum_lit_pixels);
    draw_screen(&screen);
}
