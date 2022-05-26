use chrono::Duration;
use rocket::{Rocket, Build};
use rocket_sync_db_pools::{r2d2, Error, Config, Poolable, PoolResult};

struct PgConnection(diesel::PgConnection);

impl Poolable for PgConnection {
	type Manager= diesel::r2d2::ConnectionManager<PgConnection>;
    type Error = std::convert::Infallible;

	fn pool(db_name: &str, rocket: &Rocket<Build>) -> PoolResult<Self> {
		let config = Config::from(db_name, rocket)?;
        let manager = diesel::r2d2::ConnectionManager::new(&config.url);
        let pool = r2d2::Pool::builder()
            .max_size(config.pool_size)
            .connection_timeout(Duration::from_secs(config.timeout as u64))
            .build(manager)?;

        Ok(pool)
    }


}