// use actix_web::{web, App, HttpResponse, HttpServer};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

pub fn run() {
    main();
}

fn main() {
    let server = HttpServer::new(|| 
        App::new().route("/", web::get().to(get_index)));
    println!("Serving on http://localhost:3000...");
    server
        .bind("127.0.0.1:3000")
        .expect("error binding server to address")
        .run();
    // .expect("error running server");
}

// #[get("/")]
// fn get_index() -> HttpResponse {
    async  fn get_index() -> String {
    //     HttpResponse::Ok().content_type("text/html").body(
    //         r#"
    //                     <title>GCD Calculator</title>
    //                     <form action="/gcd" method="post">
    //                     <input type="text" name="n"/>
    //                     <input type="text" name="m"/>
    //                     <button type="submit">Compute GCD</button>
    //                     </form>
    // "#,
    //     )

 let res =  r#"
<title>GCD Calculator</title>
<form action="/gcd" method="post">
<input type="text" name="n"/>
<input type="text" name="m"/>
<button type="submit">Compute GCD</button>
</form>
"# ;

res.to_owned() 
}
