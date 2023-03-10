use cfg_if::cfg_if;
use leptos::*;
use tracing::info;

// boilerplate to run in different modes
cfg_if! {
    // server-only stuff
    if #[cfg(feature = "ssr")] {
        use actix_files::{Files};
        use actix_web::*;
        use valera_web::{App,AppProps};
        use leptos_actix::{LeptosRoutes, generate_route_list};

        #[get("/style.css")]
        async fn css() -> impl Responder {
            actix_files::NamedFile::open_async("./style.css").await
        }
        #[get("/favicon.ico")]
        async fn favicon() -> impl Responder {
            actix_files::NamedFile::open_async("./target/site//favicon.ico").await
        }

        #[actix_web::main]
        async fn main() -> std::io::Result<()> {
            tracing_subscriber::fmt::init();

            // Setting this to None means we'll be using cargo-leptos and its env vars.
            let conf = get_configuration(None).await.unwrap();

            let addr = conf.leptos_options.site_addr.clone();
            // Generate the list of routes in your Leptos App
            let routes = generate_route_list(|cx| view! { cx, <App/> });

            info!("Starting server at {}", addr);

            HttpServer::new(move || {
                let leptos_options = &conf.leptos_options;
                let site_root = &leptos_options.site_root;

                App::new()
                    .service(css)
                    .service(favicon)
                    .route("/api/{tail:.*}", leptos_actix::handle_server_fns())
                    .leptos_routes(leptos_options.to_owned(), routes.to_owned(), |cx| view! { cx, <App/> })
                    .service(Files::new("/", &site_root))
                    .wrap(middleware::Compress::default())
            })
            .bind(&addr)?
            .run()
            .await
        }
    } else {
        fn main() {
            use valera_web::{App, AppProps};

            _ = console_log::init_with_level(log::Level::Debug);
            console_error_panic_hook::set_once();
            mount_to_body(|cx| view! { cx, <App/> })
        }
    }
}
