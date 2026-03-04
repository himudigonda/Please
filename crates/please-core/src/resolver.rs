use std::collections::BTreeSet;
use std::path::{Component, Path, PathBuf};

use anyhow::{anyhow, Context, Result};

pub fn resolve_inputs(workspace_root: &Path, patterns: &[String]) -> Result<Vec<PathBuf>> {
    let mut resolved = BTreeSet::new();

    for pattern in patterns {
        validate_pattern(pattern)?;

        if looks_like_glob(pattern) {
            let absolute_pattern = workspace_root.join(pattern);
            let pattern_text = absolute_pattern
                .to_str()
                .ok_or_else(|| anyhow!("non UTF-8 input pattern '{}'", pattern))?;

            let mut found = false;
            for entry in glob::glob(pattern_text)
                .with_context(|| format!("resolving input glob '{}'", pattern))?
            {
                let path = entry.with_context(|| format!("expanding input glob '{}'", pattern))?;
                if !path.exists() {
                    continue;
                }

                let rel = path
                    .strip_prefix(workspace_root)
                    .with_context(|| {
                        format!(
                            "input path '{}' escaped workspace root '{}'",
                            path.display(),
                            workspace_root.display()
                        )
                    })?
                    .to_path_buf();
                resolved.insert(rel);
                found = true;
            }

            if !found {
                resolved.insert(PathBuf::from(pattern));
            }
        } else {
            let normalized = normalize_relative_path(pattern)?;
            resolved.insert(normalized);
        }
    }

    Ok(resolved.into_iter().collect())
}

pub fn normalize_relative_path(raw: &str) -> Result<PathBuf> {
    if raw.trim().is_empty() {
        return Err(anyhow!("path cannot be empty"));
    }

    let path = Path::new(raw);
    if path.is_absolute() {
        return Err(anyhow!("path '{}' must be relative", raw));
    }

    let mut normalized = PathBuf::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::Normal(part) => normalized.push(part),
            Component::ParentDir => {
                return Err(anyhow!("path '{}' cannot contain '..' segments in pleasefile", raw))
            }
            Component::RootDir | Component::Prefix(_) => {
                return Err(anyhow!("path '{}' must be relative", raw))
            }
        }
    }

    if normalized.as_os_str().is_empty() {
        return Err(anyhow!("path '{}' cannot resolve to current directory", raw));
    }

    Ok(normalized)
}

fn validate_pattern(pattern: &str) -> Result<()> {
    if pattern.trim().is_empty() {
        return Err(anyhow!("input pattern cannot be empty"));
    }
    if pattern.starts_with('/') {
        return Err(anyhow!("input pattern '{}' must be relative to workspace", pattern));
    }
    if pattern.split('/').any(|segment| segment == "..") {
        return Err(anyhow!("input pattern '{}' cannot contain '..' segments", pattern));
    }
    Ok(())
}

fn looks_like_glob(pattern: &str) -> bool {
    pattern.contains('*') || pattern.contains('?') || pattern.contains('[') || pattern.contains('{')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_valid_relative_paths() {
        let normalized = normalize_relative_path("./src//main.rs").expect("normalize path");
        assert_eq!(normalized, PathBuf::from("src/main.rs"));
    }

    #[test]
    fn rejects_parent_segments() {
        assert!(normalize_relative_path("../secret").is_err());
    }
}
