#![feature(proc_macro_hygiene, decl_macro)]

mod database;
mod form;

#[macro_use] extern crate rocket;

use rocket_contrib::templates::Template;
use rocket_contrib::serve::StaticFiles;
use serde::Serialize;
use chrono::NaiveDate;
use rocket::request::{FlashMessage, Form};
use rocket::response::{Redirect, Flash};

#[derive(Serialize)]
struct Weight{
    date: NaiveDate,
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
    let message: Option<TemplateMessage> = flash.map(|msg| TemplateMessage{status: msg.name().to_string(), message: msg.msg().to_string()});
    let context = TemplateData{
        weight: values,
        current: current_weight,
        message: message
    };
    Template::render("index",&context)

}

#[post("/", data="<weight_form>")]
fn submit(weight_form: Form<form::WeightForm>) -> Flash<Redirect>{
    if let Err(e) = weight_form.date{
        return Flash::error(Redirect::to("/"), e);
    }
    let form = weight_form.into_inner();
    let date = form.date.unwrap();
    let kg = form.weight;
    let mut client = database::get_client().unwrap();
    client
        .execute("INSERT INTO weight(date, kg) VALUES($1,$2)", &[&date.as_str(), &kg])
        .unwrap();
    Flash::success(Redirect::to("/"), "Weight Added")

}
fn main() {
    rocket::ignite()
        .attach(Template::fairing())
        .mount("/", routes![index, submit])
        .mount("/static", StaticFiles::from("static"))
        .launch();
}
