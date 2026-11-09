//! Background, cancellable discovery of verified Minecraft client jars.
use crate::sources::{self, Pins};
use std::{
    collections::{HashSet, VecDeque},
    fs,
    path::PathBuf,
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc::{self, Receiver},
        Arc,
    },
};

pub enum Event {
    Progress(u64),
    Finished(Option<PathBuf>),
}
pub struct Search {
    pub events: Receiver<Event>,
    cancel: Arc<AtomicBool>,
}
impl Drop for Search {
    fn drop(&mut self) {
        self.cancel.store(true, Ordering::Relaxed);
    }
}

pub fn common_roots() -> Vec<PathBuf> {
    let mut roots = Vec::new();
    roots.extend(dirs::download_dir());
    roots.extend(dirs::desktop_dir());
    if let Some(f) = sources::Folders::current() {
        roots.extend(
            sources::known_folders(sources::Os::current(), &f)
                .into_iter()
                .map(|(_, p)| p),
        );
        roots.extend([
            f.data.join(".tlauncher"),
            f.data.join("TLauncher"),
            f.home.join(".tlauncher"),
        ]);
    }
    roots
}

pub fn start(roots: Vec<PathBuf>, pins: Pins) -> Search {
    let (tx, events) = mpsc::channel();
    let cancel = Arc::new(AtomicBool::new(false));
    let stop = cancel.clone();
    std::thread::spawn(move || {
        let found = scan(roots, &pins, &stop, |count| {
            let _ = tx.send(Event::Progress(count));
        });
        let _ = tx.send(Event::Finished(found));
    });
    Search { events, cancel }
}

fn scan(
    roots: Vec<PathBuf>,
    pins: &Pins,
    stop: &AtomicBool,
    progress: impl Fn(u64),
) -> Option<PathBuf> {
    let mut queue: VecDeque<_> = roots.into();
    let mut seen = HashSet::new();
    let mut count = 0;
    while let Some(dir) = queue.pop_front() {
        if stop.load(Ordering::Relaxed) {
            return None;
        }
        let Ok(meta) = fs::symlink_metadata(&dir) else {
            continue;
        };
        if skip(&meta) {
            continue;
        }
        let Ok(real) = dir.canonicalize() else {
            continue;
        };
        if !seen.insert(real) {
            continue;
        }
        let Ok(entries) = fs::read_dir(&dir) else {
            continue;
        };
        for entry in entries.flatten() {
            if stop.load(Ordering::Relaxed) {
                return None;
            }
            count += 1;
            if count % 256 == 0 {
                progress(count);
            }
            let path = entry.path();
            let Ok(meta) = fs::symlink_metadata(&path) else {
                continue;
            };
            if skip(&meta) {
                continue;
            }
            if meta.is_dir() {
                queue.push_back(path);
                continue;
            }
            let name = entry.file_name().to_string_lossy().to_ascii_lowercase();
            let candidate = name == format!("{}.jar", pins.version).to_ascii_lowercase()
                || (name.starts_with("horizon") && name.ends_with(".jar"))
                || name == format!("minecraft-{}-client.jar", pins.version).to_ascii_lowercase();
            if candidate && sources::sha1_file(&path).as_deref() == Some(&pins.client_sha1) {
                return Some(path);
            }
        }
    }
    None
}
fn skip(meta: &fs::Metadata) -> bool {
    if meta.file_type().is_symlink() {
        return true;
    }
    #[cfg(windows)]
    {
        use std::os::windows::fs::MetadataExt;
        if meta.file_attributes() & 0x400 != 0 {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn finds_nested_verified_jar_and_rejects_wrong_content() {
        let root = std::env::temp_dir().join(format!("horizon-search-{}", std::process::id()));
        fs::create_dir_all(root.join("nested")).unwrap();
        fs::write(root.join("26.3.jar"), b"wrong").unwrap();
        let correct = root.join("nested/horizon-client.jar");
        fs::write(&correct, b"abc").unwrap();
        let pins = Pins {
            version: "26.3".into(),
            client_sha1: "a9993e364706816aba3e25717850c26c9cd0d89d".into(),
            manifests: vec![],
        };
        assert_eq!(
            scan(vec![root.clone()], &pins, &AtomicBool::new(false), |_| {}),
            Some(correct)
        );
        assert_eq!(
            scan(vec![root.clone()], &pins, &AtomicBool::new(true), |_| {}),
            None
        );
        fs::remove_dir_all(root).unwrap();
    }
}
