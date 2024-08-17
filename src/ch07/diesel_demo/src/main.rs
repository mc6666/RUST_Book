mod schema;

use diesel;
use dotenv::dotenv;
use diesel::prelude::*;
use std::env;
use diesel::{Connection, ExpressionMethods, QueryDsl, RunQueryDsl, SqliteConnection};
use diesel::result::Error;
use crate::schema::human;

fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[derive(Queryable)]
pub struct Human {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub age: Option<i32>,
}

#[derive(Insertable)]
#[diesel(table_name = human)]
struct NewHuman<'a> {
    first_name: &'a str,
    last_name: &'a str,
    age: Option<i32>,
}

fn insert_into<'a>(conn: &mut SqliteConnection, 
    first_name: &'a str, last_name: &'a str, age: Option<i32>) -> Human {
    let new_human = NewHuman {
        first_name,
        last_name,
        age,
    };

    let _ = conn.transaction::<(), Error, _>(|conn| {
        diesel::insert_into(human::table).values(&new_human).execute(conn)
            .expect("Error inserting new human");
        // Err(Error::RollbackTransaction)
        Ok(())
    });

    human::table.order(human::id.desc()).first(conn).unwrap()
}

fn truncate(conn: &mut SqliteConnection) {
    let _ = conn.transaction::<(), Error, _>(|conn| {
        diesel::delete(human::table).execute(conn)?;
        Ok(())
    });
}

fn query_db(conn: &mut SqliteConnection) -> Human {
    use self::schema::human::dsl::human;
    // use self::schema::human::age;
    human.first(conn).expect("Error querying database")
    // human.first(conn).expect("Error querying database")
}

fn main() {
    // insert new row 
    let conn = &mut establish_connection();
    truncate(conn);
    let new_human = insert_into(conn, "John", "Doe", None);
    let new_human2 = insert_into(conn, "Michael", "Lin", Some(26));
    println!("New human inserted with ID: {} {}", new_human.id, new_human2.id);
    
    // query db
    let person = query_db(conn);
    // let person = human.filter(human::age.eq(25)).first(conn).expect("Error querying database");
    println!("ID: {}", person.id);
    println!("First Name: {}", person.first_name);
    println!("Last Name: {}", person.last_name);
    println!("Age: {:?}", person.age);
}