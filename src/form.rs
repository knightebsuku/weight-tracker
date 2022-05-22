use chrono::{Local, NaiveDate};
use rocket::request::FromFormValue;
use rocket::http::RawStr;


pub struct WeightDate(NaiveDate);

impl WeightDate{
    pub fn as_str(&self) -> NaiveDate{
        self.0
    }
}
impl<'v> FromFormValue<'v> for WeightDate {
    type Error = &'static str;

    fn from_form_value(date: &'v RawStr) -> Result<Self, Self::Error>{
        let dt = NaiveDate::parse_from_str(date, "%Y-%m-%d");
        match dt {
            Ok(d) => {
                if d > Local::now().naive_local().date() {
                    return Err("Date cannot be in the future");
                }
                Ok(WeightDate(d))
            }
            Err(_e) => Err("Invalid Date Format"),
        }
    }
}

#[derive(FromForm)]
pub struct WeightForm {
    pub weight: f32,
    pub date: Result<WeightDate, &'static str>,
}