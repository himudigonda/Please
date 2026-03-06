use anyhow::{anyhow, Context, Result};
use std::fs;
use std::path::Path;
use std::sync::Once;

use crate::model::BroskiFile;
use crate::parser_winnow::parse_broskifile_dsl_with_workspace;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParserMode {
    Auto,
    Toml,
    Dsl,
}

impl ParserMode {
    pub fn from_env() -> Self {
        let parser_mode = std::env::var("BROSKI_PARSER_MODE").ok();
        match parser_mode {
            Some(value) if value.eq_ignore_ascii_case("toml") => ParserMode::Toml,
            Some(value) if value.eq_ignore_ascii_case("dsl") => ParserMode::Dsl,
            _ => ParserMode::Auto,
        }
    }
}

pub fn load_broskifile(workspace_root: &Path) -> Result<BroskiFile> {
    let broskifile_path = workspace_root.join("broskifile");
    if !broskifile_path.exists() {
        return Err(anyhow!("no broskifile found in '{}'", workspace_root.display()));
    }
    let content = fs::read_to_string(&broskifile_path)
        .with_context(|| format!("reading broskifile from '{}'", broskifile_path.display()))?;
    parse_broskifile_with_mode_at(&content, ParserMode::from_env(), Some(workspace_root))
        .with_context(|| format!("parsing configuration from '{}'", broskifile_path.display()))
}

pub fn parse_broskifile(content: &str) -> Result<BroskiFile> {
    parse_broskifile_with_mode(content, ParserMode::from_env())
}

pub fn parse_broskifile_with_mode(content: &str, mode: ParserMode) -> Result<BroskiFile> {
    parse_broskifile_with_mode_at(content, mode, None)
}

fn parse_broskifile_with_mode_at(
    content: &str,
    mode: ParserMode,
    workspace_root: Option<&Path>,
) -> Result<BroskiFile> {
    match mode {
        ParserMode::Toml => parse_toml(content),
        ParserMode::Dsl => {
            let parsed = parse_broskifile_dsl_with_workspace(content, workspace_root)?;
            warn_dsl_deprecated_if_needed(&parsed);
            Ok(parsed)
        }
        ParserMode::Auto => {
            if looks_like_toml(content) {
                warn_toml_deprecated();
                parse_toml(content)
            } else {
                let parsed = parse_broskifile_dsl_with_workspace(content, workspace_root)?;
                warn_dsl_deprecated_if_needed(&parsed);
                Ok(parsed)
            }
        }
    }
}

fn parse_toml(content: &str) -> Result<BroskiFile> {
    toml::from_str(content).context("parsing broskifile TOML")
}

fn looks_like_toml(content: &str) -> bool {
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        return trimmed == "[broski]"
            || trimmed.starts_with("[task.")
            || trimmed.starts_with("[alias")
            || trimmed.starts_with("[load");
    }
    false
}

fn warn_toml_deprecated() {
    static WARN_ONCE: Once = Once::new();
    WARN_ONCE.call_once(|| {
        eprintln!("warning: TOML broskifile is deprecated; migrate to DSL before v0.6");
    });
}

fn warn_dsl_deprecated_if_needed(parsed: &BroskiFile) {
    static WARN_03: Once = Once::new();
    static WARN_04: Once = Once::new();

    if parsed.broski.version == "0.3" {
        WARN_03.call_once(|| {
            eprintln!(
                "warning: broskifile DSL version \"0.3\" is deprecated; migrate to version = \"0.5\" before v0.6"
            );
        });
    } else if parsed.broski.version == "0.4" {
        WARN_04.call_once(|| {
            eprintln!(
                "warning: broskifile DSL version \"0.4\" is deprecated; migrate to version = \"0.5\" before v0.6"
            );
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_minimal_dsl_file() {
        let input = r#"
            version = "0.3"

            echo:
                @out dist/out.txt
                echo hi > dist/out.txt
        "#;

        let parsed =
            parse_broskifile_with_mode(input, ParserMode::Dsl).expect("parse minimal file");
        assert_eq!(parsed.broski.version, "0.3");
        assert!(parsed.task.contains_key("echo"));
    }

    #[test]
    fn parses_toml_file_with_explicit_mode() {
        let input = r#"
            [broski]
            version = "0.2"

            [task.echo]
            inputs = ["src/main.rs"]
            outputs = ["dist/out.txt"]
            run = "echo hi"
        "#;

        let parsed = parse_broskifile_with_mode(input, ParserMode::Toml).expect("parse toml");
        assert_eq!(parsed.broski.version, "0.2");
        assert!(parsed.task.contains_key("echo"));
    }

    #[test]
    fn autodetect_prefers_dsl_when_no_toml_sections() {
        let input = r#"
            version = "0.3"

            hello:
                echo hi
        "#;

        let parsed = parse_broskifile_with_mode(input, ParserMode::Auto).expect("parse dsl auto");
        assert_eq!(parsed.broski.version, "0.3");
        assert!(parsed.task.contains_key("hello"));
    }

    #[test]
    fn autodetect_supports_toml_fallback() {
        let input = r#"
            [broski]
            version = "0.2"

            [task.hello]
            inputs = ["src/main.rs"]
            outputs = ["dist/out.txt"]
            run = "echo hi"
        "#;

        let parsed = parse_broskifile_with_mode(input, ParserMode::Auto).expect("parse toml auto");
        assert_eq!(parsed.broski.version, "0.2");
        assert!(parsed.task.contains_key("hello"));
    }
}
