use std::collections::HashSet;
use std::ops::Mul;
use std::ops::Add;
use std::ops::Sub;

#[derive(Hash, Eq, PartialOrd, PartialEq, Debug, Copy, Clone)]
struct Vec2d<T> {
    x: T,
    y: T,
}

impl<T: Add<Output = T>> Add for Vec2d<T> {
    type Output = Vec2d<T>;

    fn add(self, other: Vec2d<T>) -> Vec2d<T> {
        Vec2d {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T: Sub<Output = T>> Sub for Vec2d<T> {
    type Output = Vec2d<T>;

    fn sub(self, other: Vec2d<T>) -> Vec2d<T> {
        Vec2d {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T: Mul<Output = T> + Add<Output = T> + Copy> Vec2d<T> {
    fn scalar(&mut self, n: T) -> Vec2d<T> {
        Vec2d {
            x: self.x * n,
            y: self.y * n
        }
    }

//     fn dot(&self, other: &Vec2d<T>) -> T {
//         self.x * other.x + self.y * other.y
//     }

//    fn manhattan_distance(&self) -> T {
//        self.x.abs() + self.y.abs()
//    }
}

fn main() {
    let s = "L4 R2 R4 L5 L3 L1 R4 R5 R1 R3 L3 L2 L2 R5 R1 L1 L2 R2 R2 L5 R5 \
        R5 L2 R1 R2 L2 L4 L1 R5 R2 R1 R1 L2 L3 R2 L5 L186 L5 L3 R3 L5 R4 R2 L5 \
        R1 R4 L1 L3 R3 R1 L1 R4 R2 L1 L4 R5 L1 R50 L4 R3 R78 R4 R2 L4 R3 L4 R4 \
        L1 R5 L4 R1 L2 R3 L2 R5 R5 L4 L1 L2 R185 L5 R2 R1 L3 R4 L5 R2 R4 L3 R4 \
        L2 L5 R1 R2 L2 L1 L2 R2 L2 R1 L5 L3 L4 L3 L4 L2 L5 L5 R2 L3 L4 R4 R4 \
        R5 L4 L2 R4 L5 R3 R1 L1 R3 L2 R2 R1 R5 L4 R5 L3 R2 R3 R1 R4 L4 R1 R3 \
        L5 L1 L3 R2 R1 R4 L4 R3 L3 R3 R2 L3 L3 R4 L2 R4 L3 L4 R5 R1 L1 R5 R3 \
        R1 R3 R4 L1 R4 R3 R1 L5 L5 L4 R4 R3 L2 R1 R5 L3 R4 R5 L4 L5 R2";


    let mut visited = HashSet::new();
    let mut magnitude: i32;
    let mut direction = Vec2d {
        x: 0,
        y: 1
    };
    let mut position = Vec2d {
        x: 0,
        y: 0
    };
    visited.insert(position);

    let instructions = s.split(' ');
    for instruction in instructions {
        let mut c_itr = instruction.chars();
        let rotation = c_itr.next().unwrap();
        match rotation {
            'L' => {
                let tmp = direction.x;
                direction.x = -direction.y;
                direction.y = tmp;
                },
            'R' => {
                let tmp = direction.x;
                direction.x = direction.y;
                direction.y = -tmp;
            },
            _ => panic!("Bork. Bork")
        }

        let d = c_itr.collect::<String>();
        magnitude = d.parse().expect("Not a number.");

        let walk = direction.scalar(magnitude);
        let new_position = position + walk;

        // Store visited points
        for n in 1..magnitude + 1 {
            let visit = position + direction.scalar(n);
            if visited.contains(&visit) {
                println!("Already visited x: {}, y: {}, distance: {}",
                         visit.x, visit.y, visit.x.abs() + visit.y.abs());
            } else {
                visited.insert(visit);
            }
        }

        position = new_position;
    }

    println!("Final position: x: {}, y: {}, distance: {}",
             position.x, position.y, position.x.abs() + position.y.abs());
}