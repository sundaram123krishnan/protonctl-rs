use core::fmt::Display;
use dirs::home_dir;
use crate::constants;

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Debug)]
pub enum InstallType {
    Proton,
    Wine,
}

impl Display for InstallType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InstallType::Wine => write!(f, "wine"),
            InstallType::Proton => write!(f, "proton"),
        }
    }
}

impl InstallType {
    pub fn get_url(&self, latest: bool) -> String {
        match self {
            InstallType::Wine => {
                if latest {
                    format!(
                        "https://api.github.com/repos/{}/{}/releases/latest",
                        constants::PROJECT_OWNER,
                        constants::WINE_PROJECT_NAME
                    )
                } else {
                    format!(
                        "https://api.github.com/repos/{}/{}/releases",
                        constants::PROJECT_OWNER,
                        constants::WINE_PROJECT_NAME
                    )
                }
            }
            InstallType::Proton => {
                if latest {
                    format!(
                        "https://api.github.com/repos/{}/{}/releases/latest",
                        constants::PROJECT_OWNER,
                        constants::PROTON_PROJECT_NAME
                    )
                } else {
                    format!(
                        "https://api.github.com/repos/{}/{}/releases",
                        constants::PROJECT_OWNER,
                        constants::PROTON_PROJECT_NAME
                    )
                }
            }
        }
    }

    pub fn get_compat_directory_safe(&self) -> anyhow::Result<std::path::PathBuf> {
        let mut compat_dir =
            home_dir().ok_or(anyhow::anyhow!("Failed to get users home directory"))?;

        let compat_path = match self {
            InstallType::Wine => std::path::PathBuf::from(".local/share/lutris/runners/wine"),
            InstallType::Proton => {
                std::path::PathBuf::from(".local/share/Steam/compatibilitytools.d")
            }
        };
        compat_dir.push(compat_path);
        if !compat_dir.exists() {
            std::fs::create_dir_all(&compat_dir)?;
            Ok(compat_dir)
        } else {
            Ok(compat_dir)
        }
    }
}
