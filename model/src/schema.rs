table! {
    users (id) {
        id -> Bigint,
        name -> Nullable<Varchar>,
        email -> Nullable<Varchar>,
        email_verified_at -> Nullable<Timestamp>,
        password -> Nullable<Varchar>,
        remember_token -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        country_id -> Nullable<Bigint>,
        first_name -> Nullable<Varchar>,
        last_name -> Nullable<Varchar>,
        last_login_at -> Nullable<Timestamp>,
    }
}
