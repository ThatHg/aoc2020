mod pipe;
use regex::Regex;
use std::collections::HashMap;

fn reader() -> pipe::PipeReader {
    pipe::PipeReader {}
}

fn main() {
    match reader().next() {
        Some(puzzle) => match puzzle.trim() {
            "day1_1" => day1_1(),
            "day1_2" => day1_2(),
            "day2_1" => day2_1(),
            "day2_2" => day2_2(),
            "day3_1" => day3_1(),
            "day3_2" => day3_2(),
            "day4_1" => day4_1(),
            "day4_2" => day4_2(),
            _ => println!("No solution for puzzle: {}", puzzle),
        },
        None => println!("No input..."),
    }
}

fn validate_fields(passport: &String) -> bool {
    let mut validation: HashMap<&str, Box<dyn Fn(&&str) -> bool>> = HashMap::new();
    validation.insert(
        "byr",
        Box::new(|value| -> bool {
            if value.len() != 4 {
                return false;
            }
            match value.parse::<i32>() {
                Ok(r) => {
                    if r >= 1920 && r <= 2002 {
                        return true;
                    }
                }
                Err(e) => println!("ERROR: [BYR] {}", e),
            }
            println!("  - BYR invalid {}", value);
            return false;
        }),
    );
    validation.insert(
        "iyr",
        Box::new(|value| -> bool {
            if value.len() != 4 {
                return false;
            }
            match value.parse::<i32>() {
                Ok(r) => {
                    if r >= 2010 && r <= 2020 {
                        return true;
                    }
                }
                Err(e) => println!("ERROR: [IYR] {}", e),
            }
            println!("  - IYR invalid {}", value);
            return false;
        }),
    );
    validation.insert(
        "eyr",
        Box::new(|value| -> bool {
            if value.len() != 4 {
                return false;
            }
            match value.parse::<i32>() {
                Ok(r) => {
                    if r >= 2020 && r <= 2030 {
                        return true;
                    }
                }
                Err(e) => println!("ERROR: [EYR] {}", e),
            }
            println!("  - EYR invalid {}", value);
            return false;
        }),
    );
    validation.insert(
        "hgt",
        Box::new(|value| -> bool {
            if value.len() < 4 {
                return false;
            }
            let chars: Vec<char> = value.chars().collect();
            let unit: String = chars[chars.len() - 2..].into_iter().collect();
            let height: String = value.chars().take(value.len() - 2).collect();
            if unit == "in" {
                match height.parse::<i32>() {
                    Ok(r) => {
                        if r >= 59 && r <= 76 {
                            return true;
                        }
                    }
                    Err(e) => println!("ERROR: [HGT] {}", e),
                }
            } else if unit == "cm" {
                match height.parse::<i32>() {
                    Ok(r) => {
                        if r >= 150 && r <= 193 {
                            return true;
                        }
                    }
                    Err(e) => println!("ERROR: [HGT] {}", e),
                }
            }
            println!("  - HGT invalid {}", value);
            return false;
        }),
    );
    validation.insert(
        "hcl",
        Box::new(|value| -> bool {
            let valid = Regex::new("^#[0-9a-f]{6}$").unwrap().is_match(value);
            if !valid {
                println!("  - HCL invalid {}", value);
            }
            return valid;
        }),
    );
    validation.insert(
        "ecl",
        Box::new(|value| -> bool {
            let valid = Regex::new("^(amb|blu|brn|gry|grn|hzl|oth)$")
                .unwrap()
                .is_match(value);
            if !valid {
                println!("  - ECL invalid {}", value);
            }
            return valid;
        }),
    );
    validation.insert(
        "pid",
        Box::new(|value| -> bool {
            let valid = Regex::new("^[0-9]{9}$").unwrap().is_match(value);
            if !valid {
                println!("  - PID invalid {}", value);
            }
            return valid;
        }),
    );
    let fields: Vec<Vec<&str>> = passport
        .split(" ")
        .map(|key_value| return key_value.split(":").collect())
        .collect();
    let mut valid = true;
    for f in &fields {
        if &f[0] == &"cid" {
            continue;
        }
        match validation.get(&f[0]) {
            Some(validation_func) => {
                if !validation_func(&f[1]) {
                    valid = false;
                }
            }
            None => valid = false,
        }
    }
    return valid;
}

fn has_fields(passport: &String) -> bool {
    let valid_fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let fields: Vec<&str> = passport
        .split(" ")
        .map(|key_value| return &key_value[..3])
        .collect();
    for i in 0..valid_fields.len() {
        let mut valid = false;
        for f in &fields {
            if *f == valid_fields[i] {
                valid = true;
                break;
            }
        }
        if !valid {
            println!("  - missing field {}", valid_fields[i]);
            return false;
        }
    }
    return true;
}

fn day4_2() {
    let mut valid_count: i32 = 0;
    loop {
        let mut passport: String = String::new();
        for line in reader() {
            if line == "\n" || line == "" {
                break;
            }
            if passport.len() > 0 {
                passport.push_str(" ");
            }
            passport.push_str(line.trim());
        }

        if passport.len() == 0 {
            break;
        }

        println!("{}", passport);
        if has_fields(&passport) && validate_fields(&passport) {
            valid_count += 1;
            println!("  - VALID");
        } else {
            println!("  - INVALID");
        }
        passport.clear();
    }

    println!("Valid: {}", valid_count);
}

