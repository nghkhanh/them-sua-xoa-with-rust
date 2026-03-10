# Rust Axum Web Server

Dự án này sử dụng framework **Axum** để định tuyến (routing), **PostgreSQL** làm cơ sở dữ liệu và **Clorinde** để tự động sinh code Rust an toàn (type-safe) từ các truy vấn SQL.

## Các tính năng chính

- 🚀 **Axum Backend**: Hệ thống routing nhanh, mạnh mẽ và tiện lợi.
- 🐘 **PostgreSQL & Clorinde**: Tương tác với cơ sở dữ liệu theo mô hình SQL-first. Code Rust sẽ được tự động sinh ra từ câu lệnh SQL.
- 📖 **Swagger UI**: Tự động tạo và hiển thị tài liệu OpenAPI thông quan thư viện `utoipa`.
- 🛠️ **Justfile**: Đơn giản hóa các câu lệnh môi trường cho lập trình viên (khởi chạy, theo dõi code...).

## Cấu trúc Dự án

Đây là một Workspace bao gồm nhiều Crate (module) kết hợp lại:

- `crates/web-server`: Điểm neo chính của ứng dụng. Chứa Axum router, cấu hình Swagger UI, và các hàm xử lý HTTP (handlers).
- `crates/db`: Quản lý Pool kết nối đến cơ sở dữ liệu (sử dụng `deadpool-postgres`) và chứa các câu truy vấn SQL thô trong thư mục `crates/db/queries`. Crate này cũng chứa tính năng migrations DB.
- `crates/clorinde`: Crate tự động sinh code. Nó chứa các structs và hàm do tool Clorinde tự động tạo ra dựa trên các câu lệnh SQL ở `crates/db/queries`.

## Hướng dẫn

### Yêu cầu cài đặt

Đảm bảo máy tính của bạn đã cài đặt các công cụ sau:
- [Rust](https://rustup.rs/) (phiên bản stable mới nhất)
- [Just](https://github.com/casey/just) (trình chạy lệnh command runner)
- [Docker](https://www.docker.com/) & Docker Compose (dùng cho database cục bộ)
- [dbmate](https://github.com/amacneil/dbmate) (dùng để chạy database migrations)

### Chạy ứng dụng

1. **Khởi động Database**
   Đảm bảo rằng database PostgreSQL trên máy bạn đang bật (thông qua Docker hoặc cấu hình `stack` của dự án).

2. **Chạy Migrations**
   Áp dụng database migrations vào DB để khởi tạo khung các bảng (schema & tables):
   ```bash
   dbmate up
   ```

3. **Chạy Server**
   Dùng `just` để khởi động server cùng với tính năng hot-reloading (tự động build khi code thay đổi):
   ```bash
   just watch
   ```

### Tài liệu API (Swagger UI)

Ngay sau khi Web server khởi động thành công, bạn có thể ngay lập tức truy cập tài liệu giao diện web của API (Swagger UI) thông qua trình duyệt:

👉 **[http://localhost:3000/swagger-ui/](http://localhost:3000/swagger-ui/)**

Bộ tài liệu này có sẵn đầy đủ các API thao tác CRUD (Tạo, Đọc, Sửa, Xóa) cho thực thể `Users`.

## Development

### Cách tạo thêm câu truy vấn DB mới

1. Viết câu SQL của bạn vào file `crates/db/queries/users.sql` (hoặc khởi tạo file `.sql` mới).
2. Chạy Clorinde CLI để tự động sinh code Rust tương ứng với SQL vừa viết:
   ```bash
   clorinde live -q ./crates/db/queries/ -d crates/clorinde "<YOUR_DATABASE_URL>"
   ```
3. Sau khi chạy lệnh trên, các hàm tương tác với DB đã sẵn sàng để bạn Import và sử dụng bên trong Axum handler ở thư mục `crates/web-server`.