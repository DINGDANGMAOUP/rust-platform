use std::{sync::Arc, time::Duration};

use rbatis::RBatis;
use crate::{
    config::config::SystemConfig, domain::plugins::{logic_del_plugin::LogicDelPlugin,returning_id_plugin::ReturningIdPlugin,postgres_page_plugin::PostgresPagePlugin},
};

pub async fn init_db(config: &SystemConfig) -> RBatis {
    let rb = RBatis::new();
    rb.link(rbdc_pg::driver::PgDriver {}, &config.app.datasource.url)
        .await
        .unwrap();
    rb.intercepts.push(Arc::new(ReturningIdPlugin {}));
    rb.intercepts.push(Arc::new(PostgresPagePlugin {}));
    // rb.intercepts.push(Arc::new(LogicDelPlugin {}));
    let pool = rb.get_pool().unwrap();
    //max connections
    pool.set_max_open_conns(config.app.datasource.db_pool_len as u64)
        .await;
    //max timeout
    pool.set_timeout(Some(Duration::from_secs(
        config.app.datasource.db_pool_timeout as u64,
    )))
    .await;
    rb
}
