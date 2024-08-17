use diesel::prelude::*;
use chrono::prelude::*;
use chrono::{DateTime, Local, NaiveDate};

#[derive(Insertable, Queryable, Selectable, PartialEq, Debug)]
#[diesel(table_name = crate::schema::orders)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Order {
    pub order_id: i16,
    pub customer_id: Option<String>,
    pub employee_id: Option<i16>,
    pub order_date: Option<NaiveDate>,
    pub required_date: Option<NaiveDate>,
    pub shipped_date: Option<NaiveDate>,
    pub ship_via: Option<i16>,
    pub freight: Option<f32>,
    pub ship_name: Option<String>,
    pub ship_address: Option<String>,
    pub ship_city: Option<String>,
    pub ship_region: Option<String>,
    pub ship_postal_code: Option<String>,
    pub ship_country: Option<String>,
}

#[derive(Insertable, Queryable, Selectable, Associations, Debug, PartialEq)]
#[diesel(belongs_to(Order))]
#[diesel(table_name = crate::schema::order_details)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct OrderDetail {
    pub order_id: i16,
    pub product_id: i16,
    pub unit_price: f32,
    pub quantity: i16,
    pub discount: f32,
}
