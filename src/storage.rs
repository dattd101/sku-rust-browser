use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Clone, Serialize, Deserialize)]
pub struct Entry { pub url: String, pub title: String, pub visited_at: u64 }

#[derive(Default, Serialize, Deserialize)]
pub struct Data { pub history: Vec<Entry>, pub bookmarks: Vec<Entry>, #[serde(default = "default_adblock")] pub adblock: bool }
fn default_adblock() -> bool { true }

fn path() -> PathBuf {
    let base = std::env::var_os("LOCALAPPDATA").map(PathBuf::from)
        .or_else(|| std::env::var_os("APPDATA").map(PathBuf::from))
        .unwrap_or_else(|| PathBuf::from("."));
    base.join("MinimalBrowser").join("data.json")
}
impl Data {
    pub fn load() -> Self { fs::read(path()).ok().and_then(|b| serde_json::from_slice(&b).ok()).unwrap_or(Self { adblock: true, ..Self::default() }) }
    pub fn save(&self) { let p = path(); if let Some(dir) = p.parent() { let _ = fs::create_dir_all(dir); } if let Ok(bytes) = serde_json::to_vec_pretty(self) { let _ = fs::write(p, bytes); } }
    pub fn visit(&mut self, url: &str) {
        if !url.starts_with("http://") && !url.starts_with("https://") { return; }
        let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs();
        self.history.push(Entry { url: url.into(), title: url.into(), visited_at: now });
        if self.history.len() > 2000 { self.history.drain(..self.history.len()-2000); }
        self.save();
    }
    pub fn toggle_bookmark(&mut self, url: &str) {
        if let Some(i) = self.bookmarks.iter().position(|e| e.url == url) { self.bookmarks.remove(i); }
        else if url.starts_with("https://") || url.starts_with("http://") {
            self.bookmarks.push(Entry { url: url.into(), title: url.into(), visited_at: 0 });
        }
        self.save();
    }
}
