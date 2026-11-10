use crate::app::{App, TABS};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, List, ListItem, Paragraph, Tabs},
    Frame,
};

pub fn render_ui(frame: &mut Frame, app: &App) {
    let size = frame.area();

    // Chia bố cục màn hình: Header (3), Tabs (3), Danh sách (min 5), Footer (3)
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Length(3), // Tabs
            Constraint::Min(5),    // List tasks
            Constraint::Length(3), // Footer / Key hints
        ])
        .split(size);

    render_header(frame, app, chunks[0]);
    render_tabs(frame, app, chunks[1]);
    render_task_list(frame, app, chunks[2]);
    render_footer(frame, app, chunks[3]);

    // Render Modals nếu đang mở
    if app.is_adding_task {
        render_add_modal(frame, app, size);
    } else if app.is_confirming_delete {
        render_delete_modal(frame, app, size);
    }
}

fn render_header(frame: &mut Frame, app: &App, area: Rect) {
    let total_debt = app.total_debt_count();
    let (completed, total) = app.current_period_completed_count();

    let debt_style = if total_debt > 0 {
        Style::default()
            .fg(Color::LightRed)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::LightGreen)
    };

    let debt_text = if total_debt > 0 {
        format!("⚠️  Tổng nợ: {} mục tiêu", total_debt)
    } else {
        "✓ Không có nợ!".to_string()
    };

    let title_line = Line::from(vec![
        Span::styled(
            " TASK-CLI ",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" | "),
        Span::styled(
            format!("Ngày: {}", app.today.format("%d/%m/%Y")),
            Style::default().fg(Color::White),
        ),
        Span::raw(" | "),
        Span::styled(debt_text, debt_style),
        Span::raw(" | "),
        Span::styled(
            format!("Tiến độ: {}/{}", completed, total),
            Style::default().fg(Color::Yellow),
        ),
    ]);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Cyan));

    let paragraph = Paragraph::new(title_line)
        .block(block)
        .alignment(Alignment::Center);

    frame.render_widget(paragraph, area);
}

fn render_tabs(frame: &mut Frame, app: &App, area: Rect) {
    let titles: Vec<Line> = TABS
        .iter()
        .enumerate()
        .map(|(i, period)| {
            let num = i + 1;
            Line::from(format!("[{}] {}", num, period.name_vi()))
        })
        .collect();

    let tabs = Tabs::new(titles)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title(" Chu kỳ ")
                .border_style(Style::default().fg(Color::DarkGray)),
        )
        .select(app.selected_tab)
        .style(Style::default().fg(Color::Gray))
        .highlight_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        );

    frame.render_widget(tabs, area);
}

fn render_task_list(frame: &mut Frame, app: &App, area: Rect) {
    let items = app.current_items();

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title(format!(" Danh sách mục tiêu - {} ", app.current_period().name_vi()))
        .border_style(Style::default().fg(Color::LightBlue));

    if items.is_empty() {
        let empty_msg = Paragraph::new("Chưa có mục tiêu nào trong chu kỳ này.\nBấm phím [a] để thêm mục tiêu mới!")
            .block(block)
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::DarkGray));
        frame.render_widget(empty_msg, area);
        return;
    }

    let list_items: Vec<ListItem> = items
        .iter()
        .enumerate()
        .map(|(idx, item)| {
            let is_selected = idx == app.selected_index;

            let checkbox = if item.is_completed {
                Span::styled(
                    "[x] ",
                    Style::default()
                        .fg(Color::LightGreen)
                        .add_modifier(Modifier::BOLD),
                )
            } else {
                Span::styled("[ ] ", Style::default().fg(Color::White))
            };

            let title_style = if item.is_completed {
                Style::default()
                    .fg(Color::DarkGray)
                    .add_modifier(Modifier::CROSSED_OUT)
            } else if item.is_debt {
                Style::default()
                    .fg(Color::LightYellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };

            let title_span = Span::styled(&item.title, title_style);

            let label_span = if item.is_debt {
                Span::styled(
                    format!("  ({})", item.label),
                    Style::default()
                        .fg(Color::LightRed)
                        .add_modifier(Modifier::BOLD),
                )
            } else {
                Span::styled(
                    format!("  ({})", item.label),
                    Style::default().fg(Color::DarkGray),
                )
            };

            let prefix = if is_selected {
                Span::styled(
                    "▶ ",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                )
            } else {
                Span::raw("  ")
            };

            let mut line = Line::from(vec![prefix, checkbox, title_span, label_span]);

            if is_selected {
                line = line.patch_style(Style::default().bg(Color::Rgb(35, 45, 60)));
            }

            ListItem::new(line)
        })
        .collect();

    let list = List::new(list_items).block(block);
    frame.render_widget(list, area);
}

