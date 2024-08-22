use std::error::Error;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub shell: String,
    pub shell_args: Vec<String>,
    pub pager: Option<String>,
}

impl Config {
    pub fn load() -> Result<Self, Box<dyn Error>> {
        let config_path = dirs::config_dir()
            .ok_or(String::from("Failed to get config directory"))?
            .join("pea")
            .join("config.toml");

        if config_path.exists() {
            let config = std::fs::read_to_string(config_path)?;
            Ok(toml::from_str::<Self>(&config).map_err(Box::new)?)
        } else {
            let config = Self::default();
            let content = toml::to_string(&config)?;
            std::fs::create_dir_all(config_path.parent().unwrap())?;
            std::fs::write(config_path, content)?;
            Ok(config)
        }
    }

    pub fn with_pager(self) -> Self {
        match &self.pager {
            Some(_) => self,
            None => Self {
                pager: std::env::var("PEA_PAGER")
                    .unwrap_or_else(|_| {
                        std::env::var("PAGER").unwrap_or_else(|_| "less".to_string())
                    })
                    .into(),
                ..self
            },
        }
    }

    pub fn format_shell_args(&mut self, command: &str) {
        for arg in &mut self.shell_args {
            if arg.is_empty() {
                *arg = command.into();
            }    
        }
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
