use clap::Parser; // CLI parser
use dat_cli::Commands;
use log::debug;

mod logging;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Turn debugging information on
    #[arg(short, long)]
    debug: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

impl ToString for Cli {
    fn to_string(&self) -> String {
        let mut output = String::new();
        if let Some(cmd) = &self.command {
            output.push_str(&format!("{:?}", cmd.to_string()));
        }
        format!("{:?}", output)
    }
}

fn main() {
    let args = Cli::parse();
    logging::init_logging(args.debug);
    debug!("Args passed successfully: {:?}", args.to_string());
    if let Some(cmd) = args.command {
        dat_cli::run_command(cmd).unwrap();
    }
}
