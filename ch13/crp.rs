use rust_utils_crp::file_utils;

fn main() {
    let content = file_utils::read_file("path/to/file.txt");
    println!("{}", content);
}
