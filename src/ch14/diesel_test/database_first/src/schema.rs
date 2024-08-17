// @generated automatically by Diesel CLI.

diesel::table! {
    categories (category_id) {
        category_id -> Int2,
        #[max_length = 15]
        category_name -> Varchar,
        description -> Nullable<Text>,
        picture -> Nullable<Bytea>,
    }
}

diesel::table! {
    customer_customer_demo (customer_id, customer_type_id) {
        #[max_length = 5]
        customer_id -> Varchar,
        #[max_length = 5]
        customer_type_id -> Varchar,
    }
}

diesel::table! {
    customer_demographics (customer_type_id) {
        #[max_length = 5]
        customer_type_id -> Varchar,
        customer_desc -> Nullable<Text>,
    }
}

diesel::table! {
    customers (customer_id) {
        #[max_length = 5]
        customer_id -> Varchar,
        #[max_length = 40]
        company_name -> Varchar,
        #[max_length = 30]
        contact_name -> Nullable<Varchar>,
        #[max_length = 30]
        contact_title -> Nullable<Varchar>,
        #[max_length = 60]
        address -> Nullable<Varchar>,
        #[max_length = 15]
        city -> Nullable<Varchar>,
        #[max_length = 15]
        region -> Nullable<Varchar>,
        #[max_length = 10]
        postal_code -> Nullable<Varchar>,
        #[max_length = 15]
        country -> Nullable<Varchar>,
        #[max_length = 24]
        phone -> Nullable<Varchar>,
        #[max_length = 24]
        fax -> Nullable<Varchar>,
    }
}

diesel::table! {
    employee_territories (employee_id, territory_id) {
        employee_id -> Int2,
        #[max_length = 20]
        territory_id -> Varchar,
    }
}

diesel::table! {
    employees (employee_id) {
        employee_id -> Int2,
        #[max_length = 20]
        last_name -> Varchar,
        #[max_length = 10]
        first_name -> Varchar,
        #[max_length = 30]
        title -> Nullable<Varchar>,
        #[max_length = 25]
        title_of_courtesy -> Nullable<Varchar>,
        birth_date -> Nullable<Date>,
        hire_date -> Nullable<Date>,
        #[max_length = 60]
        address -> Nullable<Varchar>,
        #[max_length = 15]
        city -> Nullable<Varchar>,
        #[max_length = 15]
        region -> Nullable<Varchar>,
        #[max_length = 10]
        postal_code -> Nullable<Varchar>,
        #[max_length = 15]
        country -> Nullable<Varchar>,
        #[max_length = 24]
        home_phone -> Nullable<Varchar>,
        #[max_length = 4]
        extension -> Nullable<Varchar>,
        photo -> Nullable<Bytea>,
        notes -> Nullable<Text>,
        reports_to -> Nullable<Int2>,
        #[max_length = 255]
        photo_path -> Nullable<Varchar>,
    }
}

diesel::table! {
    order_details (order_id, product_id) {
        order_id -> Int2,
        product_id -> Int2,
        unit_price -> Float4,
        quantity -> Int2,
        discount -> Float4,
    }
}

diesel::table! {
    orders (order_id) {
        order_id -> Int2,
        #[max_length = 5]
        customer_id -> Nullable<Varchar>,
        employee_id -> Nullable<Int2>,
        order_date -> Nullable<Date>,
        required_date -> Nullable<Date>,
        shipped_date -> Nullable<Date>,
        ship_via -> Nullable<Int2>,
        freight -> Nullable<Float4>,
        #[max_length = 40]
        ship_name -> Nullable<Varchar>,
        #[max_length = 60]
        ship_address -> Nullable<Varchar>,
        #[max_length = 15]
        ship_city -> Nullable<Varchar>,
        #[max_length = 15]
        ship_region -> Nullable<Varchar>,
        #[max_length = 10]
        ship_postal_code -> Nullable<Varchar>,
        #[max_length = 15]
        ship_country -> Nullable<Varchar>,
    }
}

diesel::table! {
    products (product_id) {
        product_id -> Int2,
        #[max_length = 40]
        product_name -> Varchar,
        supplier_id -> Nullable<Int2>,
        category_id -> Nullable<Int2>,
        #[max_length = 20]
        quantity_per_unit -> Nullable<Varchar>,
        unit_price -> Nullable<Float4>,
        units_in_stock -> Nullable<Int2>,
        units_on_order -> Nullable<Int2>,
        reorder_level -> Nullable<Int2>,
        discontinued -> Int4,
    }
}

diesel::table! {
    region (region_id) {
        region_id -> Int2,
        #[max_length = 60]
        region_description -> Varchar,
    }
}

diesel::table! {
    shippers (shipper_id) {
        shipper_id -> Int2,
        #[max_length = 40]
        company_name -> Varchar,
        #[max_length = 24]
        phone -> Nullable<Varchar>,
    }
}

diesel::table! {
    suppliers (supplier_id) {
        supplier_id -> Int2,
        #[max_length = 40]
        company_name -> Varchar,
        #[max_length = 30]
        contact_name -> Nullable<Varchar>,
        #[max_length = 30]
        contact_title -> Nullable<Varchar>,
        #[max_length = 60]
        address -> Nullable<Varchar>,
        #[max_length = 15]
        city -> Nullable<Varchar>,
        #[max_length = 15]
        region -> Nullable<Varchar>,
        #[max_length = 10]
        postal_code -> Nullable<Varchar>,
        #[max_length = 15]
        country -> Nullable<Varchar>,
        #[max_length = 24]
        phone -> Nullable<Varchar>,
        #[max_length = 24]
        fax -> Nullable<Varchar>,
        homepage -> Nullable<Text>,
    }
}

diesel::table! {
    territories (territory_id) {
        #[max_length = 20]
        territory_id -> Varchar,
        #[max_length = 60]
        territory_description -> Varchar,
        region_id -> Int2,
    }
}

diesel::table! {
    us_states (state_id) {
        state_id -> Int2,
        #[max_length = 100]
        state_name -> Nullable<Varchar>,
        #[max_length = 2]
        state_abbr -> Nullable<Varchar>,
        #[max_length = 50]
        state_region -> Nullable<Varchar>,
    }
}

diesel::joinable!(customer_customer_demo -> customer_demographics (customer_type_id));
diesel::joinable!(customer_customer_demo -> customers (customer_id));
diesel::joinable!(employee_territories -> employees (employee_id));
diesel::joinable!(employee_territories -> territories (territory_id));
diesel::joinable!(order_details -> orders (order_id));
diesel::joinable!(order_details -> products (product_id));
diesel::joinable!(orders -> customers (customer_id));
diesel::joinable!(orders -> employees (employee_id));
diesel::joinable!(orders -> shippers (ship_via));
diesel::joinable!(products -> categories (category_id));
diesel::joinable!(products -> suppliers (supplier_id));
diesel::joinable!(territories -> region (region_id));

diesel::allow_tables_to_appear_in_same_query!(
    categories,
    customer_customer_demo,
    customer_demographics,
    customers,
    employee_territories,
    employees,
    order_details,
    orders,
    products,
    region,
    shippers,
    suppliers,
    territories,
    us_states,
);
