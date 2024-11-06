use  std::error::Error;
fn main() -> Result<(), Box<dyn Error>> {
    build_app_ui()?;
    Ok(())
}

fn build_app_ui() -> Result<(), Box<dyn Error>> {
    let config = slint_build::CompilerConfiguration::new().with_style("cosmic-dark".into());
    Ok(slint_build::compile_with_config("app/app.slint", config)?)
}