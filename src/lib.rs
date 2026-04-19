use zed_extension_api::{
    self as zed, settings::LspSettings, Command, LanguageServerId, Result, Worktree,
};

struct PyAtlasExtension;

impl zed::Extension for PyAtlasExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<Command> {
        let configured = LspSettings::for_worktree(language_server_id.as_ref(), worktree)
            .ok()
            .and_then(|s| s.binary)
            .and_then(|b| b.path);

        let command = match configured {
            Some(path) => path,
            None => worktree
                .which("pyatlas")
                .ok_or_else(|| "`pyatlas` not found on PATH. Install it or set `lsp.pyatlas.binary.path` in settings.".to_string())?,
        };

        Ok(Command {
            command,
            args: vec!["lsp".into()],
            env: vec![],
        })
    }
}

zed::register_extension!(PyAtlasExtension);
