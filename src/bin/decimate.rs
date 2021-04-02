use std::env;
use std::io;
use std::io::Read;
use std::io::Write;

const MAX_DECIMATION_FACTOR: usize = 128;

fn pass(decimation_factor: u64) {
    let mut stdout = io::stdout();
    let mut stdin = io::stdin();

    let mut buffer = [0; MAX_DECIMATION_FACTOR * 2];
    let bytes_to_read: usize = (decimation_factor * 2) as usize;
    loop {
        let mut restricted_input = stdin.by_ref().take(decimation_factor * 2);
        let result = restricted_input.read(&mut buffer);
        let mut bytes_to_write = 0;
        match result {
            Ok(n) if n == bytes_to_read  => bytes_to_write = 2,
            Ok(n) => break,
            Err(..) => break,
        }
        if bytes_to_write > 0 {
            stdout.write(&buffer[0..bytes_to_write]);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let decimation_factor = &args[1].parse::<u64>();
    match decimation_factor {
        Ok(n) => pass(*n),
        Err(..) => {},
    }
}
