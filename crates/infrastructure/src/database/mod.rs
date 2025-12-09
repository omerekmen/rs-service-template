pub mod postgres;

pub enum DbPoolType {
    Postgres,
}

impl DbPoolType {
    pub fn as_str(&self) -> &str {
        match self {
            DbPoolType::Postgres => "postgresql",
        }
    }
}

pub enum DbChannels {
    AuthDb,
    LogDb,
    AnalyticsDb,
}

impl DbChannels {
    pub fn as_str(&self) -> &str {
        match self {
            DbChannels::AuthDb => "auth_db",
            DbChannels::LogDb => "log_db",
            DbChannels::AnalyticsDb => "analytics_db",
        }
    }
}
