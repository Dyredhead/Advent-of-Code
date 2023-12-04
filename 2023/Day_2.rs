use std::fs;
use std::collections::HashMap;

static FILE_PATH: &str = "Day_2.txt";

fn main() {
	println!("{}", pt_1());
	println!("{}", pt_2());
}

fn pt_1() -> i32 {
    let map = HashMap::from([ 
        ("red", 12),
        ("green", 13),
        ("blue", 14)
    ]);

    let mut sum: i32 = 0;
    for game in fs::read_to_string(FILE_PATH).unwrap().lines() {
        let game: Vec<&str> = game.split(":").collect();
        let id: i32 = game[0].split(" ").collect::<Vec<&str>>()[1].parse().unwrap();
        let game: Vec<&str> = game[1].split(";").collect();

        let mut valid_game: bool = true;
        for set in game {
            let set: Vec<&str> = set.split(",").collect();
            let mut valid_set: bool = true;
            for cubes in set {
                let cubes: Vec<&str> = cubes[1..].split(" ").collect();
                let n: i32 = cubes[0].parse().unwrap();
                let color: &str = cubes[1];
                if n > *map.get(color).unwrap() {
                    valid_set = false;
                    break;
                }
            }
            if !valid_set {
                valid_game = false;
                break;
            }
        }
        if valid_game {sum += id;}
    }
    return sum;
}
fn pt_2() -> i32 {
    let mut sum: i32 = 0;
    for game in fs::read_to_string(FILE_PATH).unwrap().lines() {
        let game: Vec<&str> = game.split(":").collect::<Vec<&str>>()[1].split(";").collect();

        let mut map = HashMap::from([ 
            ("red", 0),
            ("green", 0),
            ("blue", 0)
        ]);
        for set in game {
            let set: Vec<&str> = set.split(",").collect();
            for cubes in set {
                let cubes: Vec<&str> = cubes[1..].split(" ").collect();
                let n: i32 = cubes[0].parse().unwrap();
                let color: &str = cubes[1];

                if n > *map.get(color).unwrap() {
                    map.insert(color, n);
                }
            }
        }
        let mut power: i32 = 1;
        for val in map.values() {
            power *= val;
        }
        sum += power;
    } 
    return sum;
}