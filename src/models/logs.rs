#[derive(sqlx::FromRow)]
pub struct LogsSettings {
    pub guild_id: i64,
    pub enabled: bool,
    pub channel_id: i64,
}
