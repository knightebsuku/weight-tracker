mod database;
mod form;

use database::get_weights;
use rocket_dyn_templates::{Template, context};
use rocket::fs::FileServer;
use rocket::form::{Form, Contextual};
use rocket::request::FlashMessage;
use rocket::response::{Redirect, Flash};
use serde::Serialize;
use rocket::{get, post, launch, routes};
use rocket_db_pools::{Database, Connection};


#[derive(Serialize)]
struct TemplateMessage{
    status: String,
    message: String
}
#[derive(Serialize)]
struct TemplateData{
    weight: Vec<database::Weight>,
    current: f32,
    message: Option<TemplateMessage>
}

#[get("/")]
async fn index(db: Connection<database::WeighDB>, flash: Option<FlashMessage<'_>>) -> Template{
    let weights = match get_weights(db).await{
        Ok(w) => w,
        Err(err) => {
            return Template::render("errors", context!(error: err.to_string()));
        }
    };
    let mut current_weight: f32 = 0.0;
    if weights.len() > 0{
        current_weight = weights[0].kg;    
    }
    
    let message: Option<TemplateMessage> = flash.map(
        |flash| TemplateMessage{status: flash.kind().to_string(), message: flash.message().to_string()}
    );
    let context = TemplateData{
        weight: weights,
        current: current_weight,
        message: message
    };
    Template::render("index",&context)

}

#[post("/", data="<form>")]
async fn submit<'r>(db: Connection<database::WeighDB>, form: Form<Contextual<'r, form::WeightForm>>) -> Flash<Redirect>{
    if let Some(ref valid_form) = form.value{
        let date = valid_form.date;
        let kg = valid_form.weight;
        match database::create_weight(db, date, kg).await{
            Ok(_) => Flash::success(Redirect::to("/"), "Weight Added"),
            Err(_) => Flash::success(Redirect::to("/"), "Unable to insert weight")
        }
    }else{
        Flash::error(Redirect::to("/"), "Invalid form data")

    }

}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .attach(database::WeighDB::init())
        .mount("/", routes![index, submit])
        .mount("/static", FileServer::from("static"))
}
