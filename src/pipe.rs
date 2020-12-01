pub struct PipeReader;
impl Iterator for PipeReader {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        let mut input = String::new();
        match std::io::stdin().read_line(&mut input) {
            Ok(n) => {
                if n != 0 {
                    return Some(input);
                }
                None
            }
            Err(error) => {
                println!("error: {}", error);
                None
            }
        }
    }
}
