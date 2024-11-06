use  std::error::Error;
fn main() -> Result<(), Box<dyn Error>> {
    build_app_ui()?;
    Ok(())
}

fn build_app_ui() -> Result<(), Box<dyn Error>> {
    Ok(slint_build::compile("app/app.slint")?)
}