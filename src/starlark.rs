use std::fs;
use zed_extension_api::{self as zed, Result};

struct StarlarkExtension {
    cached_binary_path: Option<String>,
}

impl StarlarkExtension {
    fn language_server_binary_path(&mut self, config: zed::LanguageServerConfig) -> Result<String> {
        Ok(String::from("C:/Users/zekew/.cargo/bin/starpls.exe"))
    }
}

impl zed::Extension for StarlarkExtension {
    fn new() -> Self {
        Self {
            cached_binary_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        config: zed::LanguageServerConfig,
        _worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        Ok(zed::Command {
            command: self.language_server_binary_path(config)?,
            args: vec![],
            env: Default::default(),
        })
    }
}

zed::register_extension!(StarlarkExtension);
