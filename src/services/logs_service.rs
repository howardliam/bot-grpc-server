use proto::logs_service_server;
use sqlx::PgPool;
use tracing::info;

use crate::{models, utils::sqlx_error_to_tonic_status};

pub mod proto {
    tonic::include_proto!("logs");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("logs_descriptor");
}

impl From<models::logs::LogsSettings> for proto::LogsSettings {
    fn from(value: models::logs::LogsSettings) -> Self {
        Self {
            guild_id: value.guild_id,
            enabled: value.enabled,
            channel_id: value.channel_id,
        }
    }
}

#[derive(Debug)]
pub struct LogsService {
    pool: PgPool,
}

impl LogsService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[tonic::async_trait]
impl logs_service_server::LogsService for LogsService {
    async fn create_or_update_settings(
        &self,
        request: tonic::Request<proto::LogsSettings>,
    ) -> Result<tonic::Response<()>, tonic::Status> {
        info!("handling `create_or_update_settings`");

        let settings = request.get_ref();

        let query = "INSERT INTO logs_settings VALUES ($1, $2, $3) ON CONFLICT (guild_id) DO UPDATE SET enabled = $2, channel_id = $3";
        let result = sqlx::query(query)
            .bind(settings.guild_id)
            .bind(settings.enabled)
            .bind(settings.channel_id)
            .execute(&self.pool)
            .await;

        match result {
            Ok(_) => {}
            Err(error) => return Err(sqlx_error_to_tonic_status(&error)),
        }

        Ok(tonic::Response::new(()))
    }

    async fn get_settings(
        &self,
        request: tonic::Request<proto::LogsSettingsRequest>,
    ) -> Result<tonic::Response<proto::LogsSettings>, tonic::Status> {
        info!("handling `get_settings`");

        let guild_id = request.get_ref().guild_id;

        let query = "SELECT * FROM logs_settings WHERE guild_id = $1";
        let result = sqlx::query_as::<_, models::logs::LogsSettings>(query)
            .bind(guild_id)
            .fetch_one(&self.pool)
            .await;

        let settings = match result {
            Ok(settings) => settings,
            Err(error) => return Err(sqlx_error_to_tonic_status(&error)),
        };

        Ok(tonic::Response::new(settings.into()))
    }
}
