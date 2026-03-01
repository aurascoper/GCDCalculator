use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct GcdParameters {
    n: u64,
    m: u64,
}

#[actix_web::main] // <--- THIS MUST BE HERE
async fn main() -> std::io::Result<()> {
    println!("Starting server at http://localhost:3000");
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_index))
            .route("/gcd", web::post().to(post_gcd))
    })
    .bind("127.0.0.1:3000")? // Using 127.0.0.1 is standard for local dev
    .run()
    .await
}

async fn get_index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(
            r#"
            <title>GCD Calculator</title>
            <style>
                body { font-family: sans-serif; display: flex; justify-content: center; align-items: center; height: 100vh; margin: 0; background-color: #f0f2f5; }
                form { background: white; padding: 2rem; border-radius: 8px; box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1); width: 300px; }
                h2 { margin-top: 0; color: #333; text-align: center; }
                input { display: block; width: 100%; margin: 10px 0; padding: 10px; border: 1px solid #ccc; border-radius: 4px; box-sizing: border-box; }
                button { width: 100%; padding: 10px; background-color: #1a73e8; color: white; border: none; border-radius: 4px; cursor: pointer; font-size: 16px; }
                button:hover { background-color: #1557b0; }
            </style>
            <form action="/gcd" method="post">
                <h2>GCD Calculator</h2>
                <input type="number" name="n" placeholder="First number" required/>
                <input type="number" name="m" placeholder="Second number" required/>
                <button type="submit">Calculate GCD</button>
            </form>
            "#,
        )
}

async fn post_gcd(form: web::Form<GcdParameters>) -> impl Responder {
    if form.n == 0 || form.m == 0 {
        return HttpResponse::BadRequest()
            .content_type("text/html")
            .body("Error: Please enter non-zero numbers. <a href='/'>Back</a>");
    }

    let result = gcd(form.n, form.m);

    let response_html = format!(
        r#"
        <title>Result</title>
        <style>
            body {{ font-family: sans-serif; display: flex; flex-direction: column; justify-content: center; align-items: center; height: 100vh; margin: 0; background-color: #f0f2f5; }}
            .card {{ background: white; padding: 2rem; border-radius: 8px; box-shadow: 0 4px 6px rgba(0,0,0,0.1); text-align: center; }}
            .result {{ font-size: 20px; margin-bottom: 20px; color: #333; }}
            a {{ color: #1a73e8; text-decoration: none; font-weight: bold; border: 1px solid #1a73e8; padding: 8px 16px; border-radius: 4px; }}
            a:hover {{ background-color: #1a73e8; color: white; }}
        </style>
        <div class="card">
            <div class="result">
                The GCD of <b>{}</b> and <b>{}</b> is <b style="color: #1a73e8;">{}</b>
            </div>
            <a href="/">Calculate again</a>
        </div>
        "#,
        form.n, form.m, result
    );

    HttpResponse::Ok()
        .content_type("text/html")
        .body(response_html)
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    while m != 0 {
        if m < n {
            std::mem::swap(&mut m, &mut n);
        }
        m %= n;
    }
    n
}