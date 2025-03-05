#[derive(sqlx::FromRow)]
pub struct AutomodSettings {
    pub guild_id: i64,
    pub autoban_enabled: bool,
    pub autoban_threshold: i32,
    pub autokick_enabled: bool,
    pub autokick_threshold: i32,
}

#[derive(sqlx::FromRow)]
pub struct Warn {
    pub id: i32,
    pub guild_id: i64,
    pub staff_member_id: i64,
    pub targer_user_id: i64,
    pub reason: String,
    pub created_at: chrono::NaiveDateTime,
}
