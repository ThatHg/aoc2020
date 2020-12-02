mod pipe;

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
            _ => println!("No solution for puzzle: {}", puzzle),
        },
        None => println!("No input..."),
    }
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
