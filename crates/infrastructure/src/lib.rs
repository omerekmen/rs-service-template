pub mod db {
    #[cfg(feature = "db-pg")]
    pub mod pg {
        // sqlx Postgres impl placeholder
    }
    #[cfg(feature = "db-mssql")]
    pub mod mssql {
        // sqlx mssql or tiberius impl placeholder
    }
}

pub mod mq {
    #[cfg(feature = "mq-rabbitmq")]
    pub mod rabbitmq {
        // lapin adapter placeholder
    }
}
