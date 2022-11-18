pub async fn start_server() {
    println!("Starting server..");

    rouille::start_server("0.0.0.0:8080", move |request| {
        rouille::router!(request,
            (GET) (/) => {
                rouille::Response::text("Hello world!")
            },

            _ => rouille::Response::empty_404()
        )
    });
}
