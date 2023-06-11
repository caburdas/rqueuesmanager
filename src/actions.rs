use crate::cli::Cli;
use reqwest::blocking::Client;
use serde_json::Value;

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
    let ret = response.text()?;

    let mut res = Vec::new();

    let v: Value = serde_json::from_str(&ret)?;
    match v {
        Value::Array(val) => {
            for item in &val {
                println!("{}", item["name"].as_str().unwrap());
                res.push(item["name"].as_str().unwrap().to_string());
            }
        }
        _ => println!("{}", &ret),
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
    let ret = response.text()?;

    let mut res = Vec::new();

    let v: Value = serde_json::from_str(&ret)?;
    match v {
        Value::Array(val) => {
            for item in &val {
                println!("{}", item["name"].as_str().unwrap());
                //res.push(item["name"].to_string());
                res.push(item["name"].as_str().unwrap().to_string());
            }
        }
        _ => println!("{}", &ret),
    }

    Ok(res)
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
        let _ = client
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
            let _ = client
                .delete(url)
                .basic_auth(cli.user.as_ref().unwrap(), cli.password.clone())
                .send()?;
            //println!("Status {}", response.status());
        }
    }

    Ok(())
}

pub fn get_messages(cli: &Cli) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut line = String::new();
    println!("Name of the queue :");
    std::io::stdin().read_line(&mut line)?;

    let mut q = String::new();
    if line.starts_with('\n') {
        return Result::Err("Error! queue name not valid ".into());
    } else {
        q = line;
    }

    let mut number = String::new();
    println!("number of messages to retrieve :");
    line = "".to_string();
    std::io::stdin().read_line(&mut line)?;
    line.trim().to_string().parse::<u16>()?; //check if valid number
    line.pop(); //reomve trail \n
    number = line;

    line = "".to_string();
    let mut mode = String::new();
    println!("delete messages (y/n)?");
    std::io::stdin().read_line(&mut line)?;
    line.pop(); //remove trail \n
    match line.as_str() {
        "y" | "Y" => mode = "ack_requeue_false".to_string(),
        _ => mode = "ack_requeue_true".to_string(),
    }

    let url = format!(
        "http://{}:{}/api/queues/%2F/{}/get",
        cli.host.as_ref().unwrap(),
        cli.port.unwrap(),
        q
    );

    let body = format!(
        "{{ \"count\":{},\"ackmode\":\"{}\",\"encoding\":\"auto\"}}",
        number, mode
    );
    let client = Client::new();
    let response = client
        .post(url)
        .basic_auth(cli.user.as_ref().unwrap(), cli.password.clone())
        .body(body)
        .send()?;

    println!("Status {}", response.status());
    let ret = response.text()?;

    let mut res = Vec::new();

    let v: Value = serde_json::from_str(&ret)?;
    match v {
        Value::Array(val) => {
            for item in &val {
                if item.as_object().is_some() {
                    println!("{}", serde_json::to_string_pretty(&item).unwrap());
                    res.push(serde_json::to_string_pretty(&item).unwrap());
                }
            }
        }
        _ => println!("{}", &ret),
    }

    Ok(res)
}
