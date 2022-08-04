use zero_to_prod::{
    configuration::get_configuration, startup::Application, telemetry::get_subscriber,
    telemetry::init_subscriber,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero_to_prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.yml");

    let application = Application::build(configuration).await?;
    application.run_until_stopped().await?;
    Ok(())
}
