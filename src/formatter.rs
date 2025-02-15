use std::fs::File;
use std::io::Read;
use std::io::{self, BufReader, Write};
use std::path::Path;

pub struct CodeFormatter<W: Write> {
    pub(crate) writer: W,
}

pub fn create_formatter(output: Option<&str>) -> io::Result<CodeFormatter<Box<dyn Write>>> {
    let writer: Box<dyn Write> = match output {
        Some("-") => Box::new(Vec::new()),
        Some(path) => Box::new(File::create(path)?),
        None => Box::new(io::stdout()),
    };
    Ok(CodeFormatter { writer })
}

impl<W: Write> CodeFormatter<W> {
    pub fn format_file(&mut self, path: &str) -> io::Result<()> {
        let path = Path::new(path);
        let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("txt");

        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        let mut content = String::new();
        reader.read_to_string(&mut content)?;

        writeln!(self.writer, "```{} {}", ext, path.display())?;
        writeln!(self.writer, "{}", content)?;
        writeln!(self.writer, "```\n")?;
        Ok(())
    }

    pub fn write_all(&mut self, content: &[u8]) -> io::Result<()> {
        self.writer.write_all(content)
    }
}
