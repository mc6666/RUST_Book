use diesel::prelude::*;
use crate::schema::users;
use serde::{Deserialize, Serialize};

/// User details.
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[derive(Serialize)]
pub struct DataSource {
    pub data: Vec<User>,
}
