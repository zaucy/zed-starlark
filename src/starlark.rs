use starpls::Starpls;
use zed_extension_api::{self as zed, Result};

mod starpls;

struct StarlarkExtension {
    starpls: Option<Starpls>,
}

impl zed::Extension for StarlarkExtension {
    fn new() -> Self {
        Self { starpls: None }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        match language_server_id.as_ref() {
            "starpls" => {
                let starpls = self.starpls.get_or_insert_with(|| Starpls::new());
                Ok(zed::Command {
                    command: starpls.language_server_binary_path(language_server_id)?,
                    args: vec![],
                    env: Default::default(),
                })
            }
            "buck2-lsp" => {
                let path = worktree.which("buck2").ok_or_else(|| {
                    "buck2 must be installed. The LSP is bundled with the buck2 cli.".to_string()
                })?;
                Ok(zed::Command {
                    command: path,
                    args: vec!["lsp".to_string()],
                    env: Default::default(),
                })
            }
            language_server_id => Err(format!("unknown language server: {language_server_id}")),
        }
    }
}

zed::register_extension!(StarlarkExtension);
