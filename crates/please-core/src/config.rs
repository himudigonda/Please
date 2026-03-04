use std::fs;
use std::path::Path;

use anyhow::{Context, Result};

use crate::model::PleaseFile;

pub fn load_pleasefile(workspace_root: &Path) -> Result<PleaseFile> {
    let path = workspace_root.join("pleasefile");
    let content = fs::read_to_string(&path)
        .with_context(|| format!("reading pleasefile from '{}'", path.display()))?;
    parse_pleasefile(&content)
}

pub fn parse_pleasefile(content: &str) -> Result<PleaseFile> {
    toml::from_str(content).context("parsing pleasefile TOML")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_minimal_file() {
        let input = r#"
            [please]
            version = "0.1"

            [task.echo]
            inputs = ["src/main.rs"]
            outputs = ["dist/out.txt"]
            run = "echo hi"
        "#;

        let parsed = parse_pleasefile(input).expect("parse minimal file");
        assert_eq!(parsed.please.version, "0.1");
        assert!(parsed.task.contains_key("echo"));
    }

    #[test]
    fn rejects_unknown_keys() {
        let input = r#"
            [please]
            version = "0.1"

            [task.echo]
            outputs = ["dist/out.txt"]
            run = "echo hi"
            unknown = "nope"
        "#;

        assert!(parse_pleasefile(input).is_err());
    }
}
