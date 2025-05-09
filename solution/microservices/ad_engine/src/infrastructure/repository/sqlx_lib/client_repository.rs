use async_trait::async_trait;

use crate::{domain, infrastructure};

#[derive(Debug)]
pub struct PgClientRepository<'p> {
    db_pool: &'p sqlx::Pool<sqlx::Postgres>,
}

impl<'p> infrastructure::repository::IRepo<'p> for PgClientRepository<'p> {
    fn new(db_pool: &'p sqlx::Pool<sqlx::Postgres>) -> Self {
        Self { db_pool }
    }
}

#[derive(Debug)]
pub struct ClientReturningSchema {
    pub client_id: uuid::Uuid,
    pub login: String,
    pub age: i32,
    pub location: String,
    pub gender: String,
}

#[async_trait]
impl<'p> domain::services::repository::IRegisterBulkClient for PgClientRepository<'p> {
    async fn register(
        &self,
        client_ids: Vec<uuid::Uuid>,
        logins: Vec<String>,
        locations: Vec<String>,
        genders: Vec<String>,
        ages: Vec<i32>,
    ) -> infrastructure::repository::RepoResult<Vec<infrastructure::repository::sqlx_lib::ClientReturningSchema>> {
        let mut transaction = self.db_pool.begin().await?;

        let clients = sqlx::query_as!(
            ClientReturningSchema,
            r#"
            INSERT INTO clients (id, login, location, gender, age)
            SELECT * FROM UNNEST($1::UUID[], $2::VARCHAR[], $3::VARCHAR[], $4::VARCHAR[], $5::INT[])
            ON CONFLICT (id)
            DO UPDATE SET
                login = EXCLUDED.login,
                location = EXCLUDED.location,
                gender = EXCLUDED.gender,
                age = EXCLUDED.age
            RETURNING id AS client_id, login, age, location, gender;
            "#,
            &client_ids,
            &logins,
            &locations,
            &genders,
            &ages
        )
        .fetch_all(&mut *transaction)
        .await?;

        transaction.commit().await?;

        Ok(clients)
    }
}

#[async_trait]
impl<'p> domain::services::repository::IGetClientById for PgClientRepository<'p> {
    async fn get_by_id(
        &self,
        client_id: uuid::Uuid,
    ) -> infrastructure::repository::RepoResult<infrastructure::repository::sqlx_lib::ClientReturningSchema> {
        let client = sqlx::query_as!(
            ClientReturningSchema,
            r#"
            SELECT id AS client_id, login, age, location, gender
            FROM clients
            WHERE id = $1
            "#,
            client_id
        )
        .fetch_one(self.db_pool)
        .await?;

        Ok(client)
    }
}

/// Implements conversion from repository user schema to domain user profile
/// schema.
impl From<infrastructure::repository::sqlx_lib::ClientReturningSchema> for domain::schemas::ClientProfileSchema {
    fn from(user: infrastructure::repository::sqlx_lib::ClientReturningSchema) -> Self {
        Self {
            client_id: user.client_id,
            login: user.login,
            location: user.location,
            gender: user.gender,
            age: user.age as u8,
        }
    }
}
