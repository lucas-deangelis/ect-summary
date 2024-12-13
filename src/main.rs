use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let total_saved: f64 = stdin
        .lock()
        .lines()
        .filter_map(|line| line.ok())
        .filter_map(|line| {
            if line.starts_with("Saved ") {
                let parts: Vec<&str> = line.split(" ").collect();
                if parts.len() >= 4 {
                    let saved_size = parts[1];
                    parse_size(saved_size)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .sum();

    println!("Total saved: {:.2}KB", total_saved / 1024.0);
    Ok(())
}

fn parse_size(size_str: &str) -> Option<f64> {
    if size_str.ends_with("KB") {
        let num_str = &size_str[..size_str.len() - 2];
        num_str.parse::<f64>().ok().map(|x| x * 1024.0)
    } else if size_str.ends_with("MB") {
        let num_str = &size_str[..size_str.len() - 2];
        num_str.parse::<f64>().ok().map(|x| x * 1024.0 * 1024.0)
    } else if size_str.ends_with("B") {
        let num_str = &size_str[..size_str.len() - 1];
        num_str.parse::<f64>().ok()
    } else {
        None
    }
}