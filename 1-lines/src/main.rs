use kiyo::app::app::{App, AppConfig};
use kiyo::app::audio_orch::AudioConfig;
use kiyo::app::draw_orch::{ClearConfig, DispatchConfig, DrawConfig, ImageConfig, Pass};

fn main() {

    let app_config = AppConfig {
        width: 1000,
        height: 1000,
        vsync: true,
        log_fps: true,
        fullscreen: false
    };

    let draw_config = DrawConfig {
        passes: vec![
            Pass {
                shader: "1-lines/shaders/fractal.comp".to_string(),
                dispatches: DispatchConfig::FullScreen,
                input_resources: Vec::from( [] ),
                output_resources: Vec::from([ 0 ]),
            },
        ],
        images: vec![
            ImageConfig {
                clear: ClearConfig::None
            }
        ],
    };

    App::run(app_config, draw_config, AudioConfig::None);
}