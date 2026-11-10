pub mod app;
pub mod models;
pub mod storage;
pub mod ui;

use app::App;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use storage::Storage;
use std::io::{self, stdout};
use std::panic;
use std::time::Duration;

fn main() -> io::Result<()> {
    // Thiết lập panic hook để luôn khôi phục terminal nếu có lỗi
    let default_hook = panic::take_hook();
    panic::set_hook(Box::new(move |info| {
        let _ = reset_terminal();
        default_hook(info);
    }));

    // Khởi tạo terminal
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Khởi tạo Storage và App
    let storage = Storage::new()?;
    let mut app = App::new(storage);

    // Chạy event loop
    let res = run_loop(&mut terminal, &mut app);

    // Dọn dẹp terminal khi thoát
    reset_terminal()?;

    if let Err(err) = res {
        eprintln!("Lỗi ứng dụng: {:?}", err);
    }

    Ok(())
}

fn reset_terminal() -> io::Result<()> {
    disable_raw_mode()?;
    execute!(stdout(), LeaveAlternateScreen)?;
    Ok(())
}

fn run_loop(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, app: &mut App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui::render_ui(f, app))?;

        if app.should_quit {
            break;
        }

        // Đợi sự kiện phím với timeout 100ms để refresh định kỳ
        if event::poll(Duration::from_millis(100))? {
            let Event::Key(key) = event::read()? else {
                continue;
            };

            // Trên Windows và crossterm hiện đại, chỉ xử lý sự kiện Press để tránh double trigger
            if key.kind != KeyEventKind::Press {
                continue;
            }

            // Xử lý khi đang mở modal thêm task
            if app.is_adding_task {
                match key.code {
                    KeyCode::Enter => {
                        app.confirm_add_task();
                    }
                    KeyCode::Esc => {
                        app.cancel_modal();
                    }
                    KeyCode::Backspace => {
                        app.input_buffer.pop();
                    }
                    KeyCode::Char(c) => {
                        app.input_buffer.push(c);
                    }
                    _ => {}
                }
                continue;
            }

            // Xử lý khi đang mở modal xác nhận xóa
            if app.is_confirming_delete {
                match key.code {
                    KeyCode::Char('y') | KeyCode::Char('Y') | KeyCode::Enter => {
                        app.confirm_delete();
                    }
                    KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
                        app.cancel_modal();
                    }
                    _ => {}
                }
                continue;
            }

            // Xử lý phím ở màn hình chính
            match key.code {
                KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    app.should_quit = true;
                }
                KeyCode::Char('q') | KeyCode::Esc => {
                    app.should_quit = true;
                }
                KeyCode::Up | KeyCode::Char('k') => {
                    app.move_up();
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    app.move_down();
                }
                KeyCode::Char(' ') => {
                    app.toggle_selected();
                }
                KeyCode::Tab | KeyCode::Right | KeyCode::Char('l') => {
                    app.next_tab();
                }
                KeyCode::BackTab | KeyCode::Left | KeyCode::Char('h') => {
                    app.prev_tab();
                }
                KeyCode::Char('1') => {
                    app.set_tab(0);
                }
                KeyCode::Char('2') => {
                    app.set_tab(1);
                }
                KeyCode::Char('3') => {
                    app.set_tab(2);
                }
                KeyCode::Char('4') => {
                    app.set_tab(3);
                }
                KeyCode::Char('a') | KeyCode::Char('n') => {
                    app.start_adding_task();
                }
                KeyCode::Char('d') | KeyCode::Delete => {
                    app.start_confirm_delete();
                }
                _ => {}
            }
        }
    }

    Ok(())
}
