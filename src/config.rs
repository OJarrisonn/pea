use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    shell: String,
    shell_args: Vec<String>,
    pager: Option<String>,
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_path = match dirs::config_dir() {
            Some(dir) => dir.join("pea").join("config.toml"),
            None => bail!("Could not find the configuration directory"),
        };

        if config_path.exists() {
            let config = std::fs::read_to_string(config_path)?;
            Ok(toml::from_str::<Self>(&config)?)
        } else {
            let config = Self::default();
            let content = toml::to_string(&config)?;
            std::fs::create_dir_all(config_path.parent().unwrap())?;
            std::fs::write(config_path, content)?;
            Ok(config)
        }
    }

    pub fn pager(&self) -> String {
        match &self.pager {
            Some(pager) => pager.clone(),
            None => std::env::var("PEA_PAGER")
                .unwrap_or_else(|_| std::env::var("PAGER").unwrap_or_else(|_| "less".to_string())),
        }
    }

    pub fn shell(&self) -> &str {
        &self.shell
    }

    pub fn shell_args<'cmd>(&'cmd self, command: &'cmd str) -> Vec<&'cmd str> {
        self.shell_args
            .iter()
            .map(|arg| {
                if arg.is_empty() {
                    command
                } else {
                    arg.as_str()
                }
            })
            .collect()
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            shell: "sh".to_string(),
            shell_args: vec!["-c".into(), "".into()],
            pager: None,
        }
    }
}
