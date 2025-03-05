use proto::guild_service_server;
use sqlx::PgPool;
use tracing::info;

use crate::utils::sqlx_error_to_tonic_status;

pub mod proto {
    tonic::include_proto!("guild");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("guild_descriptor");
}

#[derive(Debug)]
pub struct GuildService {
    pool: PgPool,
}

impl GuildService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[tonic::async_trait]
impl guild_service_server::GuildService for GuildService {
    async fn create_guild(
        &self,
        request: tonic::Request<proto::Guild>,
    ) -> Result<tonic::Response<()>, tonic::Status> {
        info!("handling `create_guild`");

        let guild_id = request.get_ref().guild_id;

        let query = "INSERT INTO guild VALUES ($1) ON CONFLICT DO NOTHING";
        let result = sqlx::query(query).bind(guild_id).execute(&self.pool).await;

        match result {
            Ok(_) => {}
            Err(error) => return Err(sqlx_error_to_tonic_status(&error)),
        }

        Ok(tonic::Response::new(()))
    }

    async fn delete_guild(
        &self,
        request: tonic::Request<proto::Guild>,
    ) -> Result<tonic::Response<()>, tonic::Status> {
        info!("handling `delete_guild`");

        let guild_id = request.get_ref().guild_id;

        let query = "DELETE FROM guild WHERE guild_id = $1";
        let result = sqlx::query(query).bind(guild_id).execute(&self.pool).await;

        match result {
            Ok(_) => {}
            Err(error) => return Err(sqlx_error_to_tonic_status(&error)),
        }

        Ok(tonic::Response::new(()))
    }
}
