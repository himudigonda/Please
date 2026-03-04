use std::collections::BTreeMap;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use walkdir::WalkDir;

use crate::model::TaskSpec;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskFingerprint(pub String);

pub fn compute_fingerprint(
    workspace_root: &Path,
    task_name: &str,
    task: &TaskSpec,
    resolved_inputs: &[PathBuf],
) -> Result<TaskFingerprint> {
    let mut hasher = blake3::Hasher::new();

    hasher.update(b"please-v0.1");
    hasher.update(task_name.as_bytes());
    hasher.update(task.run_as_shell().as_bytes());
    hasher.update(format!("isolation={:?}", task.effective_isolation()).as_bytes());

    for pattern in &task.inputs {
        hasher.update(b"pattern:");
        hasher.update(pattern.as_bytes());
    }

    let sorted_env: BTreeMap<_, _> = task.env.iter().collect();
    for (key, value) in sorted_env {
        hasher.update(b"env:");
        hasher.update(key.as_bytes());
        hasher.update(b"=");
        hasher.update(value.as_bytes());
    }

    for input in resolved_inputs {
        hasher.update(b"input:");
        hasher.update(input.to_string_lossy().as_bytes());

        let absolute = workspace_root.join(input);
        if absolute.exists() {
            hash_path(&absolute, &mut hasher)
                .with_context(|| format!("hashing input '{}'", absolute.display()))?;
        } else {
            hasher.update(b"missing");
        }
    }

    for output in &task.outputs {
        hasher.update(b"output:");
        hasher.update(output.as_bytes());
    }

    Ok(TaskFingerprint(hasher.finalize().to_hex().to_string()))
}

fn hash_path(path: &Path, hasher: &mut blake3::Hasher) -> Result<()> {
    if path.is_file() {
        hasher.update(b"file");
        let mut file = fs::File::open(path)
            .with_context(|| format!("opening file '{}' for hashing", path.display()))?;
        let mut buffer = [0u8; 16 * 1024];
        loop {
            let count = file
                .read(&mut buffer)
                .with_context(|| format!("reading file '{}' for hashing", path.display()))?;
            if count == 0 {
                break;
            }
            hasher.update(&buffer[..count]);
        }
        return Ok(());
    }

    if path.is_dir() {
        hasher.update(b"dir");

        let mut children = Vec::new();
        for entry in WalkDir::new(path) {
            let entry = entry.context("walking input directory while hashing")?;
            if entry.path().is_dir() {
                continue;
            }
            let rel = entry
                .path()
                .strip_prefix(path)
                .with_context(|| format!("stripping input prefix '{}'", path.display()))?
                .to_path_buf();
            children.push((rel, entry.path().to_path_buf()));
        }
        children.sort_by(|a, b| a.0.cmp(&b.0));

        for (rel, child) in children {
            hasher.update(rel.to_string_lossy().as_bytes());
            let mut file = fs::File::open(&child)
                .with_context(|| format!("opening file '{}' for hashing", child.display()))?;
            let mut buffer = [0u8; 16 * 1024];
            loop {
                let count = file
                    .read(&mut buffer)
                    .with_context(|| format!("reading file '{}' for hashing", child.display()))?;
                if count == 0 {
                    break;
                }
                hasher.update(&buffer[..count]);
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{RunSpec, TaskSpec};
    use std::collections::BTreeMap;
    use std::io::Write;

    #[test]
    fn fingerprint_changes_when_env_changes() {
        let temp = tempfile::tempdir().expect("tempdir");
        let src = temp.path().join("src");
        std::fs::create_dir_all(&src).expect("create src");
        let mut f = std::fs::File::create(src.join("main.rs")).expect("create main.rs");
        f.write_all(b"fn main() {} ").expect("write main.rs");

        let mut env_a = BTreeMap::new();
        env_a.insert("MODE".to_string(), "a".to_string());

        let mut env_b = BTreeMap::new();
        env_b.insert("MODE".to_string(), "b".to_string());

        let task_a = TaskSpec {
            deps: vec![],
            inputs: vec!["src/main.rs".to_string()],
            outputs: vec!["dist/out".to_string()],
            env: env_a,
            run: RunSpec::Shell("echo hi".to_string()),
            isolation: None,
        };

        let task_b = TaskSpec { env: env_b, ..task_a.clone() };

        let resolved = vec![PathBuf::from("src/main.rs")];

        let fp_a =
            compute_fingerprint(temp.path(), "build", &task_a, &resolved).expect("fingerprint a");
        let fp_b =
            compute_fingerprint(temp.path(), "build", &task_b, &resolved).expect("fingerprint b");

        assert_ne!(fp_a, fp_b);
    }
}
