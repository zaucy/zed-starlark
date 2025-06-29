use starpls::Starpls;
use zed_extension_api::{self as zed, settings::LspSettings, Result};

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
                let binary_settings = LspSettings::for_worktree("starpls", worktree)
                    .ok()
                    .and_then(|lsp_settings| lsp_settings.binary);
                let binary_args = binary_settings
                    .as_ref()
                    .and_then(|binary_settings| binary_settings.arguments.clone())
                    .unwrap_or_else(|| vec![]);
                let starpls = self.starpls.get_or_insert_with(|| Starpls::new());
                Ok(zed::Command {
                    command: starpls.language_server_binary_path(language_server_id)?,
                    args: binary_args,
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
            "tilt" => {
                let path = worktree.which("tilt").ok_or_else(|| {
                    "`tilt` must be installed. The LSP is bundled with the tilt cli.".to_string()
                })?;

                Ok(zed::Command {
                    command: path,
                    args: vec!["lsp".to_string(), "start".to_string()],
                    env: Default::default(),
                })
            }
            language_server_id => Err(format!("unknown language server: {language_server_id}")),
        }
    }

    fn language_server_initialization_options(
        &mut self,
        server_id: &zed::LanguageServerId,
        worktree: &zed_extension_api::Worktree,
    ) -> Result<Option<zed_extension_api::serde_json::Value>> {
        let settings = LspSettings::for_worktree(server_id.as_ref(), worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.initialization_options.clone())
            .unwrap_or_default();
        Ok(Some(settings))
    }

    fn language_server_workspace_configuration(
        &mut self,
        server_id: &zed::LanguageServerId,
        worktree: &zed_extension_api::Worktree,
    ) -> Result<Option<zed_extension_api::serde_json::Value>> {
        let settings = LspSettings::for_worktree(server_id.as_ref(), worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.settings.clone())
            .unwrap_or_default();
        Ok(Some(settings))
    }
}

zed::register_extension!(StarlarkExtension);
