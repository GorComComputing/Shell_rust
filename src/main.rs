use std::io::{self, stdin, Write}; // Импорт для работы с вводом/выводом
use std::process::Command; // Импорт для работы с командами процесса

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    // read_line оставляет завершающий символ новой строки, который trim удаляет
    let command = input.trim(); 

    Command::new(command)
        .spawn()
        .unwrap();
}

