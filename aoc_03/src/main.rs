use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

#[derive(Debug)]
struct Triangle {
    a: i32,
    b: i32,
    c: i32,
}

impl Triangle {
    fn is_valid(&self) -> bool {
        self.a + self.b > self.c &&
            self.a + self.c > self.b &&
            self.b + self.c > self.a
    }
}

fn part_one() {
    let path = Path::new("data");
    let file = BufReader::new(File::open(&path).expect("Couldn't open"));

    let mut valid_triangles = 0;
    for line in file.lines() {
        let triangle: Triangle;
        match line {
            Ok(l) => {
                let values = l.split_whitespace()
                    .map(|s| s.parse().expect("Bork. Bork."))
                    .collect::<Vec<_>>();
                triangle = Triangle {
                    a: values[0],
                    b: values[1],
                    c: values[2],
                };
            },
            Err(_) => panic!("Bork. Bork.")
        };
        if triangle.is_valid() {
            valid_triangles += 1;
        }
    }
    println!("PART ONE: valid triangle count: {}", valid_triangles)
}

fn part_two() {
    let path = Path::new("data");
    let file = BufReader::new(File::open(&path).expect("Couldn't open"));

    let mut valid_triangles = 0;
    let mut v = vec![];
    for line in file.lines() {
        match line {
            Ok(l) => {
                let mut values = l.split_whitespace()
                    .map(|s| s.parse().expect("Bork. Bork."))
                    .collect::<Vec<i32>>();
                v.append(&mut values);
            }
            Err(_) => panic!("Bork. Bork.")
        }
    }

    let mut i = 0;
    while i < v.len() {
        for h in 0..3 {
            let mut values = vec![0; 3];
            for n in 0..3 {
                values[n] = v[i + h + n * 3];
            }
            let triangle = Triangle {
                a: values[0],
                b: values[1],
                c: values[2],
            };
            if triangle.is_valid() {
                valid_triangles += 1;
            }
        }
        i += 9;
    }
    println!("PART TWO: valid triangle count: {}", valid_triangles)
}

fn main() {
    part_one();
    part_two();
}
