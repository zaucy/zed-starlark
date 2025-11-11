use std::fs;
use zed_extension_api::{self as zed, Result};

pub struct Starpls {
    cached_binary_path: Option<String>,
}

impl Starpls {
    pub fn new(binary_path: Option<String>) -> Self {
        Self {
            cached_binary_path: binary_path,
        }
    }

    pub fn language_server_binary_path(
        &mut self,
        language_server_id: &zed::LanguageServerId,
    ) -> Result<String> {
        if let Some(path) = &self.cached_binary_path {
            if fs::metadata(path).map_or(false, |stat| stat.is_file()) {
                return Ok(path.clone());
            }
        }

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );
        let (platform, arch) = zed::current_platform();
        let release = zed::latest_github_release(
            "withered-magic/starpls",
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )?;

        let exe_suffix = match platform {
            zed::Os::Windows => ".exe",
            _ => "",
        };
        let asset_name = format!(
            "starpls-{os}-{arch}{exe_suffix}",
            arch = match (platform, arch) {
                (zed::Os::Mac, zed::Architecture::Aarch64) => "arm64",
                (zed::Os::Linux, zed::Architecture::Aarch64) => "aarch64",
                (_, zed::Architecture::X8664) => "amd64",
                (zed::Os::Windows, zed::Architecture::Aarch64) => {
                    return Err(format!("Unsupported platform/architecture combination: Windows ARM64"));
                }
                (_, zed::Architecture::X86) => {
                    return Err(format!("Unsupported architecture: x86"));
                }
            },
            os = match platform {
                zed::Os::Mac => "darwin",
                zed::Os::Linux => "linux",
                zed::Os::Windows => "windows",
            },
        );

        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == asset_name)
            .ok_or_else(|| format!("no asset found matching {:?}", asset_name))?;

        let version_dir = format!("starpls-{}", release.version);
        let binary_path = format!("{version_dir}/starpls{exe_suffix}");

        if !fs::metadata(&binary_path).map_or(false, |stat| stat.is_file()) {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );

            _ = fs::create_dir(version_dir);

            zed::download_file(
                &asset.download_url,
                &binary_path,
                zed::DownloadedFileType::Uncompressed,
            )
            .map_err(|e| format!("failed to download file: {e}"))?;

            zed::make_file_executable(&binary_path)
                .map_err(|e| format!("failed to make file executable: {e}"))?;
        }

        self.cached_binary_path = Some(binary_path.clone());
        Ok(binary_path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_supported_architectures() {
        let test_cases = vec![
            // Supported combinations
            (zed::Os::Mac, zed::Architecture::Aarch64, Ok("starpls-darwin-arm64")),
            (zed::Os::Mac, zed::Architecture::X8664, Ok("starpls-darwin-amd64")),
            (zed::Os::Linux, zed::Architecture::Aarch64, Ok("starpls-linux-aarch64")),
            (zed::Os::Linux, zed::Architecture::X8664, Ok("starpls-linux-amd64")),
            (zed::Os::Windows, zed::Architecture::X8664, Ok("starpls-windows-amd64.exe")),
            
            // Unsupported combinations
            (zed::Os::Mac, zed::Architecture::X86, Err("Unsupported architecture: x86")),
            (zed::Os::Linux, zed::Architecture::X86, Err("Unsupported architecture: x86")),
            (zed::Os::Windows, zed::Architecture::X86, Err("Unsupported architecture: x86")),
            (zed::Os::Windows, zed::Architecture::Aarch64, Err("Unsupported platform/architecture combination: Windows ARM64"))
        ];

        for (platform, arch, expected) in test_cases {
            let exe_suffix = match platform {
                zed::Os::Windows => ".exe",
                _ => "",
            };

            let result = (|| {
                let asset_name = format!(
                    "starpls-{os}-{arch}{exe_suffix}",
                    arch = match (platform, arch) {
                        (zed::Os::Mac, zed::Architecture::Aarch64) => "arm64",
                        (zed::Os::Linux, zed::Architecture::Aarch64) => "aarch64",
                        (_, zed::Architecture::X8664) => "amd64",
                        (zed::Os::Windows, zed::Architecture::Aarch64) => {
                            return Err(format!("Unsupported platform/architecture combination: Windows ARM64"));
                        }
                        (_, zed::Architecture::X86) => {
                            return Err(format!("Unsupported architecture: x86"));
                        }
                    },
                    os = match platform {
                        zed::Os::Mac => "darwin",
                        zed::Os::Linux => "linux",
                        zed::Os::Windows => "windows",
                    },
                );
                Ok(asset_name)
            })();

            match (result, expected) {
                (Ok(name), Ok(expected_name)) => {
                    assert_eq!(name, expected_name, 
                        "Platform: {:?}, Arch: {:?}", platform, arch);
                }
                (Err(msg), Err(expected_msg)) => {
                    assert_eq!(msg, expected_msg,
                        "Platform: {:?}, Arch: {:?}", platform, arch);
                }
                (Ok(name), Err(expected_msg)) => {
                    panic!("Expected error '{}' but got success: '{}' for Platform: {:?}, Arch: {:?}",
                        expected_msg, name, platform, arch);
                }
                (Err(msg), Ok(expected_name)) => {
                    panic!("Expected success '{}' but got error: '{}' for Platform: {:?}, Arch: {:?}",
                        expected_name, msg, platform, arch);
                }
            }
        }
    }

    #[test]
    fn test_all_release_assets_covered() {
        // Test that all the release assets mentioned by the user are supported
        let release_assets = vec![
            "starpls-darwin-amd64",
            "starpls-darwin-arm64",
            "starpls-linux-aarch64",
            "starpls-linux-amd64",
            "starpls-windows-amd64.exe",
        ];

        let supported_combinations = vec![
            (zed::Os::Mac, zed::Architecture::X8664),
            (zed::Os::Mac, zed::Architecture::Aarch64),
            (zed::Os::Linux, zed::Architecture::Aarch64),
            (zed::Os::Linux, zed::Architecture::X8664),
            (zed::Os::Windows, zed::Architecture::X8664),
        ];

        let mut generated_assets = Vec::new();
        
        for (platform, arch) in supported_combinations {
            let exe_suffix = match platform {
                zed::Os::Windows => ".exe",
                _ => "",
            };

            let asset_name = format!(
                "starpls-{os}-{arch}{exe_suffix}",
                arch = match (platform, arch) {
                    (zed::Os::Mac, zed::Architecture::Aarch64) => "arm64",
                    (zed::Os::Linux, zed::Architecture::Aarch64) => "aarch64",
                    (_, zed::Architecture::X8664) => "amd64",
                    (zed::Os::Windows, zed::Architecture::Aarch64) => panic!("Windows ARM64 should not be in supported combinations"),
                    (_, zed::Architecture::X86) => panic!("X86 should not be in supported combinations"),
                },
                os = match platform {
                    zed::Os::Mac => "darwin",
                    zed::Os::Linux => "linux",
                    zed::Os::Windows => "windows",
                },
            );
            
            generated_assets.push(asset_name);
        }

        // Check that all release assets are covered
        for asset in &release_assets {
            assert!(
                generated_assets.contains(&asset.to_string()),
                "Release asset '{}' is not generated by any supported combination", 
                asset
            );
        }

        // Check that we don't generate any extra assets
        for generated in &generated_assets {
            assert!(
                release_assets.contains(&generated.as_str()),
                "Generated asset '{}' is not in the list of available release assets",
                generated
            );
        }
    }
}
