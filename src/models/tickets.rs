#[derive(sqlx::FromRow)]
pub struct TicketsSettings {
    pub guild_id: i64,
    pub enabled: bool,
    pub channel_id: i64,
}

#[derive(sqlx::FromRow)]
pub struct Ticket {
    pub id: i32,
    pub guild_id: i64,
    pub author_id: i64,
    pub title: String,
    pub info: String,
    pub created_at: chrono::NaiveDateTime,
}
