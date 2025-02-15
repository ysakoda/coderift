use clap::Parser;
use coderift::run;

#[derive(Parser)]
#[command(author, version, about = "Format source code for AI analysis")]
struct Cli {
    #[arg(default_value = ".")]
    dir: String,

    #[arg(short, long)]
    output: Option<String>,

    #[arg(short, long)]
    clipboard: bool,
}

fn main() -> anyhow::Result<()> {
    env_logger::init();
    let cli = Cli::parse();
    run(&cli.dir, cli.output.as_deref(), cli.clipboard)?;
    Ok(())
}
