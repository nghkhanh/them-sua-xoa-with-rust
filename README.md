<div align="center">
  <h1>🦀 Modern Rust Microservice Template</h1>
  <p>
    <strong>A high-performance, type-safe, and production-ready Web API built with <a href="https://github.com/tokio-rs/axum">Axum</a>.</strong>
  </p>
  <p>
    <img src="https://img.shields.io/badge/Rust-1.74+-orange.svg?style=flat-square" alt="Rust Version" />
    <img src="https://img.shields.io/badge/Axum-0.8-blue.svg?style=flat-square" alt="Axum Version" />
    <img src="https://img.shields.io/badge/PostgreSQL-15%2B-336791.svg?style=flat-square&logo=postgresql" alt="PostgreSQL" />
    <img src="https://img.shields.io/badge/CI%2FCD-Dagger-yellow.svg?style=flat-square" alt="Dagger CI" />
  </p>
</div>

<hr />

## 📖 Overview

Dự án này là một bộ khung hiện đại, dành cho việc xây dựng các ứng dụng Backend/Microservice hiệu suất cao bằng Rust.
Kiến trúc dự án hướng đến **Sự An Toàn Kiểu Dữ Liệu (Type Safety)** từ tầng lõi đến tận Database, kết hợp cùng tự động hoá CI/CD và tài liệu OpenAPI 100%.

### ✨ Core Features & Tech Stack

- 🚀 **Routing & HTTP Server:** `axum` (được bảo trợ bởi Tokio) - cực nhanh, ổn định.
- 🐘 **Database Connectivity:** Mô hình truy vấn SQL-first thông qua thư viện `clorinde`. Đảm bảo sinh code Rust chặt chẽ 1:1 từ file `.sql` gốc để tận dụng tối ưu sức mạnh của PostgreSQL.
- 📖 **API Documentation:** Tự động phát sinh OpenAPI Specs + Swagger UI từ comments và structs thông qua `utoipa`.
- 🛠️ **DevOps & CI/CD:**
  - Triển khai "Infrastructure as Code" ở thư mục `infrastructure/` bằng Rust qua framework **Dagger SDK**.
  - Tự động thay đổi lược đồ database bằng tool độc lập **dbmate**.
- 🛡️ **Code Quality:** Phên dậu **Pre-commit hooks** giúp format, xót lỗi qua `cargo fmt` và `cargo clippy` ở từng điểm chạm commit, đảm bảo kho code sạch sẽ tối đa.
- ⚙️ **Developer Experience (DX):** Thay thế các file bash scripts phức tạp bằng công cụ **Justfile**, tối giản chu trình khởi chạy.

---

## 🏗️ Project Architecture (Workspace)

Dự án phân rã logic hoạt động qua một `Cargo Workspace` gồm nhiều crates (module) chuyên biệt:

```plaintext
modern-rust/
├── crates/
│   ├── web-server/     # (Crate chính) Entry point, chứa Middleware, API Routing (Axum), OpenAPI Swagger.
│   ├── db/             # Kết nối Database (Pool, Transaction) sử dụng `deadpool-postgres` & chứa files `.sql`.
│   └── clorinde/       # (Auto-generated Crate) Tự động sinh mã logic Rust dựa trên các tệp `.sql` từ crate `db`.
│
├── infrastructure/     # (DevOps Crate) Chạy Pipelines Build/Test/Deploy cục bộ hoặc trên Cloud dùng Dagger SDK.
├── Tech-Stack.md       # Bảng phân tích chi tiết Tech Stack giữa Rust và Python.
├── .pre-commit-config  # Lưới chắn ngăn lỗi syntax trước khi đẩy code lên Git.
└── Justfile            # Các câu lệnh command runner hàng ngày.
```

---

## 🚀 Getting Started

### 1. Prerequisites (Yêu cầu Môi trường)

Đảm bảo bạn đã cài đặt đủ dàn công cụ sau:
- [Rust & Cargo](https://rustup.rs/) (Phiên bản Stable Mới Nhất)
- [Docker](https://www.docker.com/) hoặc colima để chạy Container
- [Just](https://github.com/casey/just) (Trình gọi lệnh thay thế `make`)
- [dbmate](https://github.com/amacneil/dbmate) (Để khởi chạy tính năng di trú lược đồ CSDL)
- [Pre-commit](https://pre-commit.com/) (Khoá chặt lỗi syntax cục bộ qua Git hook)

### 2. Initialization & Setup

Lần đầu clone dự án, hãy thiết lập khóa chặn pre-commit để tự động format code của bạn ngay khi gõ lệnh `git commit`:
```bash
pre-commit install
```

### 3. Database Bootstrap

Khởi chạy cơ sở dữ liệu và cấy các bảng dữ liệu gốc bằng `dbmate` (Thông qua Just):
```bash
# Lệnh 'dbmate up' sẽ tự động chèn database url thông qua config `.env` của thư mục hiện hành
dbmate up
```

### 4. Run Development Server

Việc duy nhất bạn cần làm mỗi sáng thức dậy:
```bash
just watch
```
*(Lệnh này sử dụng `cargo watch` để hot-reload lại Web Server - port mặc định: 3000 - bất cứ khi nào bạn nhấn nút Save trên file mã nguồn).*

---

## 📚 API Reference (Swagger UI)

Tài liệu tương tác với Backend tự động được sinh ra và cung cấp ở cấp độ trực quan nhất. Ngay sau khi server báo log màu xanh, hãy mở trình duyệt và trải nghiệm:

👉 **[http://localhost:3000/swagger-ui/](http://localhost:3000/swagger-ui/)**

Cung cấp sẵn bộ cắm `Users` (CRUD đầy đủ: Tạo mới, Lấy danh sách, Chỉnh sửa, và Xóa theo UUID).

---

## 💻 Development Workflow

**Cách thêm tính năng (API) mới truy vấn dữ liệu theo hệ tư tưởng SQL-First:**

1. Tạo một hàm hoặc câu lệnh viết bằng ngôn ngữ `SQL` thuần túy vào tệp con phù hợp trong thu mục `crates/db/queries/`.
2. Gọi công cụ `clorinde` tự phân tích (parse) cú pháp PostgreSQL đó thành Struct tĩnh trong Rust (`crates/clorinde`):
   ```bash
   clorinde live -q ./crates/db/queries/ -d crates/clorinde "<YOUR_DATABASE_URL>"
   ```
3. Lúc này, tại file Handler của `crates/web-server`, bạn chỉ việc Import hàm vừa được `clorinde` sinh ra, "bind" với Database Client đang chạy ngầm, và hứng kết quả đã được map kiểu vô cùng an toàn (Type-checked at compile time).

---

_Được duy trì và phát triển với tốc độ và sự bảo mật tối đa của Rust._ 🦀
