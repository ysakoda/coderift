use clap::Parser;
use coderift::run;

#[derive(Parser)]
#[command(author, version, about = "Format source code for AI analysis")]
struct Cli {
    /// Directory to scan (default: current directory)
    #[arg(default_value = ".")]
    dir: String,

    /// Output file (default: stdout)
    #[arg(short, long)]
    output: Option<String>,
}

fn main() -> anyhow::Result<()> {
    env_logger::init();
    let cli = Cli::parse();
    run(&cli.dir, cli.output.as_deref())?;
    Ok(())
}
