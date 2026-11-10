use crate::models::AppData;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub const APP_DIR_NAME: &str = "task-cli";
pub const DATA_FILE_NAME: &str = "tasks.json";

/// Lấy đường dẫn lưu file dữ liệu mặc định của hệ điều hành
pub fn default_data_path() -> io::Result<PathBuf> {
    let base_dir = dirs::data_local_dir()
        .or_else(dirs::data_dir)
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Không tìm thấy thư mục data của OS"))?;

    let app_dir = base_dir.join(APP_DIR_NAME);
    Ok(app_dir.join(DATA_FILE_NAME))
}

pub struct Storage {
    file_path: PathBuf,
}

impl Storage {
    pub fn new() -> io::Result<Self> {
        let file_path = default_data_path()?;
        Ok(Self { file_path })
    }

    pub fn with_path(file_path: PathBuf) -> Self {
        Self { file_path }
    }

    pub fn path(&self) -> &Path {
        &self.file_path
    }

    pub fn load(&self) -> io::Result<AppData> {
        if !self.file_path.exists() {
            return Ok(AppData::default());
        }

        let content = fs::read_to_string(&self.file_path)?;
        if content.trim().is_empty() {
            return Ok(AppData::default());
        }

        let data: AppData = serde_json::from_str(&content)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        Ok(data)
    }

    pub fn save(&self, data: &AppData) -> io::Result<()> {
        if let Some(parent) = self.file_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let content = serde_json::to_string_pretty(data)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        fs::write(&self.file_path, content)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Period;
    use chrono::NaiveDate;
    use std::env;

    #[test]
    fn test_storage_save_and_load() {
        let temp_dir = env::temp_dir().join(format!("task_cli_test_{}", uuid::Uuid::new_v4()));
        let test_file = temp_dir.join("tasks_test.json");
        let storage = Storage::with_path(test_file.clone());

        let mut app_data = AppData::default();
        let date = NaiveDate::from_ymd_opt(2026, 11, 10).unwrap();
        app_data.add_goal("Viết test".to_string(), Period::Daily, date);

        assert!(storage.save(&app_data).is_ok());

        let loaded = storage.load().expect("Load failed");
        assert_eq!(loaded.goals.len(), 1);
        assert_eq!(loaded.goals[0].title, "Viết test");

        // Dọn dẹp thư mục tạm
        let _ = fs::remove_dir_all(temp_dir);
    }
}
