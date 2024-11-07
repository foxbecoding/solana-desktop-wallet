use slint_build::CompileError;

fn main() -> Result<(), CompileError> {
    build_app_ui()?;
    Ok(())
}

fn build_app_ui() -> Result<(), CompileError> {
    let config = slint_build::CompilerConfiguration::new().with_style("fluent-dark".into());
    slint_build::compile_with_config("app/app.slint", config)?;
    Ok(())
}