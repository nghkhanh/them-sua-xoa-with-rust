# Getting Started

## Cấu trúc thư mục dự án

```
.
├── .devcontainer/          # Cấu hình môi trường dev (Docker)
├── .github/workflows/      # CI/CD GitHub Actions
├── crates/
│   ├── web-server/         # Axum HTTP server (main binary)
│   │   └── src/
│   │       ├── main.rs
│   │       ├── config.rs
│   │       └── errors.rs
│   ├── db/                 # Database logic
│   │   ├── migrations/     # File SQL migration (dbmate)
│   │   ├── queries/        # File SQL query (clorinde đọc từ đây)
│   │   └── src/lib.rs
│   ├── clorinde/           # Code Rust được sinh tự động từ SQL (KHÔNG sửa tay)
│   └── infrastructure/     # Dagger CI pipeline (Rust)
├── infra-as-code/
│   └── stack.yaml          # Khai báo StackApp cho Kubernetes
├── Cargo.toml              # Workspace
└── Justfile                # Lệnh tắt
```

---

## Khởi động môi trường

| Lệnh | Mô tả |
| :--- | :--- |
| `just dev-init` | Xoá và tạo lại cụm k3d-nails, mở port 30000-30001 |
| `just dev-setup` | Cài CloudNativePG Operator, khởi động PostgreSQL trong K8S |
| `just dev-secrets` | Trích xuất credentials từ K8S, ghi vào `.env` |
| `just watch` | Chạy server với hot-reload (dùng `mold` + `cargo watch`) |

---

## Database

| Lệnh | Mô tả |
| :--- | :--- |
| `dbmate new <tên>` | Tạo file SQL mới trong `crates/db/migrations/` |
| `dbmate status` | Xem migration nào đã/chưa chạy |
| `dbmate up` | Chạy tất cả migration chưa chạy |
| `dbmate down` | Rollback migration gần nhất |
| `clorinde live -q ./crates/db/queries/ -d crates/clorinde` | Đọc file `.sql` trong `queries/`, sinh ra Rust functions vào `crates/clorinde/` |

---

## Workflow thêm feature mới

Mỗi khi thêm bảng mới hoặc thêm query mới, làm theo thứ tự:

```
1. Viết migration       →  dbmate new <tên>
                            # Sửa file SQL trong crates/db/migrations/

2. Chạy migration       →  dbmate up

3. Viết SQL query       →  # Thêm/sửa file trong crates/db/queries/*.sql

4. Sinh Rust code       →  clorinde live -q ./crates/db/queries/ -d crates/clorinde

5. Dùng trong API       →  # Import từ clorinde::queries::<tên> trong web-server
```

---

## Deploy lên production (K8S + Cloudflare)

| Lệnh | Mô tả |
| :--- | :--- |
| `cargo run --bin stack-cli -- init` | Cài operators (CloudNativePG, Keycloak, ingress) lên K8S |
| `cargo run --bin stack-cli -- install --manifest infra-as-code/stack.yaml` | Apply StackApp manifest, tạo namespace + database + deployment |
| `cargo run --bin stack-cli -- operator` | Reconcile lại toàn bộ StackApp resources |
| `stack cloudflare --manifest infra-as-code/stack.yaml --ingress-target <url>` | Mở Cloudflare Tunnel, expose app ra Internet |
| `kubectl logs -n stack-demo deployment/cloudflared -f` | Xem log để lấy link public `*.trycloudflare.com` |

---

## CI/CD (Dagger + GitHub Actions)

**Build và test local:**

| Lệnh | Mô tả |
| :--- | :--- |
| `cargo run --release -p infrastructure -- build` | Chạy toàn bộ pipeline: compile, migration, sinh clorinde code, build container |
| `cargo run -p infrastructure -- build --web-tag docker-daemon:local/web:latest` | Build + export Docker image ra local daemon |
| `cargo run -p infrastructure -- build --migrations-tag docker-daemon:local/dbmate:latest` | Build + export migration container |

**GitHub Actions** tự động trigger khi push lên `main` hoặc tạo PR, chạy:
```
cargo run --release -p infrastructure -- build
```

Pipeline Dagger (`crates/infrastructure/`) làm theo thứ tự:
1. Khởi động PostgreSQL service tạm
2. Chạy migration
3. Sinh clorinde code
4. Compile web-server (target `x86_64-unknown-linux-musl`)
5. Build Docker image tối giản (không có Rust toolchain)

---

## Kiểm tra nhanh

| Lệnh | Mô tả |
| :--- | :--- |
| `cargo test -- --nocapture` | Chạy toàn bộ test, in output |
| `kubectl get pods -A` | Xem trạng thái tất cả services trong K8S |
| `k9s` | Dashboard terminal cho Kubernetes |
| `db` | Mở psql kết nối trực tiếp vào database |

---
