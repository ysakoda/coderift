mod formatter;
mod walker;

use anyhow::Result;

pub fn run(dir: &str, output: Option<&str>) -> Result<()> {
    let walker = walker::FileWalker::new(dir);
    let formatter = formatter::create_formatter(output)?;
    walker.process_files(formatter)?;
    Ok(())
}
