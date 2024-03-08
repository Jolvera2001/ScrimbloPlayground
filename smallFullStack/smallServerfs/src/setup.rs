use sea_orm::*;

const DATABASE_URL: &str = "postgresql://jolvera6:L49hbFIXpqlo@ep-lively-glitter-a5akpl5b.us-east-2.aws.neon.tech/AplHelp?sslmode=require";
const DATABASE_NAME: &str = "AplHelp";

pub(super) async fn set_up_db() -> Result<DatabaseConnection, DbErr> {
    // connecting to db
    let db = Database::connect(DATABASE_URL).await?;
}