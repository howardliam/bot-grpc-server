use proto::tickets_service_server;
use sqlx::PgPool;
use tracing::info;

use crate::{models, utils::sqlx_error_to_tonic_status};

pub mod proto {
    tonic::include_proto!("tickets");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("tickets_descriptor");
}

impl From<models::tickets::TicketsSettings> for proto::TicketsSettings {
    fn from(value: models::tickets::TicketsSettings) -> Self {
        Self {
            guild_id: value.guild_id,
            enabled: value.enabled,
            channel_id: value.channel_id,
        }
    }
}

impl From<models::tickets::Ticket> for proto::Ticket {
    fn from(value: models::tickets::Ticket) -> Self {
        Self {
            id: value.id,
            guild_id: value.guild_id,
            author_id: value.author_id,
            title: value.title,
            info: value.info,
            created_at: value.created_at.and_utc().timestamp(),
        }
    }
}

impl From<&models::tickets::Ticket> for proto::Ticket {
    fn from(value: &models::tickets::Ticket) -> Self {
        Self {
            id: value.id,
            guild_id: value.guild_id,
            author_id: value.author_id,
            title: value.title.clone(),
            info: value.info.clone(),
            created_at: value.created_at.and_utc().timestamp(),
        }
    }
}

#[derive(Debug)]
pub struct TicketsService {
    pool: PgPool,
}

impl TicketsService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[tonic::async_trait]
impl tickets_service_server::TicketsService for TicketsService {
    async fn create_or_update_settings(
        &self,
        request: tonic::Request<proto::TicketsSettings>,
    ) -> Result<tonic::Response<()>, tonic::Status> {
        info!("handling `create_or_update_settings`");

        let settings = request.get_ref();

        let query = "INSERT INTO tickets_settings VALUES ($1, $2, $3) ON CONFLICT (guild_id) DO UPDATE SET enabled = $2, channel_id = $3";
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
        request: tonic::Request<proto::TicketsSettingsRequest>,
    ) -> Result<tonic::Response<proto::TicketsSettings>, tonic::Status> {
        info!("handling `get_settings`");

        let guild_id = request.get_ref().guild_id;

        let query = "SELECT * FROM tickets_settings WHERE guild_id = $1";
        let result = sqlx::query_as::<_, models::tickets::TicketsSettings>(query)
            .bind(guild_id)
            .fetch_one(&self.pool)
            .await;

        let settings = match result {
            Ok(settings) => settings,
            Err(error) => return Err(sqlx_error_to_tonic_status(&error)),
        };

        Ok(tonic::Response::new(settings.into()))
    }

    async fn create_ticket(
        &self,
        request: tonic::Request<proto::NewTicket>,
    ) -> Result<tonic::Response<()>, tonic::Status> {
        info!("handling `create_ticket`");

        let new_ticket = request.get_ref();

        let query = "INSERT INTO ticket (guild_id, author_id, title, info) VALUES ($1, $2, $3, $4)";
        let result = sqlx::query(query)
            .bind(new_ticket.guild_id)
            .bind(new_ticket.author_id)
            .bind(&new_ticket.title)
            .bind(&new_ticket.info)
            .execute(&self.pool)
            .await;

        match result {
            Ok(_) => {}
            Err(error) => return Err(sqlx_error_to_tonic_status(&error)),
        }

        Ok(tonic::Response::new(()))
    }

    async fn get_ticket(
        &self,
        request: tonic::Request<proto::TicketRequest>,
    ) -> Result<tonic::Response<proto::Ticket>, tonic::Status> {
        info!("handling `get_ticket`");

        let ticket_request = request.get_ref();

        let query =
            "SELECT * FROM ticket WHERE guild_id = $1 AND author_id = $2 ORDER BY created_at ASC";
        let result = sqlx::query_as::<_, models::tickets::Ticket>(query)
            .bind(ticket_request.guild_id)
            .bind(ticket_request.author_id)
            .fetch_one(&self.pool)
            .await;

        let ticket = match result {
            Ok(ticket) => ticket,
            Err(error) => return Err(sqlx_error_to_tonic_status(&error)),
        };

        Ok(tonic::Response::new(ticket.into()))
    }

    async fn get_tickets(
        &self,
        request: tonic::Request<proto::TicketRequest>,
    ) -> Result<tonic::Response<proto::Tickets>, tonic::Status> {
        info!("handling `get_tickets`");

        let ticket_request = request.get_ref();

        let query =
            "SELECT * FROM ticket WHERE guild_id = $1 AND author_id = $2 ORDER BY created_at ASC LIMIT 5";
        let result = sqlx::query_as::<_, models::tickets::Ticket>(query)
            .bind(ticket_request.guild_id)
            .bind(ticket_request.author_id)
            .fetch_all(&self.pool)
            .await;

        let tickets = match result {
            Ok(tickets) => tickets,
            Err(error) => return Err(sqlx_error_to_tonic_status(&error)),
        }
        .iter()
        .map(|ticket| proto::Ticket::from(ticket))
        .collect();

        Ok(tonic::Response::new(proto::Tickets { tickets }))
    }

    async fn delete_ticket(
        &self,
        request: tonic::Request<proto::TicketRequest>,
    ) -> Result<tonic::Response<()>, tonic::Status> {
        info!("handling `delete_ticket`");

        let ticket_request = request.get_ref();

        let query =
            "DELETE FROM ticket WHERE guild_id = $1 AND author_id = $2 AND created_at = (SELECT MAX (created_at) FROM ticket WHERE guild_id = $1 AND author_id = $2)";
        let result = sqlx::query(query)
            .bind(ticket_request.guild_id)
            .bind(ticket_request.author_id)
            .execute(&self.pool)
            .await;

        match result {
            Ok(_) => {}
            Err(error) => return Err(sqlx_error_to_tonic_status(&error)),
        }

        Ok(tonic::Response::new(()))
    }
}
