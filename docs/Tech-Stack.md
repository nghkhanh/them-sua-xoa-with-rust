# Tech Stack

---

## 1. Core Architecture

| Nhiệm vụ | Rust | Python (2026) |
| :--- | :--- | :--- |
| **Web Framework** — Nhận request, định tuyến | **Axum** — DX tốt, type-safe | **FastAPI** — Chuẩn |
| **Async Runtime** — Chạy đa nhiệm I/O | **Tokio** — Chuẩn vàng của Rust | **asyncio** + **uvloop** — uvloop tối ưu tốc độ I/O |
| **Tương tác CSDL** — Query an toàn | **Clorinde** — Database-first, sinh code từ SQL | **SQLModel** — Code-first, gộp chung Validator & ORM; hoặc **SQLc** — Database-first như Clorinde |
| **Tài liệu API** — Docs cho Frontend/Test | **Utoipa** — Macro sinh Swagger UI | Tích hợp tự động 100% qua Pydantic |
| **Migration DB** — Tạo/sửa bảng | **Dbmate** — Công cụ độc lập | **Alembic** hoặc giữ nguyên **Dbmate** |
| **CLI App** — Lệnh chạy Terminal | **Clap** — Ergonomic cao | **Typer** — Đồng bộ type-hints với FastAPI |

---

## 2. Production-Ready

| Nhiệm vụ | Rust | Python (2026) |
| :--- | :--- | :--- |
| **Cache** — Tránh sập DB do đọc nhiều | `redis-rs` + pool `deadpool-redis` | `redis-py` (async) hoặc `FastAPI-Cache` |
| **Background Jobs** — Đẩy tác vụ nặng ra nền | `tokio::spawn` / **Fang** | **Celery** — Phổ biến nhưng nặng; **TaskIQ** — Nhẹ hơn, hợp async |
| **Logging & Tracing** — Theo dõi lỗi và hiệu năng | `tracing` + OpenTelemetry SDK | **Loguru** — In log; OpenTelemetry SDK — Trace |
| **Auth** — Quản lý user, JWT, hash password | `jsonwebtoken` + `argon2` | `PyJWT` + `Passlib` hoặc **FastAPI-Users** |
| **Testing** — Tự động kiểm thử | `cargo test` + `rstest` | **pytest** + `FastAPI TestClient` |

---

## 3. Quản Lý Mã & DevOps

Xu hướng nổi bật của Python 2026: dùng **công cụ viết bằng Rust** để tăng tốc hệ thống nội bộ.

| Nhiệm vụ | Rust | Python (2026) |
| :--- | :--- | :--- |
| **Infrastructure as Code** — CI/CD bằng code | **Dagger SDK (Rust)** | **Dagger SDK (Python)** — `dagger-io` |
| **Quản lý gói** — Cài thư viện nhanh | `cargo` — Mặc định | **uv** — Thay pip, viết bằng Rust, siêu nhanh |
| **Pre-commit hooks** — Kiểm tra trước khi push | `.pre-commit-config` | `.pre-commit-config` |
| **Format & Lint** — Làm sạch code | `cargo fmt` + `cargo clippy` | **Ruff** — Thay Flake8/Black, viết bằng Rust |
| **Parse & Validate Data** | `serde` | **Pydantic V2** — Lõi `pydantic-core` viết bằng Rust |