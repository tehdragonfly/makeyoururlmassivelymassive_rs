use super::schema::massive_urls;


#[table_name="massive_urls"]
#[derive(Queryable, Insertable)]
pub struct MassiveURL {
    pub id: Option<i32>,
    pub path: String,
    pub destination: String,
}
