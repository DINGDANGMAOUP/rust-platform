use std::sync::Arc;
use platform_app::{config, controller::test_controller::{echo, hello, index, manual_hello}, domain, controller::user_controller};
use actix_files as fs;
use actix_web::{ middleware::{Logger,Compress}, web, App, HttpServer};
use platform_app::middleware::filter::test_filter::SayHi;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = config::config::SystemConfig::default();
    config::log::init_log(&config);
    let rb = config::db::init_db(&config).await;
    domain::table::table_init::sync_tables(&rb).await;
    domain::table::table_init::sync_tables_data(&rb).await;
    let url = &config.server.host;
    let port = &config.server.port;
    let server_url = format!("{}:{}", url, port);
   
    HttpServer::new(move || {
        App::new()
        .wrap(SayHi)
            .app_data(web::Data::new(Arc::new(config.clone())))
            .app_data(web::Data::new(Arc::new(rb.clone())))
            .wrap(Compress::default())
            .service(hello)
            .service(echo)
            // .service(fs::Files::new("/resource",".").show_files_listing().use_last_modified(true))
            .service( fs::Files::new("/resource","./resource").index_file("index.html"))
            .service(web::scope("/user").route("",web::post().to(user_controller::create)))
            .route("/index", web::post().to(index))
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(&server_url)?
    .run()
    .await
}
