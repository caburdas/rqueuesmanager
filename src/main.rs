mod actions;
mod cli;

use clap::Parser;
use cli::Cli;

fn main() {
    let mut cli = Cli::parse();

    enter_parameters(&mut cli);
    loop {
        match show_menu() {
            0 => break,
            1 => match actions::get_queues(&cli) {
                Ok(_) => {}
                Err(e) => print!("{}", e),
            },
            2 => match actions::delete_queues(&cli) {
                Ok(_) => {}
                Err(e) => print!("{}", e),
            },
            3 => match actions::get_exchanges(&cli) {
                Ok(_) => {}
                Err(e) => print!("{}", e),
            },
            4 => match actions::delete_exchanges(&cli) {
                Ok(_) => {}
                Err(e) => print!("{}", e),
            },
            5 => match actions::get_messages(&cli) {
                Ok(_) => {}
                Err(e) => print!("{}", e),
            },
            _ => {}
        }
    }
}

fn enter_parameters(cli: &mut Cli) {
    let mut line = String::new();
    while cli.host.is_none() {
        println!("Enter hostname : ");
        line = "".to_string();
        match std::io::stdin().read_line(&mut line) {
            Ok(_) => {
                if line.starts_with('\n') {
                    continue;
                }
                cli.host = Some(line.trim().to_string().clone())
            }
            Err(e) => println!("{:?}", e),
        }
    }

    line = "".to_string();
    loop {
        println!("Enter port (15672): ");
        line = "".to_string();
        match std::io::stdin().read_line(&mut line) {
            Ok(_) => {
                if line.starts_with('\n') {
                    cli.port = Some(15672);
                    break;
                }
                match line.trim().to_string().parse::<u16>() {
                    Ok(num) => {
                        cli.port = Some(num);
                        break;
                    }
                    Err(_) => continue,
                }
            }
            Err(e) => println!("{:?}", e),
        }
    }

    line = "".to_string();
    while cli.user.is_none() {
        println!("Enter user : ");
        line = "".to_string();
        match std::io::stdin().read_line(&mut line) {
            Ok(_) => {
                if line.starts_with('\n') {
                    continue;
                }
                cli.user = Some(line.trim().to_string().clone())
            }
            Err(e) => println!("{:?}", e),
        }
    }

    line = "".to_string();
    while cli.password.is_none() {
        println!("Enter password : ");
        line = "".to_string();
        match std::io::stdin().read_line(&mut line) {
            Ok(_) => {
                if line.starts_with('\n') {
                    continue;
                }
                cli.password = Some(line.trim().to_string().clone())
            }
            Err(e) => println!("{:?}", e),
        }
    }
    println!();
    println!();
    println!("Parameters: ");
    println!("Host: {}", cli.host.as_ref().unwrap());
    println!("Port: {}", cli.port.unwrap());
    println!("User: {}", cli.user.as_ref().unwrap());
    println!("Password: {}", cli.password.as_ref().unwrap());
    println!();
    println!();
}

fn show_menu() -> u32 {
    println!();
    println!();
    println!("0 - Exit");
    println!("1 - Get all queues");
    println!("2 - Delete all queues");
    println!("3 - Get all exchanges");
    println!("4 - Delete all exchanges");
    println!("5 - Retrive messages from queue");

    let mut line = "".to_string();
    loop {
        println!("Choose action: ");
        line = String::new();
        match std::io::stdin().read_line(&mut line) {
            Ok(_) => {
                if line.starts_with('\n') {
                    continue;
                }
                match line.trim().to_string().parse::<u32>() {
                    Ok(num) => {
                        return num;
                    }
                    Err(_) => continue,
                }
            }
            Err(e) => println!("{:?}", e),
        }
    }
}
