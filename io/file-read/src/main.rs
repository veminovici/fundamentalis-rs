use std::io::{BufRead, BufReader};

fn read_file_string(filepath: &str) -> std::io::Result<String> {
    let data = std::fs::read_to_string(filepath);
    data
}

fn read_file_vec(filepath: &str) -> std::io::Result<Vec<u8>> {
    let data = std::fs::read(filepath);
    data
}

fn read_file_line_by_line(filepath: &str) -> std::io::Result<()> {
    let file = std::fs::File::open(filepath)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        println!("{}", line?);
    }

    Ok(())
}

fn main() {
    read_file_line_by_line("./test.md").unwrap();
}
