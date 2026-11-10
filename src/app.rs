use crate::models::{AppData, Period, TaskViewItem};
use crate::storage::Storage;
use chrono::{Local, NaiveDate};
use std::time::{Duration, Instant};

pub const TABS: [Period; 4] = [
    Period::Daily,
    Period::Weekly,
    Period::Monthly,
    Period::Yearly,
];

pub struct App {
    pub storage: Storage,
    pub data: AppData,
    pub selected_tab: usize,
    pub selected_index: usize,
    pub is_adding_task: bool,
    pub input_buffer: String,
    pub is_confirming_delete: bool,
    pub notification: Option<(String, Instant)>,
    pub should_quit: bool,
    pub today: NaiveDate,
}

impl App {
    pub fn new(storage: Storage) -> Self {
        let data = storage.load().unwrap_or_default();
        let today = Local::now().date_naive();

        Self {
            storage,
            data,
            selected_tab: 0,
            selected_index: 0,
            is_adding_task: false,
            input_buffer: String::new(),
            is_confirming_delete: false,
            notification: None,
            should_quit: false,
            today,
        }
    }

    pub fn current_period(&self) -> Period {
        TABS[self.selected_tab % TABS.len()]
    }

    pub fn current_items(&self) -> Vec<TaskViewItem> {
        self.data.generate_task_items(self.current_period(), self.today)
    }

    pub fn next_tab(&mut self) {
        self.selected_tab = (self.selected_tab + 1) % TABS.len();
        self.selected_index = 0;
    }

    pub fn prev_tab(&mut self) {
        if self.selected_tab == 0 {
            self.selected_tab = TABS.len() - 1;
        } else {
            self.selected_tab -= 1;
        }
        self.selected_index = 0;
    }

    pub fn set_tab(&mut self, index: usize) {
        if index < TABS.len() {
            self.selected_tab = index;
            self.selected_index = 0;
        }
    }

    pub fn move_up(&mut self) {
        let items_len = self.current_items().len();
        if items_len == 0 {
            self.selected_index = 0;
            return;
        }
        if self.selected_index > 0 {
            self.selected_index -= 1;
        } else {
            self.selected_index = items_len - 1;
        }
    }

    pub fn move_down(&mut self) {
        let items_len = self.current_items().len();
        if items_len == 0 {
            self.selected_index = 0;
            return;
        }
        if self.selected_index + 1 < items_len {
            self.selected_index += 1;
        } else {
            self.selected_index = 0;
        }
    }

    pub fn toggle_selected(&mut self) {
        let items = self.current_items();
        if items.is_empty() || self.selected_index >= items.len() {
            return;
        }

        let item = &items[self.selected_index];
        let was_completed = self.data.toggle_completion(item.goal_id, &item.period_key);

        if let Err(e) = self.storage.save(&self.data) {
            self.show_notification(format!("Lỗi lưu dữ liệu: {}", e));
        } else if was_completed {
            self.show_notification("Đã đánh dấu hoàn thành! ✓".to_string());
        } else {
            self.show_notification("Đã hủy đánh dấu hoàn thành.".to_string());
        }

        // Điều chỉnh lại vị trí con trỏ sau khi danh sách cập nhật
        let new_items_len = self.current_items().len();
        if new_items_len == 0 {
            self.selected_index = 0;
        } else if self.selected_index >= new_items_len {
            self.selected_index = new_items_len - 1;
        }
    }

    pub fn start_adding_task(&mut self) {
        self.is_adding_task = true;
        self.input_buffer.clear();
    }

    pub fn confirm_add_task(&mut self) {
        let title = self.input_buffer.trim().to_string();
        if !title.is_empty() {
            let period = self.current_period();
            self.data.add_goal(title, period, self.today);
            if let Err(e) = self.storage.save(&self.data) {
                self.show_notification(format!("Lỗi lưu: {}", e));
            } else {
                self.show_notification("Đã thêm mục tiêu mới!".to_string());
            }
        }
        self.is_adding_task = false;
        self.input_buffer.clear();
    }

    pub fn cancel_modal(&mut self) {
        self.is_adding_task = false;
        self.is_confirming_delete = false;
        self.input_buffer.clear();
    }

    pub fn start_confirm_delete(&mut self) {
        let items = self.current_items();
        if !items.is_empty() && self.selected_index < items.len() {
            self.is_confirming_delete = true;
        }
    }

    pub fn confirm_delete(&mut self) {
        let items = self.current_items();
        if !items.is_empty() && self.selected_index < items.len() {
            let goal_id = items[self.selected_index].goal_id;
            self.data.delete_goal(goal_id);
            if let Err(e) = self.storage.save(&self.data) {
                self.show_notification(format!("Lỗi lưu: {}", e));
            } else {
                self.show_notification("Đã xóa mục tiêu!".to_string());
            }
        }
        self.is_confirming_delete = false;

        let new_len = self.current_items().len();
        if new_len == 0 {
            self.selected_index = 0;
        } else if self.selected_index >= new_len {
            self.selected_index = new_len - 1;
        }
    }

    pub fn show_notification(&mut self, message: String) {
        self.notification = Some((message, Instant::now()));
    }

    pub fn current_notification(&self) -> Option<&str> {
        self.notification
            .as_ref()
            .filter(|(_, time)| time.elapsed() < Duration::from_secs(3))
            .map(|(msg, _)| msg.as_str())
    }

    /// Tính tổng số mục tiêu còn nợ trên toàn bộ 4 chu kỳ
    pub fn total_debt_count(&self) -> usize {
        let mut total = 0;
        for period in TABS {
            let items = self.data.generate_task_items(period, self.today);
            total += items.iter().filter(|i| i.is_debt && !i.is_completed).count();
        }
        total
    }

    /// Số mục tiêu chu kỳ hiện tại đã hoàn thành
    pub fn current_period_completed_count(&self) -> (usize, usize) {
        let items = self.current_items();
        let completed = items.iter().filter(|i| i.is_completed).count();
        (completed, items.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    fn setup_test_app() -> App {
        let temp_dir = env::temp_dir().join(format!("task_cli_app_test_{}", uuid::Uuid::new_v4()));
        let storage = Storage::with_path(temp_dir.join("test.json"));
        App::new(storage)
    }

    #[test]
    fn test_app_tabs_navigation() {
        let mut app = setup_test_app();
        assert_eq!(app.current_period(), Period::Daily);
        app.next_tab();
        assert_eq!(app.current_period(), Period::Weekly);
        app.next_tab();
        assert_eq!(app.current_period(), Period::Monthly);
        app.next_tab();
        assert_eq!(app.current_period(), Period::Yearly);
        app.next_tab();
        assert_eq!(app.current_period(), Period::Daily);
        app.prev_tab();
        assert_eq!(app.current_period(), Period::Yearly);
    }

    #[test]
    fn test_app_add_and_toggle_task() {
        let mut app = setup_test_app();
        app.start_adding_task();
        app.input_buffer = "Học Rust TUI".to_string();
        app.confirm_add_task();

        let items = app.current_items();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].title, "Học Rust TUI");
        assert!(!items[0].is_completed);

        // Toggle tick
        app.toggle_selected();
        let items = app.current_items();
        assert_eq!(items.len(), 1);
        assert!(items[0].is_completed);

        // Toggle untick
        app.toggle_selected();
        let items = app.current_items();
        assert_eq!(items.len(), 1);
        assert!(!items[0].is_completed);
    }
}
