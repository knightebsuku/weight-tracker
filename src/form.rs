use rocket::form::{FromForm, Result, Error};
use time::{self, format_description::well_known::Iso8601};


fn date_validate<'v>(date: &time::Date) -> Result<'v, ()>{
    if *date > time::OffsetDateTime::now_utc().date(){
        return Err(Error::validation("Date cannot be in the future"))?;
    }else{
        Ok(())
    }
    // let _ = match time::PrimitiveDateTime::parse(date, &Iso8601::DATE){
    //     Ok(d) => {
    //         if d.date() > time::OffsetDateTime::now_utc().date(){
    //             return Err(Error::validation("Date cannot be in the future"))?;
    //         }
    //         Ok(())
    //     },
    //     Err(_) => Err(Error::validation("Invalid date format"))
    // };
    // Ok(())


}

#[derive(FromForm)]
pub struct WeightForm {
    pub weight: f32,
    #[field(validate = date_validate())]
    pub date: time::Date
}

