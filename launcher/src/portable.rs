//! One distributed executable; native programs are materialized into a versioned user cache.
#[cfg(feature = "portable")]
const PAYLOAD: &[u8] = include_bytes!(env!("HORIZON_PORTABLE_PAYLOAD"));

pub fn install() -> Result<(), String> {
    #[cfg(feature = "portable")]
    {
        use sha2::{Digest, Sha256};
        use std::{fs, io::Cursor};
        let hash = format!("{:x}", Sha256::digest(PAYLOAD));
        let tag = format!("portable-{}", &hash[..16]);
        let dir = crate::backend::version_dir(&tag);
        if !dir.join(".installed").is_file() {
            let destination = dir.clone();
            let dir = crate::backend::data_dir()
                .join("portable-staging")
                .join(format!("{tag}-{}", std::process::id()));
            fs::create_dir_all(destination.parent().ok_or("Missing versions directory")?)
                .map_err(|e| e.to_string())?;
            fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
            let mut zip = zip::ZipArchive::new(Cursor::new(PAYLOAD)).map_err(|e| e.to_string())?;
            for i in 0..zip.len() {
                let mut entry = zip.by_index(i).map_err(|e| e.to_string())?;
                let path = entry
                    .enclosed_name()
                    .ok_or("Unsafe portable payload path")?;
                let path = dir.join(path);
                if entry.is_dir() {
                    fs::create_dir_all(path).map_err(|e| e.to_string())?;
                    continue;
                }
                fs::create_dir_all(path.parent().ok_or("Invalid payload path")?)
                    .map_err(|e| e.to_string())?;
                let mut file = fs::File::create(path).map_err(|e| e.to_string())?;
                std::io::copy(&mut entry, &mut file).map_err(|e| e.to_string())?;
            }
            for required in [
                "horizon-client.exe",
                "horizon-prepare.exe",
                "research-lock.json",
                "portable.json",
                "horizon-mixins.exe",
            ] {
                if !dir.join(required).is_file() {
                    return Err(format!("Portable payload lacks {required}"));
                }
            }
            fs::write(dir.join(".installed"), hash).map_err(|e| e.to_string())?;
            if let Err(error) = fs::rename(&dir, &destination) {
                if !destination.join(".installed").is_file() {
                    return Err(error.to_string());
                }
                // A concurrent launcher installed the same immutable payload first.
                let _ = fs::remove_dir_all(&dir);
            }
        }
        // A selected native build stays selected. No global bundled-client override.
        let mut settings = crate::backend::Settings::load();
        if settings.selected.is_none() {
            settings.selected = Some(tag);
            if settings.install_new_versions.is_none() {
                settings.install_new_versions = Some(false);
            }
            settings.save();
        }
    }
    Ok(())
}

/// Only reuse an embedded tool when its exact revision matches the mod's requested tool.
pub fn mixins(revision: &str) -> Option<std::path::PathBuf> {
    if !cfg!(feature = "portable") {
        return None;
    }
    for tag in crate::backend::installed() {
        if !tag.starts_with("portable-") {
            continue;
        }
        let dir = crate::backend::version_dir(&tag);
        let Some(meta) = std::fs::read(dir.join("portable.json"))
            .ok()
            .and_then(|b| serde_json::from_slice::<serde_json::Value>(&b).ok())
        else {
            continue;
        };
        if meta["mixins_revision"].as_str() == Some(revision) {
            let tool = dir.join("horizon-mixins.exe");
            if tool.is_file() {
                return Some(tool);
            }
        }
    }
    None
}
