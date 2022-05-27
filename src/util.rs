use chrono::Duration;
use r2d2::{PooledConnection, ManageConnection};
use rocket::{Rocket, Build};
use rocket_sync_db_pools::{r2d2, Error, Config, Poolable, PoolResult};
// use diesel::{r2d2::{R2D2Connection, ConnectionManager}, Connection, connection::SimpleConnection, PgConnection};

pub struct PgConnection(diesel::PgConnection);

impl Poolable for PgConnection{
	type Manager = ConnectionManager<PgConnection>;
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