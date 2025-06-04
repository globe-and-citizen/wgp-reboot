mod proxy;
mod config;
mod router;
mod message;

use std::env;
use clap::Parser;
use env_logger;
use chrono::Local;
use log::*;
use std::fs::OpenOptions;
use std::io::Write;

use pingora::server::configuration::Opt;
use pingora::server::Server;
use crate::message::handler::WGPMessageHandler;

fn log_init(filepath: &String, level: &LevelFilter) {
    let mut target = env_logger::Target::Stdout;

    if filepath != "console" {
        let file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(filepath)
            .expect("Can't create file!");

        target = env_logger::Target::Pipe(Box::new(file));
    }

    env_logger::Builder::new()
        .target(target)
        .filter(None, *level)
        .format(|buf, record| {
            writeln!(
                buf,
                "[{} {} {}:{}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
                record.level(),
                // record.file().and_then(|f| f.rsplit_once('/').map(|(_, file)| file)).unwrap_or("unknown"),
                record.file().map(|f| {
                    let parts: Vec<&str> = f.split('/').collect();
                    if parts.len() > 6 {
                        parts[parts.len() - 6..].join("/")
                    } else {
                        f.to_string()
                    }
                }).unwrap_or("unknown".to_string()),
                record.line().unwrap_or(0),
                record.args()
            )
        })
        .init();
}

// RUST_LOG=INFO cargo run proxy
// curl 127.0.0.1:6191
fn main() {
    let config_path = env::var("CONFIG_PATH").unwrap_or_else(|_| "config.toml".to_string());

    let wgp_config = config::Config::from_file(&config_path);
    wgp_config.validate();
    println!("{:?}", wgp_config);

    log_init(&wgp_config.log.path, &wgp_config.log.to_level_filter());

    let msg_handler = WGPMessageHandler::new(wgp_config.handler);
    let mut router = router::Router::new(msg_handler);
    router.post("/login".to_string(), Box::new([WGPMessageHandler::handle_login]));
    router.post("/register".to_string(), Box::new([WGPMessageHandler::handle_register]));
    router.get("/profile".to_string(), Box::new([WGPMessageHandler::authentication_middleware, WGPMessageHandler::get_profile]));
    router.get("/poems?id={}".to_string(), Box::new([WGPMessageHandler::authentication_middleware, WGPMessageHandler::get_poems, WGPMessageHandler::ntor_encrypt]));
    router.get("/images?id={}".to_string(), Box::new([WGPMessageHandler::authentication_middleware, WGPMessageHandler::get_images]));
    router.post("/ntor_init".to_string(), Box::new([WGPMessageHandler::ntor_init]));

    let handler = proxy::handler::ProxyHandler::new(router);

    let opt = Opt::parse();
    let mut my_server = Server::new(Some(opt)).unwrap();
    my_server.bootstrap();

    let mut my_proxy = pingora::proxy::http_proxy_service(
        &my_server.configuration,
        proxy::Proxy::new(wgp_config.upstream.host, wgp_config.upstream.port, handler),
    );

    my_proxy.add_tcp(wgp_config.server.address.as_str());
    my_server.add_service(my_proxy);
    my_server.run_forever();
}