fn day4_1() {
    let mut valid_count: i32 = 0;
    loop {
        let mut passport: String = String::new();
        for line in reader() {
            if line == "\n" || line == "" {
                break;
            }
            if passport.len() > 0 {
                passport.push_str(" ");
            }
            passport.push_str(line.trim());
        }

        if passport.len() == 0 {
            break;
        }

        if has_fields(&passport) {
            valid_count += 1;
            println!("OK {}", passport);
        } else {
            println!("FAIL {}", passport);
        }
        passport.clear();
    }

    println!("Valid: {}", valid_count);
}

fn traverse(woods: &Vec<Vec<char>>, right: usize, down: usize, tree: char) -> usize {
    let mut x: usize = 0;
    let mut tree_count: usize = 0;
    for tree_line in woods.iter().step_by(down) {
        if tree_line[x % tree_line.len()] == tree {
            tree_count += 1;
        }
        x = x + right;
    }
    println!(
        "right={}, down={}, tree={}, tree_count={}",
        right, down, tree, tree_count
    );
    return tree_count;
}

fn day3_2() {
    println!("=== Day 3 | Puzzle 2 ===");
    let mut woods: Vec<Vec<char>> = Vec::new();
    for line in reader() {
        woods.push(line.trim().chars().collect());
    }

    let a = traverse(&woods, 1, 1, '#');
    let b = traverse(&woods, 3, 1, '#');
    let c = traverse(&woods, 5, 1, '#');
    let d = traverse(&woods, 7, 1, '#');
    let e = traverse(&woods, 1, 2, '#');
    println!("day3 answer: {}", a * b * c * d * e);
}

fn day3_1() {
    println!("=== Day 3| Puzzle 1 ===");
    let mut woods: Vec<Vec<char>> = Vec::new();
    for line in reader() {
        woods.push(line.trim().chars().collect());
    }
    traverse(&woods, 3, 1, '#');
}

#[derive(Debug)]
struct Password {
    min: i32,
    max: i32,
    character: char,
    whitespace_count: i32,
    password: String,
    ok: bool,
}

fn day2_2() {
    println!("=== Day 2 | Puzzle 2 ===");
    // Input = "15-16 f: ffffffffffffffhf"
    let mut valid_count: i32 = 0;
    for line in reader() {
        let mut chars: Vec<char> = Vec::new();
        let mut password: Password = Password {
            min: 0,
            max: 0,
            character: '.',
            whitespace_count: 0,
            password: String::new(),
            ok: false,
        };
        for c in line.chars() {
            if c == '-' {
                password.min = chars.iter().collect::<String>().parse::<i32>().unwrap_or(0);
                chars.clear();
            } else if c == ' ' {
                if password.whitespace_count == 0 {
                    password.max = chars.iter().collect::<String>().parse::<i32>().unwrap_or(0);
                }
                password.whitespace_count += 1;
                chars.clear();
            } else if c == ':' {
                password.character = chars[0];
                chars.clear();
            } else {
                chars.push(c)
            }
        }
        password.password = chars.iter().collect::<String>();

        let mut only_one: usize = 0;
        if password.password.chars().collect::<Vec<char>>()[(password.min - 1) as usize]
            == password.character
        {
            only_one += 1;
        }
        if password.password.chars().collect::<Vec<char>>()[(password.max - 1) as usize]
            == password.character
        {
            only_one += 1;
        }
        if only_one == 1 {
            password.ok = true;
            valid_count += 1;
        }

        println!("{:?}", password);
    }
    println!("valid password count: {}", valid_count);
}

fn day2_1() {
    println!("=== Day 2 | Puzzle 1 ===");
    // Input = "15-16 f: ffffffffffffffhf"
    let mut valid_count: i32 = 0;
    for line in reader() {
        let mut chars: Vec<char> = Vec::new();
        let mut password: Password = Password {
            min: 0,
            max: 0,
            character: '.',
            whitespace_count: 0,
            password: String::new(),
            ok: false,
        };
        for c in line.chars() {
            if c == '-' {
                password.min = chars.iter().collect::<String>().parse::<i32>().unwrap_or(0);
                chars.clear();
            } else if c == ' ' {
                if password.whitespace_count == 0 {
                    password.max = chars.iter().collect::<String>().parse::<i32>().unwrap_or(0);
                }
                password.whitespace_count += 1;
                chars.clear();
            } else if c == ':' {
                password.character = chars[0];
                chars.clear();
            } else {
                chars.push(c)
            }
        }
        password.password = chars.iter().collect::<String>();

        let mut occurances: i32 = 0;
        for c in password.password.chars() {
            if password.character == c {
                occurances += 1;
            }
        }
        if occurances >= password.min && occurances <= password.max {
            password.ok = true;
            valid_count += 1;
        }

        println!("{:?}", password);
    }
    println!("valid password count: {}", valid_count);
}

fn day1_2() {
    println!("=== Day 1 | Puzzle 2 ===");
    let mut numbers = Vec::new();
    for line in reader() {
        let num = line.trim().parse::<i32>().unwrap_or(0);
        for i in numbers.iter() {
            for j in numbers.iter() {
                if num + i + j == 2020 {
                    println!("{}*{}*{}={}", num, i, j, num * i * j);
                    break;
                }
            }
        }
        numbers.push(num);
    }
}

fn day1_1() {
    println!("=== Day 1 | Puzzle 1 ===");

    let mut numbers = Vec::new();
    for line in reader() {
        let num = line.trim().parse::<i32>().unwrap_or(0);
        for i in numbers.iter() {
            if num + i == 2020 {
                println!("{}*{}={}", num, i, num * i);
                break;
            }
        }
        numbers.push(num);
    }
}
