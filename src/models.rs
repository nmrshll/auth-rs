use crate::schema::users;
use chrono::{Local, NaiveDateTime};
// use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "users"]
pub struct User {
    pub email: String,
    pub hash_pass: String,
    pub created_at: NaiveDateTime, // only NaiveDateTime works because of diesel limitations
}

impl User {
    pub fn from_details(email: String, hash_pass: String) -> Self {
        User {
            email,
            hash_pass,
            created_at: Local::now().naive_local(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SlimUser {
    pub email: String,
}

impl From<User> for SlimUser {
    fn from(user: User) -> Self {
        SlimUser { email: user.email }
    }
}
