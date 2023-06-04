use clap::Parser;
use reqwest::blocking::Client;
use serde::Deserialize;
use serde::Serialize;
use serde_json::{json, Value};

#[derive(Debug, Parser)]
#[command(name = "queuescleaner")]
#[command(version = "1.0")]
#[command(about = "Rabbitmq queues management", long_about = None)]
pub struct Cli {
    /// Hostname
    #[arg(short = 'o', long)]
    pub host: Option<String>,

    /// Port default 15672
    //#[arg(short, long, default_value = "15672")]
    #[arg(short, long)]
    pub port: Option<u16>,

    /// User name
    #[arg(short, long)]
    pub user: Option<String>,

    /// Password
    #[arg(short = 'a', long)]
    pub password: Option<String>,

    // ///Run silently
    #[arg(short, long, default_value = "false")]
    pub run: Option<bool>,
}

pub struct Queue {
    name: String,
}

pub fn get_queues(cli: &Cli) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let url = format!(
        "http://{}:{}/api/queues/",
        cli.host.as_ref().unwrap(),
        cli.port.unwrap()
    );

    let client = Client::new();
    let response = client
        .get(url)
        .basic_auth(cli.user.as_ref().unwrap(), cli.password.clone())
        .send()?;

    println!("Status {}", response.status());
    let kk = response.text()?;

    let mut res = Vec::new();

    let v: Value = serde_json::from_str(&kk)?;
    match v {
        Value::Array(val) => {
            for item in &val {
                println!("{}", item["name"].as_str().unwrap());
                res.push(item["name"].as_str().unwrap().to_string());
            }
        }
        _ => println!("{}", &kk),
    }

    Ok(res)
}

pub fn get_exchanges(cli: &Cli) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let url = format!(
        "http://{}:{}/api/exchanges/",
        cli.host.as_ref().unwrap(),
        cli.port.unwrap()
    );

    let client = Client::new();
    let response = client
        .get(url)
        .basic_auth(cli.user.as_ref().unwrap(), cli.password.clone())
        .send()?;

    println!("Status {}", response.status());
    let kk = response.text()?;

    let mut res = Vec::new();

    let v: Value = serde_json::from_str(&kk)?;
    match v {
        Value::Array(val) => {
            for item in &val {
                println!("{}", item["name"].as_str().unwrap());
                //res.push(item["name"].to_string());
                res.push(item["name"].as_str().unwrap().to_string());
            }
        }
        _ => println!("{}", &kk),
    }

    Ok(res)
}

pub fn get_messages(cli: &Cli) -> Result<(Vec<String>), Box<dyn std::error::Error>> {
    todo!();
}

pub fn delete_queues(cli: &Cli) -> Result<(), Box<dyn std::error::Error>> {
    let v = get_queues(cli)?;

    for q in v {
        let url = format!(
            "http://{}:{}/api/queues/%2F/{}",
            cli.host.as_ref().unwrap(),
            cli.port.unwrap(),
            q
        );

        let client = Client::new();
        let response = client
            .delete(url)
            .basic_auth(cli.user.as_ref().unwrap(), cli.password.clone())
            .send()?;
        //println!("Status {}", response.status());
    }

    Ok(())
}

pub fn delete_exchanges(cli: &Cli) -> Result<(), Box<dyn std::error::Error>> {
    let v = get_exchanges(cli)?;

    for q in v {
        if !q.contains("amq.") {
            let url = format!(
                "http://{}:{}/api/exchanges/%2F/{}",
                cli.host.as_ref().unwrap(),
                cli.port.unwrap(),
                q
            );

            let client = Client::new();
            let response = client
                .delete(url)
                .basic_auth(cli.user.as_ref().unwrap(), cli.password.clone())
                .send()?;
            println!("Status {}", response.status());
        }
    }

    Ok(())
}
