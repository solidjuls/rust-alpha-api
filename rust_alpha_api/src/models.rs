use diesel::Insertable;
use super::schema::users;

#[derive(Queryable)]
pub struct Users {
    pub id: i64,
    pub name: Option<String>,
    pub email: Option<String>,
    pub email_verified_at: Option<chrono::NaiveDateTime>,
    pub password: Option<String>,
    pub remember_token: Option<String>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub country_id: Option<i64>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub last_login_at: Option<chrono::NaiveDateTime>,
}


#[derive(Insertable)]
#[table_name="users"]
pub struct NewUsers<'u> {
    pub name: &'u str,
    pub email: &'u str,
    pub password: &'u str,
    pub remember_token: &'u str,
    pub first_name: &'u str,
    pub last_name: &'u str,
}
