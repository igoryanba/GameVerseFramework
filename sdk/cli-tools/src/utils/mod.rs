//! Utility functions for GameVerse CLI

use anyhow::{Result, Context};
use std::path::{Path, PathBuf};
use std::process::Stdio;
use tokio::process::Command;
use walkdir::WalkDir;

pub struct FileUtils;
pub struct ProcessUtils;

impl FileUtils {
    /// Find files matching patterns in directory
    pub async fn find_files(dir: &Path, patterns: &[&str]) -> Result<Vec<PathBuf>> {
        let mut files = Vec::new();
        
        for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().is_file() {
                let path = entry.path();
                
                for pattern in patterns {
                    if Self::matches_pattern(path, pattern) {
                        files.push(path.to_path_buf());
                        break;
                    }
                }
            }
        }
        
        Ok(files)
    }
    
    /// Check if directory has files with given extension
    pub fn has_files_with_extension(dir: &Path, extension: &str) -> bool {
        WalkDir::new(dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .any(|entry| {
                entry.file_type().is_file() &&
                entry.path().extension().and_then(|ext| ext.to_str()) == Some(extension)
            })
    }
    
    /// Copy directory recursively
    pub async fn copy_dir(src: &Path, dst: &Path) -> Result<()> {
        tokio::fs::create_dir_all(dst).await?;
        
        for entry in WalkDir::new(src).into_iter().filter_map(|e| e.ok()) {
            let src_path = entry.path();
            let relative_path = src_path.strip_prefix(src)?;
            let dst_path = dst.join(relative_path);
            
            if entry.file_type().is_dir() {
                tokio::fs::create_dir_all(&dst_path).await?;
            } else {
                if let Some(parent) = dst_path.parent() {
                    tokio::fs::create_dir_all(parent).await?;
                }
                tokio::fs::copy(src_path, &dst_path).await?;
            }
        }
        
        Ok(())
    }
    
    /// Remove directory recursively
    pub async fn remove_dir(dir: &Path) -> Result<()> {
        if dir.exists() {
            tokio::fs::remove_dir_all(dir).await
                .with_context(|| format!("Failed to remove directory: {}", dir.display()))?;
        }
        Ok(())
    }
    
    /// Get file size in bytes
    pub async fn file_size(path: &Path) -> Result<u64> {
        let metadata = tokio::fs::metadata(path).await?;
        Ok(metadata.len())
    }
    
    /// Check if path exists
    pub async fn exists(path: &Path) -> bool {
        tokio::fs::metadata(path).await.is_ok()
    }
    
    /// Create file with content
    pub async fn create_file(path: &Path, content: &str) -> Result<()> {
        if let Some(parent) = path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }
        
        tokio::fs::write(path, content).await
            .with_context(|| format!("Failed to create file: {}", path.display()))?;
        
        Ok(())
    }
    
    fn matches_pattern(path: &Path, pattern: &str) -> bool {
        if pattern.starts_with("*.") {
            let extension = &pattern[2..];
            path.extension().and_then(|ext| ext.to_str()) == Some(extension)
        } else {
            path.file_name().and_then(|name| name.to_str()) == Some(pattern)
        }
    }
}

impl ProcessUtils {
    /// Run command and wait for completion
    pub async fn run_command(
        program: &str,
        args: &[&str],
        working_dir: Option<&Path>,
    ) -> Result<()> {
        let mut command = Command::new(program);
        command.args(args);
        
        if let Some(dir) = working_dir {
            command.current_dir(dir);
        }
        
        let status = command.status().await
            .with_context(|| format!("Failed to execute command: {}", program))?;
        
        if !status.success() {
            return Err(anyhow::anyhow!(
                "Command failed: {} {}",
                program,
                args.join(" ")
            ));
        }
        
        Ok(())
    }
    
    /// Run command and capture output
    pub async fn run_command_output(
        program: &str,
        args: &[&str],
        working_dir: Option<&Path>,
    ) -> Result<String> {
        let mut command = Command::new(program);
        command.args(args);
        command.stdout(Stdio::piped());
        command.stderr(Stdio::piped());
        
        if let Some(dir) = working_dir {
            command.current_dir(dir);
        }
        
        let output = command.output().await
            .with_context(|| format!("Failed to execute command: {}", program))?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow::anyhow!(
                "Command failed: {} {}\nError: {}",
                program,
                args.join(" "),
                stderr
            ));
        }
        
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
    
    /// Check if command exists in PATH
    pub async fn command_exists(program: &str) -> bool {
        #[cfg(target_os = "windows")]
        let which_cmd = "where";
        #[cfg(not(target_os = "windows"))]
        let which_cmd = "which";
        
        Command::new(which_cmd)
            .arg(program)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .await
            .map(|status| status.success())
            .unwrap_or(false)
    }
    
    /// Run command with real-time output
    pub async fn run_command_interactive(
        program: &str,
        args: &[&str],
        working_dir: Option<&Path>,
    ) -> Result<()> {
        let mut command = Command::new(program);
        command.args(args);
        
        if let Some(dir) = working_dir {
            command.current_dir(dir);
        }
        
        // Inherit stdio for interactive output
        command.stdin(Stdio::inherit());
        command.stdout(Stdio::inherit());
        command.stderr(Stdio::inherit());
        
        let status = command.status().await
            .with_context(|| format!("Failed to execute command: {}", program))?;
        
        if !status.success() {
            return Err(anyhow::anyhow!(
                "Command failed: {} {}",
                program,
                args.join(" ")
            ));
        }
        
        Ok(())
    }
    
    /// Get environment variable
    pub fn get_env(key: &str) -> Option<String> {
        std::env::var(key).ok()
    }
    
    /// Set environment variable for child processes
    pub fn set_env(key: &str, value: &str) {
        std::env::set_var(key, value);
    }
    
    /// Get current working directory
    pub fn current_dir() -> Result<PathBuf> {
        std::env::current_dir()
            .context("Failed to get current directory")
    }
    
    /// Change current working directory
    pub fn change_dir(dir: &Path) -> Result<()> {
        std::env::set_current_dir(dir)
            .with_context(|| format!("Failed to change directory to: {}", dir.display()))
    }
} 