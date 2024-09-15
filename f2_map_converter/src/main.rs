use std::path::Path;

use f2_map_converter::{ignore, Context};

fn main() {
    env_logger::init();

    let base_path = "../f2_dat/dats/fallout2";
    let protos_path = Path::new("../f2_dat/output/fallout2/protos");
    let maps_path = Path::new("../f2_dat/output/fallout2/maps");
    let context = Context::init(base_path.as_ref());

    for (pid, proto) in &context.protos {
        match serde_json::to_string_pretty(proto) {
            Ok(json) => {
                let json_path = protos_path.join(format!("{:?}/{}.json", pid.ty(), pid.id()));
                if let Err(err) = std::fs::create_dir_all(json_path.parent().unwrap()) {
                    log::error!("Can't create path {json_path:?} for proto: {err:#}");
                }
                if let Err(err) = std::fs::write(&json_path, &json) {
                    log::error!("Can't write proto to {json_path:?}: {err:#}");
                }
            }
            Err(err) => {
                log::error!(
                    "Can't serialize proto {} ({:?}): {err:#}",
                    pid.id(),
                    pid.ty()
                );
            }
        }
    }

    let maps = context.walk_maps(base_path.as_ref(), ignore::fallout2);
    for (name, (path, map)) in &maps.valid_maps {
        match serde_json::to_string_pretty(map) {
            Ok(json) => {
                let mut json_path = maps_path.join(path);
                json_path.set_extension("json");
                if let Err(err) = std::fs::create_dir_all(json_path.parent().unwrap()) {
                    log::error!(
                        "Can't create path {json_path:?} for map {name:?} from {path:?}: {err:#}"
                    );
                    continue;
                }
                if let Err(err) = std::fs::write(&json_path, &json) {
                    log::error!("Can't write map {name:?} from {path:?} to {json_path:?}: {err:#}");
                }
            }
            Err(err) => {
                log::error!("Can't serialize mape {name:?} from {path:?}: {err:#}");
            }
        }
    }
}
