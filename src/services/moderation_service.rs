use proto::moderation_service_server;
use sqlx::PgPool;
use tracing::info;

use crate::{models, utils::sqlx_error_to_tonic_status};

pub mod proto {
    tonic::include_proto!("moderation");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("moderation_descriptor");
}

impl From<models::moderation::AutomodSettings> for proto::AutomodSettings {
    fn from(value: models::moderation::AutomodSettings) -> Self {
        Self {
            guild_id: value.guild_id,
            autoban_enabled: value.autoban_enabled,
            autoban_threshold: value.autoban_threshold,
            autokick_enabled: value.autokick_enabled,
            autokick_threshold: value.autokick_threshold,
        }
    }
}

impl From<models::moderation::Warn> for proto::Warn {
    fn from(value: models::moderation::Warn) -> Self {
        Self {
            id: value.id,
            guild_id: value.guild_id,
            staff_member_id: value.staff_member_id,
            target_user_id: value.targer_user_id,
            reason: value.reason,
            created_at: value.created_at.and_utc().timestamp(),
        }
    }
}

impl From<&models::moderation::Warn> for proto::Warn {
    fn from(value: &models::moderation::Warn) -> Self {
        Self {
            id: value.id,
            guild_id: value.guild_id,
            staff_member_id: value.staff_member_id,
            target_user_id: value.targer_user_id,
            reason: value.reason.clone(),
            created_at: value.created_at.and_utc().timestamp(),
        }
    }
}

#[derive(Debug)]
pub struct ModerationService {
    pool: PgPool,
}

impl ModerationService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[tonic::async_trait]
impl moderation_service_server::ModerationService for ModerationService {
    async fn create_or_update_settings(
        &self,
        request: tonic::Request<proto::AutomodSettings>,
    ) -> Result<tonic::Response<()>, tonic::Status> {
        info!("handling `create_or_update_settings`");

        let settings = request.get_ref();

        let query =
            "INSERT INTO automod_settings VALUES ($1, $2, $3, $4, $5) ON CONFLICT (guild_id) \
            DO UPDATE SET autoban_enabled = $2, autoban_threshold = $3, autokick_enabled = $4, autokick_threshold = $5";
        let result = sqlx::query(query)
            .bind(settings.guild_id)
            .bind(settings.autoban_enabled)
            .bind(settings.autoban_threshold)
            .bind(settings.autokick_enabled)
            .bind(settings.autokick_threshold)
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
        request: tonic::Request<proto::AutomodSettingsRequest>,
    ) -> Result<tonic::Response<proto::AutomodSettings>, tonic::Status> {
        info!("handling `get_settings`");

        let guild_id = request.get_ref().guild_id;

        let query = "SELECT * FROM automod_settings WHERE guild_id = $1";
        let result = sqlx::query_as::<_, models::moderation::AutomodSettings>(query)
            .bind(guild_id)
            .fetch_one(&self.pool)
            .await;

        let settings = match result {
            Ok(settings) => settings,
            Err(error) => return Err(sqlx_error_to_tonic_status(&error)),
        };

        Ok(tonic::Response::new(settings.into()))
    }

    async fn create_warn(
        &self,
        request: tonic::Request<proto::NewWarn>,
    ) -> Result<tonic::Response<()>, tonic::Status> {
        info!("handling `create_warn`");

        let new_ticket = request.get_ref();

        let query = "INSERT INTO warn (guild_id, staff_member_id, target_user_id, reason) VALUES ($1, $2, $3, $4)";
        let result = sqlx::query(query)
            .bind(new_ticket.guild_id)
            .bind(new_ticket.staff_member_id)
            .bind(new_ticket.target_user_id)
            .bind(&new_ticket.reason)
            .execute(&self.pool)
            .await;

        match result {
            Ok(_) => {}
            Err(error) => return Err(sqlx_error_to_tonic_status(&error)),
        }

        Ok(tonic::Response::new(()))
    }

    async fn get_warn(
        &self,
        request: tonic::Request<proto::WarnRequest>,
    ) -> Result<tonic::Response<proto::Warn>, tonic::Status> {
        info!("handling `get_warn`");

        let warn_request = request.get_ref();

        let query =
            "SELECT * FROM warn WHERE guild_id = $1 AND target_user_id = $2 ORDER BY created_at ASC";
        let result = sqlx::query_as::<_, models::moderation::Warn>(query)
            .bind(warn_request.guild_id)
            .bind(warn_request.target_user_id)
            .fetch_one(&self.pool)
            .await;

        let warn = match result {
            Ok(warn) => warn,
            Err(error) => return Err(sqlx_error_to_tonic_status(&error)),
        };

        Ok(tonic::Response::new(warn.into()))
    }

    async fn get_warns(
        &self,
        request: tonic::Request<proto::WarnRequest>,
    ) -> Result<tonic::Response<proto::Warns>, tonic::Status> {
        info!("handling `get_warns`");

        let warn_request = request.get_ref();

        let query =
            "SELECT * FROM warn WHERE guild_id = $1 AND target_user_id = $2 ORDER BY created_at ASC LIMIT 5";
        let result = sqlx::query_as::<_, models::moderation::Warn>(query)
            .bind(warn_request.guild_id)
            .bind(warn_request.target_user_id)
            .fetch_all(&self.pool)
            .await;

        let warns = match result {
            Ok(warns) => warns,
            Err(error) => return Err(sqlx_error_to_tonic_status(&error)),
        }
        .iter()
        .map(|warn| warn.into())
        .collect();

        Ok(tonic::Response::new(proto::Warns { warns }))
    }

    async fn delete_warn(
        &self,
        request: tonic::Request<proto::WarnRequest>,
    ) -> Result<tonic::Response<()>, tonic::Status> {
        info!("handling `delete_warn`");

        let warn_request = request.get_ref();

        let query =
            "DELETE FROM warn WHERE guild_id = $1 AND target_user_id = $2 AND created_at = (SELECT MAX (created_at) FROM warn WHERE guild_id = $1 AND target_user_id = $2)";
        let result = sqlx::query(query)
            .bind(warn_request.guild_id)
            .bind(warn_request.target_user_id)
            .execute(&self.pool)
            .await;

        match result {
            Ok(_) => {}
            Err(error) => return Err(sqlx_error_to_tonic_status(&error)),
        }

        Ok(tonic::Response::new(()))
    }
}
