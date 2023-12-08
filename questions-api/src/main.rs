use actix_web::web::Data;
use actix_web::Result;
use actix_web::{get, web, App, HttpServer};
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct QuizzEntries {
    questions: Vec<QuizzEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct QuizzEntry {
    question: String,
    options: [String; 4],
    answer: usize,
    explanation: String,
}

#[get("/question")]
async fn get_question(data: web::Data<QuizzEntries>) -> Result<web::Json<QuizzEntry>> {
    let mut rng = thread_rng();

    let index = rng.gen_range(0..data.questions.len());

    Ok(web::Json(data.questions[index].clone()))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let questions = include_str!("../questions.json");
    let questions = serde_json::from_str::<QuizzEntries>(questions).unwrap();
    let questions = Data::new(questions);

    HttpServer::new(move || {
        let cors = actix_cors::Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();
        App::new()
            .wrap(cors)
            .service(get_question)
            .app_data(questions.clone())
    })
    .bind("0.0.0.0:8000")
    .unwrap()
    .run()
    .await
    .unwrap();

    Ok(())
}
