mod formatter;
mod walker;

use anyhow::Result;
use arboard::Clipboard;

pub fn run(dir: &str, output: Option<&str>, clipboard: bool) -> Result<()> {
    let mut output_content = Vec::new();
    let mut formatter = formatter::create_formatter(Some("-"))?;
    formatter.writer = Box::new(&mut output_content);

    let walker = walker::FileWalker::new(dir);
    walker.process_files(formatter)?;

    let mut formatter = formatter::create_formatter(output)?;
    formatter.write_all(&output_content)?;

    if clipboard {
        let content = String::from_utf8(output_content)
            .map_err(|e| anyhow::anyhow!("Failed to convert content to UTF-8: {}", e))?;

        let mut clipboard =
            Clipboard::new().map_err(|e| anyhow::anyhow!("Failed to create clipboard: {}", e))?;

        clipboard
            .set_text(content)
            .map_err(|e| anyhow::anyhow!("Failed to set clipboard content: {}", e))?;
    }

    Ok(())
}
