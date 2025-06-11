//! Shell completions command

use anyhow::Result;
use clap::CommandFactory;
use clap_complete::{generate, shells::*};
use std::io;

use crate::Cli;

pub async fn execute(shell: String) -> Result<()> {
    let mut cmd = Cli::command();
    let name = cmd.get_name().to_string();
    
    match shell.to_lowercase().as_str() {
        "bash" => {
            generate(Bash, &mut cmd, name, &mut io::stdout());
        }
        "zsh" => {
            generate(Zsh, &mut cmd, name, &mut io::stdout());
        }
        "fish" => {
            generate(Fish, &mut cmd, name, &mut io::stdout());
        }
        "powershell" => {
            generate(PowerShell, &mut cmd, name, &mut io::stdout());
        }
        _ => {
            return Err(anyhow::anyhow!(
                "Unsupported shell: {}. Supported shells: bash, zsh, fish, powershell",
                shell
            ));
        }
    }
    
    Ok(())
} 