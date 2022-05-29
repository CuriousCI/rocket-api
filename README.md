# 🚀 Rocket ⛽ diesel 📫️ AWS ses ~💸 API

Rocket.rs 0.5-rc template for building an API with email sending,

```bash
rustup toolchain install nightly
rustup override set nightly
cargo install diesel_cli --no-default-features --features "postgres"
```

```sql
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
```

```bash
diesel migration run
```
