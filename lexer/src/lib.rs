mod file;
mod types;
mod lexer;
mod tokenizer;

pub fn run(file_path: &str) {
    let file_content = file::get_file_content(&file_path);
    let mut lexer = lexer::Lexer::new(file_content);
    let tokens = lexer.run();

    println!("{:?}", tokens);
}