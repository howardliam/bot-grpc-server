use dotenv::dotenv;
use services::{
    guild_service::{self, proto::guild_service_server::GuildServiceServer},
    logs_service::{self, proto::logs_service_server::LogsServiceServer},
    moderation_service::{self, proto::moderation_service_server::ModerationServiceServer},
    tickets_service::{self, proto::tickets_service_server::TicketsServiceServer},
};
use sqlx::postgres::PgPoolOptions;
use tonic::transport::Server;

mod models;
mod services;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    dotenv().ok();

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set.");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to connect to the database with provided DATABASE_URL.");

    let addr = "[::1]:50051".parse()?;

    let service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(guild_service::proto::FILE_DESCRIPTOR_SET)
        .register_encoded_file_descriptor_set(logs_service::proto::FILE_DESCRIPTOR_SET)
        .register_encoded_file_descriptor_set(moderation_service::proto::FILE_DESCRIPTOR_SET)
        .register_encoded_file_descriptor_set(tickets_service::proto::FILE_DESCRIPTOR_SET)
        .build_v1()?;

    let guild_service = GuildServiceServer::new(guild_service::GuildService::new(pool.clone()));
    let logs_service = LogsServiceServer::new(logs_service::LogsService::new(pool.clone()));
    let moderation_service =
        ModerationServiceServer::new(moderation_service::ModerationService::new(pool.clone()));
    let tickets_serivce =
        TicketsServiceServer::new(tickets_service::TicketsService::new(pool.clone()));

    Server::builder()
        .layer(tower_http::cors::CorsLayer::permissive())
        .add_service(service)
        .add_service(guild_service)
        .add_service(logs_service)
        .add_service(moderation_service)
        .add_service(tickets_serivce)
        .serve(addr)
        .await?;

    Ok(())
}
