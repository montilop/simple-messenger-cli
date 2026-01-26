use std::io::{self, Write};

fn main() {
    let mut messages: Vec<String> = Vec::new();

    loop {
        println!("\n=== Simple CLI Messenger ===");
        println!("1. Написать сообщение");
        println!("2. Показать все сообщения");
        println!("3. Выход");
        print!("Выберите опцию: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();

        match choice {
            "1" => {
                print!("Введите сообщение: ");
                io::stdout().flush().unwrap();
                let mut msg = String::new();
                io::stdin().read_line(&mut msg).unwrap();
                let msg = msg.trim().to_string();
                messages.push(msg);
                println!("Сообщение отправлено!");
            }
            "2" => {
                println!("\n=== Все сообщения ===");
                for (i, msg) in messages.iter().enumerate() {
                    println!("{}: {}", i + 1, msg);
                }
            }
            "3" => {
                println!("Выход...");
                break;
            }
            _ => println!("Неверная опция!"),
        }
    }
}
