mod pipe;

fn reader() -> pipe::PipeReader {
    pipe::PipeReader {}
}

fn main() {
    match reader().next() {
        Some(puzzle) => match puzzle.trim() {
            "day1_1" => day1_1(),
            "day1_2" => day1_2(),
            _ => println!("No solution for puzzle: {}", puzzle),
        },
        None => println!("No input..."),
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
