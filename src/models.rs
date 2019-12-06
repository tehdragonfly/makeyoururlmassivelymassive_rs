#[derive(Queryable)]
pub struct MassiveURL {
    pub id: i32,
    pub path: String,
    pub destination: String,
}
