use std::process::Command;
use std::process::Stdio;
use std::thread;
use std::time::{Duration, Instant};

fn main() -> Result<(), String> {
    // let now = Instant::now();
    let (cols, lines) = get_term_size()?;
    // println!("{}", now.elapsed().as_millis());
    // thread::sleep_ms(2000);

    clear_screen();
    print_at_pos("Hello!", cols / 2, lines / 2);
    println!("Next2");
    set_cursor_at_pos(1, 1);
    thread::sleep_ms(2000);
    clear_screen();
    // println!("\x1b[0;31mHello\x1b[0m")
    Ok(())
}

fn get_term_size() -> Result<(usize, usize), String> {
    match Command::new("stty")
        .arg("size")
        .stdin(Stdio::inherit())
        .output()
    {
        Ok(output) => {
            let res = String::from_utf8_lossy(&output.stdout);
            let parts: Vec<&str> = res.split_whitespace().collect();
            let cols = parts[0].parse::<usize>().unwrap();
            let lines = parts[1].parse::<usize>().unwrap();

            return Ok((cols, lines));
        }
        Err(_) => return Err("Could not determine term size".to_string()),
    }
}

fn clear_screen() {
    println!("\x1b[0;0H\x1b[2J");
}

fn set_cursor_at_pos(col: usize, line: usize) {
    print!("\x1b[{};{}H", col, line);
}

fn print_at_pos(s: &str, col: usize, line: usize) {
    set_cursor_at_pos(col, line);
    print!("{}", s);
}
