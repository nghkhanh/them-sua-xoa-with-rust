# Troubleshooting Notes

Ghi chú các lỗi đã gặp và cách khắc phục.

---

## Mục lục

- [Lỗi 500: db error tại endpoint /users](#lỗi-500-db-error-tại-endpoint-users)
- [Lỗi: pq: permission denied for schema public](#lỗi-pq-permission-denied-for-schema-public)

---

## Lỗi 500: db error tại endpoint /users

**Triệu chứng:** `500 Internal Server Error` với nội dung `db error` khi gọi GET `/users` lúc dev local.

**Nguyên nhân:** Database đang chạy nhưng schema/bảng chưa được tạo. Axum xử lý lỗi Postgres một cách chung chung, không log chi tiết lỗi SQL (ví dụ: `relation "auth.users" does not exist`).

**Cách khắc phục:** Chạy migration bằng `dbmate`:

1. Lấy database URL từ file `.env` (thường là `MIGRATIONS_URL` hoặc `DATABASE_URL`)
2. Chạy lệnh:

```bash
dbmate --url "postgres://db-owner:testpassword@host.docker.internal:30001/stack-app?sslmode=disable" \
       --migrations-dir crates/db/migrations up
```

> **Tip:** Thêm recipe vào `Justfile` để chạy nhanh sau này.

**Lưu ý:** Nếu `dbmate up` báo `duplicate key value violates unique constraint "users_external_id_key"` — script đã chạy 2 lần liên tiếp, bảng đã được tạo thành công, bỏ qua lỗi này an toàn.

---

## Lỗi: pq: permission denied for schema public

**Triệu chứng:** Chạy `dbmate up` gặp `Error: pq: permission denied for schema public`.

**Nguyên nhân:** Từ PostgreSQL 15 trở đi, user thông thường không còn mặc định có quyền `CREATE` trên schema `public`. `dbmate` cần tạo bảng `schema_migrations` ở đó để theo dõi tiến độ migration.

**Cách khắc phục — Cách 1:** Cấp quyền cho user (cần chạy với superuser):

```sql
GRANT ALL ON SCHEMA public TO "db-owner";
GRANT ALL ON SCHEMA public TO "application_user";
```

**Cách khắc phục — Cách 2:** Chỉ định bảng tracking trong schema khác mà user đã có quyền:

```bash
dbmate --migrations-table "auth.schema_migrations" up
```

---

<!-- THÊM LỖI MỚI BÊN DƯỚI THEO TEMPLATE SAU -->
<!--
## Lỗi: <tên lỗi>

**Triệu chứng:** ...

**Nguyên nhân:** ...

**Cách khắc phục:** ...

-->