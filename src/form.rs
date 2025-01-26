use rocket::form::{FromForm, Result, Error};
use serde::Serialize;
use time;


fn date_validate<'v>(date: &time::Date) -> Result<'v, ()>{
    if *date > time::OffsetDateTime::now_utc().date(){
        return Err(Error::validation("Date cannot be in the future"))?;
    }
    Ok(())
}

#[derive(Serialize, FromForm)]
pub struct WeightForm {
    pub weight: f32,
    #[field(validate = date_validate())]
    pub date: time::Date
}

