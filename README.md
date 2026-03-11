<div align="center">
  <h1>🦀 Modern Rust Microservice Template</h1>
  <p>
    <strong>High-performance, type-safe Web API built with <a href="https://github.com/tokio-rs/axum">Axum</a>.</strong>
  </p>
  <p>
    <img src="https://img.shields.io/badge/Rust-1.74+-orange.svg?style=flat-square" />
    <img src="https://img.shields.io/badge/Axum-0.8-blue.svg?style=flat-square" />
    <img src="https://img.shields.io/badge/PostgreSQL-15%2B-336791.svg?style=flat-square&logo=postgresql" />
    <img src="https://img.shields.io/badge/CI%2FCD-Dagger-yellow.svg?style=flat-square" />
  </p>
</div>

---

## Tech Stack

| Thành phần | Công cụ | Vai trò |
| :--- | :--- | :--- |
| HTTP Server | `axum` + `tokio` | Routing, middleware, async runtime |
| Database | `clorinde` + `deadpool-postgres` | SQL-first, sinh Rust code từ file `.sql` |
| API Docs | `utoipa` | Tự động sinh OpenAPI Spec + Swagger UI |
| Migration | `dbmate` | Tạo/sửa schema database |
| CI/CD | Dagger SDK (Rust) | Build pipeline viết bằng Rust |
| Code Quality | `pre-commit` + `cargo fmt` + `cargo clippy` | Tự động format và lint trước mỗi commit |
| Task Runner | `just` | Lệnh tắt thay thế `make` |

---

## Cấu trúc thư mục

```
.
├── crates/
│   ├── web-server/     # Entry point — routing, middleware, handlers, Swagger UI
│   ├── db/             # Kết nối DB (pool) + file migration + file SQL query
│   └── clorinde/       # ⚠️ Auto-generated — KHÔNG sửa tay
├── infrastructure/     # Dagger CI pipeline (Rust)
├── Justfile            # Lệnh tắt hàng ngày
├── .pre-commit-config  # Git hooks tự động format/lint
└── .env                # Database URL (không commit)
```

---

## Getting Started

**1. Cài pre-commit hook** (chạy một lần sau khi clone):
```bash
pre-commit install
```

**2. Thiết lập database:**
```bash
just dev-init      # Tạo cụm Kubernetes local (k3d)
just dev-setup     # Khởi động PostgreSQL trong K8S
just dev-secrets   # Tạo file .env với database credentials
dbmate up          # Chạy migration, tạo bảng
```

**3. Chạy server:**
```bash
just watch
```
Server khởi động tại `http://localhost:3000`. Tự động reload khi sửa code.

---

## API Documentation

Swagger UI tại: **[http://localhost:3000/swagger-ui/](http://localhost:3000/swagger-ui/)**

Endpoints hiện có: `Users` CRUD đầy đủ (GET / POST / PUT / DELETE).

---

## Thêm feature mới (SQL-First workflow)

```
1. Viết SQL query    →  crates/db/queries/<tên>.sql
2. Sinh Rust code    →  clorinde live -q ./crates/db/queries/ -d crates/clorinde
3. Dùng trong API    →  import từ clorinde::queries::<tên> trong web-server
```

Nếu cần thêm bảng mới, tạo migration trước:
```bash
dbmate new <tên_migration>   # tạo file SQL
dbmate up                    # chạy migration
```

---

## CI/CD

GitHub Actions tự động chạy khi push lên `main` hoặc tạo PR:
```bash
cargo run --release -p infrastructure -- build
```

Pipeline thực hiện theo thứ tự: migration → sinh clorinde code → compile → build Docker image.

Build thủ công local:
```bash
cargo run -p infrastructure -- build --web-tag docker-daemon:local/web:latest
```