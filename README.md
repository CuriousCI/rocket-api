# rocket-api

Rocket.rs 0.5-rc template for building an API with email sending,

```bash
rustup toolchain install nightly
rustup override set nightly
cargo install diesel_cli --version "2.0.0-rc.0" --no-default-features --features "postgres"
```

```sql
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
```

```bash
diesel migration run
```
