use actix_web::{web, HttpRequest, HttpResponse, Responder};

use crate::controllers:: {
    app_controller,
};

pub fn router(config: &mut web::ServiceConfig) {
    config.route("/", web::get().to(app_controller::index));
}
