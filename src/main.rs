use aperture::{
    exec::{Config, Exec, MultiThreaded},
    scene::Scene,
};
use std::path::PathBuf;
use std::time::SystemTime;

const SCENE_PATH: &str = "cornell.json";

fn main() {
    let num_threads = num_cpus::get() as u32;
    let out_path = PathBuf::from("./");

    let (mut scene, mut rt, spp, frame_info) = Scene::load_file(SCENE_PATH);
    let dim = rt.dimensions();

    let scene_start = SystemTime::now();
    let mut config = Config::new(
        out_path,
        SCENE_PATH.to_string(),
        spp,
        num_threads,
        frame_info,
        (0, 0),
    );
    let mut exec = MultiThreaded::new(num_threads);
    for i in frame_info.start..frame_info.end + 1 {
        config.current_frame = i;
        exec.render(&mut scene, &mut rt, &config);

        let img = rt.get_render();
        let out_file = match config.out_path.extension() {
            Some(_) => config.out_path.clone(),
            None => config
                .out_path
                .join(PathBuf::from(format!("frame{:05}.png", i))),
        };
        match image::save_buffer(
            &out_file.as_path(),
            &img[..],
            dim.0 as u32,
            dim.1 as u32,
            image::RGB(8),
        ) {
            Ok(_) => {}
            Err(e) => println!("Error saving image, {}", e),
        };
        rt.clear();
        println!(
            "Frame {}: rendered to '{}'\n--------------------",
            i,
            out_file.display()
        );
    }
    let time = scene_start.elapsed().expect("Failed to get render time?");
    println!(
        "Rendering entire sequence took {:4}s",
        time.as_secs() as f64 + time.subsec_nanos() as f64 * 1e-9
    )
}
