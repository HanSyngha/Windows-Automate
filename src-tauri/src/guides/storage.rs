// Guide storage - file operations for guides

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Guide entry (file or folder)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuideEntry {
    pub name: String,
    pub path: String,
    pub is_dir: bool,
}

/// Guide index entry for depth-1 listing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuideIndexEntry {
    pub path: String,
    pub title: String,
}

/// Get the guides directory path
pub fn get_guides_dir() -> Result<PathBuf> {
    let data_dir = dirs::data_local_dir()
        .ok_or_else(|| anyhow::anyhow!("Could not find data directory"))?
        .join("automate")
        .join("guides");

    if !data_dir.exists() {
        fs::create_dir_all(&data_dir)?;
        // Create default folders
        fs::create_dir_all(data_dir.join("websites"))?;
        fs::create_dir_all(data_dir.join("applications"))?;
        fs::create_dir_all(data_dir.join("workflows"))?;
    }

    Ok(data_dir)
}

/// List guides in a directory (guide_ls)
pub fn list_guides(subpath: Option<&str>) -> Result<Vec<GuideEntry>> {
    let guides_dir = get_guides_dir()?;
    let target_dir = match subpath {
        Some(p) => guides_dir.join(p.trim_start_matches('/')),
        None => guides_dir,
    };

    if !target_dir.exists() {
        return Ok(Vec::new());
    }

    let mut entries = Vec::new();
    for entry in fs::read_dir(&target_dir)? {
        let entry = entry?;
        let metadata = entry.metadata()?;
        let name = entry.file_name().to_string_lossy().to_string();

        // Skip hidden files
        if name.starts_with('.') {
            continue;
        }

        let relative_path = entry
            .path()
            .strip_prefix(&guides_dir)
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|_| name.clone());

        entries.push(GuideEntry {
            name,
            path: relative_path,
            is_dir: metadata.is_dir(),
        });
    }

    // Sort: directories first, then alphabetically
    entries.sort_by(|a, b| match (a.is_dir, b.is_dir) {
        (true, false) => std::cmp::Ordering::Less,
        (false, true) => std::cmp::Ordering::Greater,
        _ => a.name.cmp(&b.name),
    });

    Ok(entries)
}

/// Get guide preview (first 10 lines) - guide_preview
pub fn preview_guide(path: &str) -> Result<String> {
    let guides_dir = get_guides_dir()?;
    let file_path = guides_dir.join(path.trim_start_matches('/'));

    if !file_path.exists() {
        return Err(anyhow::anyhow!("Guide not found: {}", path));
    }

    if file_path.is_dir() {
        return Err(anyhow::anyhow!("Path is a directory: {}", path));
    }

    let content = fs::read_to_string(&file_path)?;
    let preview: String = content.lines().take(10).collect::<Vec<_>>().join("\n");

    Ok(preview)
}

/// Read full guide content - guide_read
pub fn read_guide(path: &str) -> Result<String> {
    let guides_dir = get_guides_dir()?;
    let file_path = guides_dir.join(path.trim_start_matches('/'));

    if !file_path.exists() {
        return Err(anyhow::anyhow!("Guide not found: {}", path));
    }

    if file_path.is_dir() {
        return Err(anyhow::anyhow!("Path is a directory: {}", path));
    }

    let content = fs::read_to_string(&file_path)?;
    Ok(content)
}

/// Save a guide - for guide creation
pub fn save_guide(path: &str, content: &str) -> Result<()> {
    let guides_dir = get_guides_dir()?;
    let file_path = guides_dir.join(path.trim_start_matches('/'));

    // Create parent directories if needed
    if let Some(parent) = file_path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent)?;
        }
    }

    fs::write(&file_path, content)?;
    Ok(())
}

/// Get depth-1 guide index (folder/file.md format with titles)
pub fn get_guide_index() -> Result<Vec<GuideIndexEntry>> {
    let guides_dir = get_guides_dir()?;
    let mut index = Vec::new();

    // Iterate through top-level directories
    for folder_entry in fs::read_dir(&guides_dir)? {
        let folder_entry = folder_entry?;
        let folder_name = folder_entry.file_name().to_string_lossy().to_string();

        if folder_name.starts_with('.') || !folder_entry.metadata()?.is_dir() {
            continue;
        }

        // Iterate through files in each folder
        for file_entry in fs::read_dir(folder_entry.path())? {
            let file_entry = file_entry?;
            let file_name = file_entry.file_name().to_string_lossy().to_string();

            if file_name.starts_with('.') || !file_name.ends_with(".md") {
                continue;
            }

            // Extract title from frontmatter or first heading
            let title = extract_title(&file_entry.path())?;
            let path = format!("{}/{}", folder_name, file_name);

            index.push(GuideIndexEntry { path, title });
        }
    }

    // Sort alphabetically by path
    index.sort_by(|a, b| a.path.cmp(&b.path));

    Ok(index)
}

/// Extract title from markdown file
fn extract_title(path: &PathBuf) -> Result<String> {
    let content = fs::read_to_string(path)?;

    // Try to find title in frontmatter
    if content.starts_with("---") {
        if let Some(end) = content[3..].find("---") {
            let frontmatter = &content[3..end + 3];
            for line in frontmatter.lines() {
                if line.starts_with("title:") {
                    return Ok(line[6..].trim().trim_matches('"').to_string());
                }
            }
        }
    }

    // Fall back to first heading
    for line in content.lines() {
        if line.starts_with("# ") {
            return Ok(line[2..].trim().to_string());
        }
    }

    // Fall back to filename
    let filename = path
        .file_stem()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| "Untitled".to_string());

    Ok(filename)
}
