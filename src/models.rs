use chrono::{Datelike, Days, NaiveDate};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Period {
    Daily,
    Weekly,
    Monthly,
    Yearly,
}

impl Period {
    pub fn name_vi(&self) -> &'static str {
        match self {
            Period::Daily => "Hàng ngày",
            Period::Weekly => "Hàng tuần",
            Period::Monthly => "Hàng tháng",
            Period::Yearly => "Hàng năm",
        }
    }

    /// Định dạng chu kỳ thành một key đại diện duy nhất
    pub fn get_key(&self, date: NaiveDate) -> String {
        match self {
            Period::Daily => date.format("%Y-%m-%d").to_string(),
            Period::Weekly => {
                let iso = date.iso_week();
                format!("{}-W{:02}", iso.year(), iso.week())
            }
            Period::Monthly => date.format("%Y-%m").to_string(),
            Period::Yearly => date.format("%Y").to_string(),
        }
    }

    /// Nhãn hiển thị cho chu kỳ hiện tại
    pub fn current_label(&self) -> &'static str {
        match self {
            Period::Daily => "Hôm nay",
            Period::Weekly => "Tuần này",
            Period::Monthly => "Tháng này",
            Period::Yearly => "Năm này",
        }
    }

    /// Nhãn hiển thị cho chu kỳ nợ
    pub fn debt_label(&self, date: NaiveDate) -> String {
        match self {
            Period::Daily => format!("Nợ từ {}", date.format("%d/%m")),
            Period::Weekly => {
                let iso = date.iso_week();
                format!("Nợ Tuần {}/{}", iso.week(), iso.year())
            }
            Period::Monthly => format!("Nợ Tháng {}", date.format("%m/%Y")),
            Period::Yearly => format!("Nợ Năm {}", date.format("%Y")),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Goal {
    pub id: Uuid,
    pub title: String,
    pub period: Period,
    pub created_date: NaiveDate,
}

impl Goal {
    pub fn new(title: String, period: Period, created_date: NaiveDate) -> Self {
        Self {
            id: Uuid::new_v4(),
            title,
            period,
            created_date,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Completion {
    pub goal_id: Uuid,
    pub period_key: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskViewItem {
    pub goal_id: Uuid,
    pub title: String,
    pub period: Period,
    pub period_key: String,
    pub is_debt: bool,
    pub label: String,
    pub is_completed: bool,
    pub date: NaiveDate,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AppData {
    pub goals: Vec<Goal>,
    pub completions: Vec<Completion>,
}

impl AppData {
    pub fn is_completed(&self, goal_id: Uuid, period_key: &str) -> bool {
        self.completions
            .iter()
            .any(|c| c.goal_id == goal_id && c.period_key == period_key)
    }

    pub fn toggle_completion(&mut self, goal_id: Uuid, period_key: &str) -> bool {
        if let Some(pos) = self
            .completions
            .iter()
            .position(|c| c.goal_id == goal_id && c.period_key == period_key)
        {
            self.completions.remove(pos);
            false
        } else {
            self.completions.push(Completion {
                goal_id,
                period_key: period_key.to_string(),
            });
            true
        }
    }

    pub fn add_goal(&mut self, title: String, period: Period, today: NaiveDate) {
        let goal = Goal::new(title, period, today);
        self.goals.push(goal);
    }

    pub fn delete_goal(&mut self, goal_id: Uuid) {
        self.goals.retain(|g| g.id != goal_id);
        self.completions.retain(|c| c.goal_id != goal_id);
    }

    /// Sinh danh sách các dòng checkbox hiển thị dựa trên ngày hôm nay.
    /// Bao gồm cả các dòng nhiệm vụ nợ chưa hoàn thành trong quá khứ và nhiệm vụ chu kỳ hiện tại.
    pub fn generate_task_items(&self, period: Period, today: NaiveDate) -> Vec<TaskViewItem> {
        let mut items = Vec::new();
        let current_key = period.get_key(today);

        let filtered_goals: Vec<&Goal> = self.goals.iter().filter(|g| g.period == period).collect();

        for goal in filtered_goals {
            match period {
                Period::Daily => {
                    let mut curr = goal.created_date;
                    while curr <= today {
                        let p_key = period.get_key(curr);
                        let is_current = p_key == current_key;
                        let completed = self.is_completed(goal.id, &p_key);

                        // Nếu là chu kỳ hiện tại HOẶC nếu là quá khứ mà CHƯA hoàn thành (nợ)
                        // Nếu trong quá khứ mà đã hoàn thành thì không cần hiển thị lại để tránh rác màn hình.
                        if is_current || !completed {
                            items.push(TaskViewItem {
                                goal_id: goal.id,
                                title: goal.title.clone(),
                                period: goal.period,
                                period_key: p_key,
                                is_debt: !is_current,
                                label: if is_current {
                                    period.current_label().to_string()
                                } else {
                                    period.debt_label(curr)
                                },
                                is_completed: completed,
                                date: curr,
                            });
                        }
                        if let Some(next) = curr.checked_add_days(Days::new(1)) {
                            curr = next;
                        } else {
                            break;
                        }
                    }
                }
                Period::Weekly => {
                    // Lặp tuần: nhảy mỗi 7 ngày bắt đầu từ ngày tạo
                    let mut curr = goal.created_date;
                    let mut visited_keys = std::collections::HashSet::new();

                    while curr <= today {
                        let p_key = period.get_key(curr);
                        if visited_keys.insert(p_key.clone()) {
                            let is_current = p_key == current_key;
                            let completed = self.is_completed(goal.id, &p_key);

                            if is_current || !completed {
                                items.push(TaskViewItem {
                                    goal_id: goal.id,
                                    title: goal.title.clone(),
                                    period: goal.period,
                                    period_key: p_key,
                                    is_debt: !is_current,
                                    label: if is_current {
                                        period.current_label().to_string()
                                    } else {
                                        period.debt_label(curr)
                                    },
                                    is_completed: completed,
                                    date: curr,
                                });
                            }
                        }
                        if let Some(next) = curr.checked_add_days(Days::new(7)) {
                            curr = next;
                        } else {
                            break;
                        }
                    }
                }
                Period::Monthly => {
                    // Lặp qua từng tháng: từ created_date đến today
                    let start_year = goal.created_date.year();
                    let start_month = goal.created_date.month();
                    let end_year = today.year();
                    let end_month = today.month();

                    let mut y = start_year;
                    let mut m = start_month;

                    while (y < end_year) || (y == end_year && m <= end_month) {
                        let sample_date = NaiveDate::from_ymd_opt(y, m, 1).unwrap_or(today);
                        let p_key = period.get_key(sample_date);
                        let is_current = p_key == current_key;
                        let completed = self.is_completed(goal.id, &p_key);

                        if is_current || !completed {
                            items.push(TaskViewItem {
                                goal_id: goal.id,
                                title: goal.title.clone(),
                                period: goal.period,
                                period_key: p_key,
                                is_debt: !is_current,
                                label: if is_current {
                                    period.current_label().to_string()
                                } else {
                                    period.debt_label(sample_date)
                                },
                                is_completed: completed,
                                date: sample_date,
                            });
                        }

                        if m == 12 {
                            y += 1;
                            m = 1;
                        } else {
                            m += 1;
                        }
                    }
                }
                Period::Yearly => {
                    let start_year = goal.created_date.year();
                    let end_year = today.year();

                    for y in start_year..=end_year {
                        let sample_date = NaiveDate::from_ymd_opt(y, 1, 1).unwrap_or(today);
                        let p_key = period.get_key(sample_date);
                        let is_current = p_key == current_key;
                        let completed = self.is_completed(goal.id, &p_key);

                        if is_current || !completed {
                            items.push(TaskViewItem {
                                goal_id: goal.id,
                                title: goal.title.clone(),
                                period: goal.period,
                                period_key: p_key,
                                is_debt: !is_current,
                                label: if is_current {
                                    period.current_label().to_string()
                                } else {
                                    period.debt_label(sample_date)
                                },
                                is_completed: completed,
                                date: sample_date,
                            });
                        }
                    }
                }
            }
        }

        // Sắp xếp: Các task nợ (càng cũ) ưu tiên hiển thị trước để người dùng bù trước, sau đó đến task hiện tại
        items.sort_by(|a, b| {
            // Nợ ưu tiên lên đầu
            match (a.is_debt, b.is_debt) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => a.date.cmp(&b.date).then_with(|| a.title.cmp(&b.title)),
            }
        });

        items
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_daily_debt_generation() {
        let mut app_data = AppData::default();
        let create_date = NaiveDate::from_ymd_opt(2026, 11, 8).unwrap();
        let today = NaiveDate::from_ymd_opt(2026, 11, 10).unwrap();

        app_data.add_goal("Đọc sách 30p".to_string(), Period::Daily, create_date);

        let items = app_data.generate_task_items(Period::Daily, today);
        // Ngày 8/11 (nợ), Ngày 9/11 (nợ), Ngày 10/11 (hôm nay) => 3 items
        assert_eq!(items.len(), 3);
        assert!(items[0].is_debt);
        assert_eq!(items[0].period_key, "2026-11-08");
        assert!(items[1].is_debt);
        assert_eq!(items[1].period_key, "2026-11-09");
        assert!(!items[2].is_debt);
        assert_eq!(items[2].period_key, "2026-11-10");
        assert_eq!(items[2].label, "Hôm nay");
    }

    #[test]
    fn test_complete_debt_removes_from_pending_list() {
        let mut app_data = AppData::default();
        let create_date = NaiveDate::from_ymd_opt(2026, 11, 8).unwrap();
        let today = NaiveDate::from_ymd_opt(2026, 11, 10).unwrap();

        app_data.add_goal("Chạy bộ".to_string(), Period::Daily, create_date);
        let goal_id = app_data.goals[0].id;

        // Trả nợ cho ngày 8/11
        app_data.toggle_completion(goal_id, "2026-11-08");

        let items = app_data.generate_task_items(Period::Daily, today);
        // Ngày 8 đã trả nợ nên không còn xuất hiện, chỉ còn ngày 9 (nợ) và ngày 10 (hôm nay)
        assert_eq!(items.len(), 2);
        assert_eq!(items[0].period_key, "2026-11-09");
        assert_eq!(items[1].period_key, "2026-11-10");
    }

    #[test]
    fn test_weekly_and_monthly_debt() {
        let mut app_data = AppData::default();
        let oct_date = NaiveDate::from_ymd_opt(2026, 10, 1).unwrap();
        let nov_date = NaiveDate::from_ymd_opt(2026, 11, 10).unwrap();

        app_data.add_goal("Báo cáo tháng".to_string(), Period::Monthly, oct_date);

        let items = app_data.generate_task_items(Period::Monthly, nov_date);
        // Tháng 10 (nợ) và tháng 11 (tháng này) => 2 items
        assert_eq!(items.len(), 2);
        assert!(items[0].is_debt);
        assert_eq!(items[0].period_key, "2026-10");
        assert!(!items[1].is_debt);
        assert_eq!(items[1].period_key, "2026-11");
    }
}
