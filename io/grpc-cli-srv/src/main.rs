use structopt::StructOpt;

// These are the options used by the `server` subcommand
#[derive(Debug, StructOpt)]
pub struct ServerOptions {
    /// The address of the server that will run commands.
    #[structopt(long, default_value = "127.0.0.1:50051")]
    pub server_listen_addr: String,
}

// These are the options used by the `run` subcommand
#[derive(Debug, StructOpt)]
pub struct RemoteCommandOptions {
    /// The address of the server that will run commands.
    #[structopt(long = "server", default_value = "http://127.0.0.1:50051")]
    pub server_addr: String,

    /// The full command and arguments for the server to execute
    pub command: Vec<String>,
}

// These are the only valid values for our subcommands
#[derive(Debug, StructOpt)]
pub enum SubCommand {
    /// Start the remote command gRPC server
    #[structopt(name = "server")]
    StartServer(ServerOptions),

    /// Send a remote command to the gRPC server
    #[structopt(setting = structopt::clap::AppSettings::TrailingVarArg)]
    Run(RemoteCommandOptions),
}

// This is the main arguments structure that we'll parse from
#[derive(StructOpt, Debug)]
#[structopt(name = "gcs")]
struct ApplicationArguments {
    #[structopt(flatten)]
    pub subcommand: SubCommand,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = ApplicationArguments::from_args();

    match args.subcommand {
        SubCommand::StartServer(opts) => println!("starting the server: {:?}", opts),
        SubCommand::Run(opts) => println!("sending a command: {:?}", opts),
    }

    Ok(())
}
