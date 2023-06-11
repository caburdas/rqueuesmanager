use clap::Parser;

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
}
