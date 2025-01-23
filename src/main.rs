mod database;
mod form;

use rocket_dyn_templates::Template;
use rocket::fs::FileServer;
use rocket::form::Form;
use rocket::request::FlashMessage;
use rocket::response::{Redirect, Flash};
use serde::Serialize;
use rocket::{get, post, launch};
use chrono;

//#[macro_use] extern crate rocket;

#[derive(Serialize)]
struct Weight{
    date: chrono::NaiveDate,
    kg: f32
}

#[derive(Serialize)]
struct TemplateMessage{
    status: String,
    message: String
}
#[derive(Serialize)]
struct TemplateData{
    weight: Vec<Weight>,
    current: f32,
    message: Option<TemplateMessage>
}

#[get("/")]
fn index(flash: Option<FlashMessage>) -> Template{
    let mut values = Vec::new();
    let mut client = database::get_client().unwrap();
    for row in client
        .query("SELECT kg, date from weight order by id desc", &[])
        .unwrap()
    {
        values.push(Weight {
            kg: row.get(0),
            date: row.get(1),
        })
    }
    let current_weight = values[0].kg;
    let message: Option<TemplateMessage> = flash.map(
        |flash| TemplateMessage{status: flash.kind().to_string(), message: flash.message().to_string()}
    );
    let context = TemplateData{
        weight: values,
        current: current_weight,
        message: message
    };
    Template::render("index",&context)

}

#[post("/", data="<weight_form>")]
fn submit(weight_form: Form<form::WeightForm>) -> Flash<Redirect>{
    let f = weight_form.value();

    if let Err(e) = weight_form.date{
        return Flash::error(Redirect::to("/"), e);
    }
    let form = weight_form.into_inner();
    let date = form.date;
    let kg = form.weight;
    let mut client = database::get_client().unwrap();
    client
        .execute("INSERT INTO weight(date, kg) VALUES($1,$2)", &[&date.as_str(), &kg])
        .unwrap();
    Flash::success(Redirect::to("/"), "Weight Added")

}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .mount("/", routes![index, submit])
        .mount("/static", FileServer::from("static"))
        .launch()
}
