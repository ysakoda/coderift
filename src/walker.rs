use crate::formatter;

use ignore::WalkBuilder;
use std::path::Path;

const EXCLUDE_DIRS: &[&str] = &["node_modules", "target", ".git", "dist", ".next"];

const INCLUDE_EXTENSIONS: &[&str] = &[
    "rs", "ts", "tsx", "astro", "sql", "yml", "yaml", "tf", "json", "toml", "md",
];

pub struct FileWalker {
    root: String,
}

impl FileWalker {
    pub fn new(root: &str) -> Self {
        Self {
            root: root.to_string(),
        }
    }

    pub fn process_files<W: std::io::Write>(
        &self,
        mut formatter: formatter::CodeFormatter<W>,
    ) -> anyhow::Result<()> {
        let walker = WalkBuilder::new(&self.root).standard_filters(true).build();

        for entry in walker.filter_map(Result::ok) {
            if let Some(path) = entry.path().to_str() {
                if self.should_process(path) {
                    formatter.format_file(path)?;
                }
            }
        }
        Ok(())
    }

    fn should_process(&self, path: &str) -> bool {
        let path = Path::new(path);

        if EXCLUDE_DIRS
            .iter()
            .any(|dir| path.components().any(|c| c.as_os_str() == *dir))
        {
            return false;
        }

        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            return INCLUDE_EXTENSIONS.contains(&ext)
                || path.file_name().map_or(false, |f| {
                    f.to_str()
                        .map_or(false, |s| s.starts_with("Dockerfile") || s == "Makefile")
                });
        }
        false
    }
}
