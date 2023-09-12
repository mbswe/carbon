mod project;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(
    name = "carbon", 
    version = "0.1", 
    author = "Magnus Strömberg", 
    about = "A time tracking CLI"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Start tracking a new project
    #[command(arg_required_else_help = true)]
    Start {
        /// The name of your project
        title: String,
    },
    
    /// Pause tracking a project
    #[command(arg_required_else_help = true)]
    Pause {
        /// The name of your project
        id: u32,
    },

    /// Resume tracking a project
    #[command(arg_required_else_help = true)]
    Resume {
        /// The name of your project
        id: u32,
    },
    
    /// Stop tracking a project
    #[command(arg_required_else_help = true)]
    Stop {
        /// The name of your project
        id: u32,
    },
    
    /// Show status of the current project
    Status,
    
    /// List all projects
    #[command(arg_required_else_help = true)]
    List {
        #[command(subcommand)]
        command: ListCommands,
    },
}

#[derive(Debug, Subcommand)]
enum ListCommands {
    /// List all projects
    All,
    /// List today's projects
    Today,
    /// List yesterday's projects
    Yesterday,
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Start { title } => {
            project::start(title);
        }
        Commands::Stop { id } => {
            project::stop(id);
        }
        Commands::Pause { id } => {
            project::pause(id);
        }
        Commands::Resume { id } => {
            project::resume(id);
        }
        Commands::Status => {
            project::status();
        }
        Commands::List { command } => match command {
            ListCommands::All => {
                project::list_all();
            }
            ListCommands::Today => {
                project::list_today();
            }
            ListCommands::Yesterday => {
                project::list_yesterday();
            }
        }
    }
}
