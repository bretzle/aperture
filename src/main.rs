// const PBRT_PATH: &str = "scenes/cornell-box/scene.pbrt";
// const PBRT_PATH: &str = "hello.pbrt";

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    env_logger::Builder::from_default_env()
        .format_timestamp(None)
        .parse_filters("info")
        .init();

    Ok(())
}
