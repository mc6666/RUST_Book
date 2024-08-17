#[derive(Debug, Insertable)]
#[diesel(table_name = self::schema::users)]
struct NewUser<'a> {
    id: &'a str,
    name: &'a str,
}

fn insert_new_user(
    conn: &mut SqliteConnection,
    user_name: String,
) -> diesel::QueryResult<User> {
    use crate::schema::users::dsl::*;

    // Create insertion model
    let uid = format!("{}", uuid::Uuid::new_v4());
    let new_user = NewUser {
        id: &uid,
        name: &user_name,
    };

    // normal diesel operations
    diesel::insert_into(users)
        .values(&new_user)
        .execute(conn)
        .expect("Error inserting person");

    let user = users
        .filter(id.eq(&uid))
        .first::<User>(conn)
        .expect("Error loading person that was just inserted");

    Ok(user)
}

