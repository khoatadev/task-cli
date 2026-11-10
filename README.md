# 📋 task-cli

> **Trình quản lý mục tiêu Checkbox trên Terminal với cơ chế cộng dồn nợ (Debt Accumulation).**  
> Chạy độc lập, siêu nhẹ và hỗ trợ mượt mà trên cả 3 hệ điều hành: **macOS, Linux và Windows**.

---

## ✨ Điểm nổi bật (Features)

- 🎯 **Quản lý đa chu kỳ:** Đặt và theo dõi mục tiêu theo **Hàng ngày (Daily)**, **Hàng tuần (Weekly)**, **Hàng tháng (Monthly)** và **Hàng năm (Yearly)**.
- ⚡ **Giao diện TUI tương tác (Ratatui):** Giao diện bảng đẹp mắt, hỗ trợ phím điều hướng, đổi tab nhanh, tick/untick bằng phím Space.
- ⏳ **Cơ chế Nợ & Cộng dồn không bao giờ xóa:**
  - Nếu ngày hôm nay bạn không hoàn thành mục tiêu, ngày hôm sau nó sẽ trở thành một **nhiệm vụ nợ** kèm mốc thời gian (ví dụ: `[ ] Đọc sách (Nợ từ 09/11)`).
  - Càng nhiều ngày không làm thì các dòng nợ sẽ tiếp tục cộng dồn lại theo thời gian.
  - Bạn **bắt buộc phải làm bù** và tick hoàn thành để trừ hết nợ!
- 💾 **Lưu trữ dữ liệu tự động & Cross-platform:**
  - Tự động nhận diện đường dẫn chuẩn của hệ điều hành:
    - **Linux / macOS:** `~/.local/share/task-cli/tasks.json`
    - **Windows:** `%APPDATA%\task-cli\tasks.json`
- 🛡️ **An toàn Terminal:** Xử lý thoát sạch sẽ (Clean terminal exit), bảo toàn màn hình và không gây loạn phông terminal.

---

## 🚀 Cài đặt & Sử dụng (Installation & Usage)

### 1. Yêu cầu hệ thống
- Đã cài đặt [Rust & Cargo](https://rustup.rs/) (phiên bản 1.75 trở lên).

### 2. Tải mã nguồn & Chạy trực tiếp
```bash
git clone https://github.com/khoatadev/task-cli.git
cd task-cli
cargo run
```

### 3. Cài đặt vào hệ thống (Toàn cục)
```bash
cargo install --path .
```
Sau khi cài đặt xong, bạn chỉ cần gõ lệnh ở bất kỳ terminal nào:
```bash
task-cli
```

---

## ⌨️ Phím tắt điều khiển (Keybindings)

| Phím tắt | Chức năng |
| :--- | :--- |
| `↑` hoặc `k` | Di chuyển lên dòng phía trên |
| `↓` hoặc `j` | Di chuyển xuống dòng phía dưới |
| `Space` | **Tick / Bỏ tick** mục tiêu (hoàn thành / trả nợ) |
| `Tab` hoặc `→` / `l` | Chuyển sang Tab chu kỳ tiếp theo |
| `Shift + Tab` hoặc `←` / `h` | Chuyển về Tab chu kỳ trước đó |
| `1`, `2`, `3`, `4` | Nhảy trực tiếp đến Tab (1: Ngày, 2: Tuần, 3: Tháng, 4: Năm) |
| `a` hoặc `n` | **Thêm mục tiêu mới** cho chu kỳ hiện tại |
| `d` hoặc `Delete` | **Xóa mục tiêu** đang chọn (có hộp thoại xác nhận) |
| `q` hoặc `Esc` / `Ctrl+C` | Thoát ứng dụng (tự động lưu trạng thái) |

---

## 📁 Cấu trúc thư mục dữ liệu

Dữ liệu nhiệm vụ được tự động đồng bộ và lưu trữ dạng JSON:
- **Linux:** `/home/<user>/.local/share/task-cli/tasks.json`
- **macOS:** `/Users/<user>/Library/Application Support/task-cli/tasks.json`
- **Windows:** `C:\Users\<user>\AppData\Roaming\task-cli\tasks.json`

---

## 🛠️ Phát triển & Kiểm thử (Development)

- **Chạy kiểm thử:**
  ```bash
  cargo test
  ```
- **Kiểm tra tiêu chuẩn mã nguồn (Clippy):**
  ```bash
  cargo clippy -- -D warnings
  ```
- **Biên dịch bản Release tối ưu hóa:**
  ```bash
  cargo build --release
  ```

---

## 📄 Bản quyền (License)

Dự án được phân phối dưới giấy phép **MIT License**.
