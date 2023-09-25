use clap::{Args, Parser, Subcommand};
use ifunny_basic::{BasicToken, BasicTokenLength};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, arg_required_else_help = true)]
/// Command line tool for managing iFunny Basic Tokens and Bearer Tokens
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Parser, Debug)]
pub enum Commands {
    #[command(subcommand)]
    Basic(BasicCommand),
    // Login,
    // Register,
    // Logout,
}

#[derive(Subcommand, Debug)]
#[command(author, version, about, long_about = None, arg_required_else_help = true)]
/// Commands for managing basic tokens
pub enum BasicCommand {
    /// Create a new basic token
    New {
        /// Length of the token (112 or 156)
        length: usize,
        #[arg(short, long, default_value = BasicToken::CLIENT_ID)]
        /// Client ID to use for the token
        id: String,
        #[arg(short, long, default_value = BasicToken::CLIENT_SECRET)]
        /// Client Secret to use for the token
        secret: String,
    },
}

impl Runnable for Cli {
    fn run(&self) {
        match &self.command {
            Commands::Basic(cmd) => cmd.run(),
            // Commands::Login => println!("Login"),
            // Commands::Register => println!("Register"),
            // Commands::Logout => println!("Logout"),
        }
    }
}

impl Runnable for BasicCommand {
    fn run(&self) {
        match self {
            BasicCommand::New { length, id, secret } => {
                let length = match length {
                    112 => BasicTokenLength::Basic112,
                    156 => BasicTokenLength::Basic156,
                    _ => BasicTokenLength::Basic112,
                };
                let basic = BasicToken::new(id, secret, length);

                println!("Basic token: {}", basic);
            }
        }
    }
}

pub trait Runnable {
    fn run(&self);
}
