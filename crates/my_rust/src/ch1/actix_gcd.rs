use actix_web::{get, post, web, App, HttpResponse, HttpServer};
use serde::Deserialize;

// NOTE: Copy from internet: https://medium.com/@gitaeklee/rust-gcd-function-web-application-back-end-4128943a851b

#[derive(Deserialize)]
struct GcdParameters {
    n: u64,
    m: u64,
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

pub fn run() {
    // @see https://docs.rs/actix-rt/latest/actix_rt/struct.Runtime.html
    let system_runner = actix_web::rt::System::new();
    let actix_runtime = system_runner.runtime();

    //  actix_runtime.spawn(async {

    //  });
    actix_runtime.block_on(async {
        main().await;
    })
}

// #[actix_web::main]
// #[::actix_web::rt::main(system = "::actix_web::rt::System")]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_index))
            .route("/gcd", web::post().to(post_gcd))
    });

    println!("Serving on http://localhost:3000...");

    server.bind(("127.0.0.1", 3000))?.run().await
}

async fn post_gcd(form: web::Form<GcdParameters>) -> HttpResponse {
    if form.n == 0 || form.m == 0 {
        return HttpResponse::BadRequest()
            .content_type("text/html")
            .body("computing the gcd with zero is boring");
    }

    let response = format!(
        "the greatest common divisor of the numbers {} and {} is <b> {} </b>\n",
        form.n,
        form.m,
        gcd(form.n, form.m)
    );

    HttpResponse::Ok().content_type("text/html").body(response)
}

async fn get_index() -> HttpResponse {
    HttpResponse::Ok().content_type("text/html").body(
        r#"
                <title>gcd calculator</title>
                <form action="/gcd" method="post">
                <input type="text" name="n" placeholder="first number" />
                <input type="text" name="m" placeholder="second number" /> 
                <button type="submit"> Compute GCD </button>
                </form>
            "#,
    )
}
