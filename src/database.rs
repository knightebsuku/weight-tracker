use serde::{Serialize, Serializer};
use rocket_db_pools::{Database, Connection};
use rocket_db_pools::sqlx::PgPool;
use sqlx::FromRow;
use time;

// Setup Connection Pool
#[derive(Database)]
#[database("weight_tracker")]
pub struct WeighDB(PgPool);


fn serialize_date<S>(date: &time::Date, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let date_string = date.to_string();
    serializer.serialize_str(&date_string)
}



#[derive(Debug, Serialize, FromRow)]
pub struct Weight{
    #[serde(serialize_with = "serialize_date")]
    pub date: time::Date,
    pub kg: f32
}


pub async fn get_weights(mut db: Connection<WeighDB>) -> Result<Vec<Weight>, sqlx::Error>{
    let weights: Vec<Weight> = sqlx::query_as("SELECT kg, date from weight order by id desc")
    .fetch_all(&mut **db).await?;
    Ok(weights)
}

pub async fn create_weight(mut db: Connection<WeighDB>, date: time::Date, kg: f32) -> Result<(), sqlx::Error>{
    let _row = sqlx::query("insert into weight(date, kg) values($1, $2)")
    .bind(date)
    .bind(kg)
    .execute(&mut **db).await?;
    Ok(())
}