fn render_footer(frame: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::DarkGray));

    let content = if let Some(notification) = app.current_notification() {
        Line::from(vec![
            Span::styled(" 🔔 ", Style::default().fg(Color::Yellow)),
            Span::styled(
                notification,
                Style::default()
                    .fg(Color::LightYellow)
                    .add_modifier(Modifier::BOLD),
            ),
        ])
    } else {
        Line::from(vec![
            Span::styled("[↑/↓] ", Style::default().fg(Color::Cyan)),
            Span::raw("Di chuyển  "),
            Span::styled("[Space] ", Style::default().fg(Color::Cyan)),
            Span::raw("Tick/Bỏ tick  "),
            Span::styled("[Tab/1-4] ", Style::default().fg(Color::Cyan)),
            Span::raw("Đổi Tab  "),
            Span::styled("[a] ", Style::default().fg(Color::Green)),
            Span::raw("Thêm  "),
            Span::styled("[d] ", Style::default().fg(Color::Red)),
            Span::raw("Xóa  "),
            Span::styled("[q] ", Style::default().fg(Color::Magenta)),
            Span::raw("Thoát"),
        ])
    };

    let paragraph = Paragraph::new(content)
        .block(block)
        .alignment(Alignment::Center);

    frame.render_widget(paragraph, area);
}

fn render_add_modal(frame: &mut Frame, app: &App, area: Rect) {
    let popup_area = centered_rect(60, 25, area);

    frame.render_widget(Clear, popup_area);

    let title = format!(" ➕ Thêm mục tiêu mới ({}) ", app.current_period().name_vi());
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .border_style(Style::default().fg(Color::LightGreen))
        .title(title);

    let inner_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Length(1),
        ])
        .margin(1)
        .split(popup_area);

    frame.render_widget(block, popup_area);

    let prompt = Paragraph::new("Nhập tên mục tiêu cần thực hiện:");
    frame.render_widget(prompt, inner_layout[0]);

    let input_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Yellow));
    let input = Paragraph::new(format!("{}▏", app.input_buffer)).block(input_block);
    frame.render_widget(input, inner_layout[1]);

    let help = Paragraph::new("Phím tắt: [Enter] Lưu | [Esc] Hủy")
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::DarkGray));
    frame.render_widget(help, inner_layout[2]);
}

fn render_delete_modal(frame: &mut Frame, app: &App, area: Rect) {
    let popup_area = centered_rect(60, 25, area);

    frame.render_widget(Clear, popup_area);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .border_style(Style::default().fg(Color::Red))
        .title(" ⚠️  Xác nhận xóa mục tiêu ");

    let items = app.current_items();
    let task_title = if !items.is_empty() && app.selected_index < items.len() {
        &items[app.selected_index].title
    } else {
        ""
    };

    let content = vec![
        Line::from(vec![
            Span::raw("Bạn có chắc chắn muốn xóa mục tiêu: "),
            Span::styled(
                format!("\"{}\"", task_title),
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" ?"),
        ]),
        Line::from("Hành động này sẽ xóa vĩnh viễn mục tiêu cùng toàn bộ lịch sử!"),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "[y / Enter] Xác nhận xóa",
                Style::default()
                    .fg(Color::Red)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("   |   "),
            Span::styled("[n / Esc] Hủy bỏ", Style::default().fg(Color::Gray)),
        ]),
    ];

    let paragraph = Paragraph::new(content)
        .block(block)
        .alignment(Alignment::Center);

    frame.render_widget(paragraph, popup_area);
}

/// Helper tạo hình chữ nhật căn giữa màn hình
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::Storage;
    use ratatui::backend::TestBackend;
    use ratatui::Terminal;
    use std::env;

    #[test]
    fn test_ui_render_without_panic() {
        let temp_dir = env::temp_dir().join(format!("task_cli_ui_test_{}", uuid::Uuid::new_v4()));
        let storage = Storage::with_path(temp_dir.join("ui_test.json"));
        let mut app = App::new(storage);

        let backend = TestBackend::new(100, 30);
        let mut terminal = Terminal::new(backend).unwrap();

        // Test render bình thường
        terminal.draw(|f| render_ui(f, &app)).unwrap();

        // Test render khi đang mở modal thêm task
        app.is_adding_task = true;
        terminal.draw(|f| render_ui(f, &app)).unwrap();

        // Test render khi đang mở modal xóa task
        app.is_adding_task = false;
        app.is_confirming_delete = true;
        terminal.draw(|f| render_ui(f, &app)).unwrap();
    }
}

