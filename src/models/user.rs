use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::PgConnection;
// use uuid::Uuid;
//
use crate::errors::ServiceError;
use crate::schema::users::{self, dsl::users as usersTable};
use crate::utils::pass_hash::hash_password;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub hash_pass: String,
    pub created_at: NaiveDateTime, // Local::now().naive_local()
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub email: &'a str,
    pub hash_pass: String,
}
impl<'a> NewUser<'a> {
    pub fn from_credentials(email: &'a str, password: &'a str) -> Result<Self, ServiceError> {
        let hash_pass = hash_password(password)?;
        Ok(Self { email, hash_pass })
    }
    pub fn insert(self, db_conn: &PgConnection) -> Result<User, ServiceError> {
        let user: User = diesel::insert_into(usersTable)
            .values(&self)
            .get_result(db_conn)?;
        Ok(user.into())
    }
}
