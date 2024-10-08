use std::path::{Path, PathBuf};

use f2_common_format::{GetProto, Pid};
use f2_map_format::{parse_map, Map};
use f2_proto_format::{parse_proto, proto::Proto};
use hashbrown::HashMap;

pub mod ignore;

pub struct Context {
    pub protos: HashMap<Pid, Proto>,
}
impl<'a> GetProto for &'a Context {
    type Proto = Proto;

    fn get_proto(&self, proto_id: Pid) -> Option<&Self::Proto> {
        self.protos.get(&proto_id)
    }
}

impl Context {
    pub fn init(protos_path: &Path) -> Self {
        let protos = Self::walk_protos(protos_path);
        Self { protos }
    }

    fn walk_protos(path: &Path) -> HashMap<Pid, Proto> {
        let pro = Some("PRO".into());
        let mut protos = HashMap::new();
        for entry in walkdir::WalkDir::new(path)
            .into_iter()
            .flatten()
            .filter(|entry| entry.file_type().is_file())
            .filter(|entry| entry.path().extension().map(|ext| ext.to_ascii_uppercase()) == pro)
        {
            let path = entry.path();
            let res = parse_proto(path);
            println!("{path:?}");
            let proto = res.unwrap();
            if let Some(old) = protos.insert(proto.common().proto_pid.pid(), proto) {
                panic!("Duplicate proto: {old:?}");
            }
        }
        println!("Parsed {} protos in {path:?}", protos.len());
        protos
    }

    pub fn walk_maps(
        &self,
        base_path: &Path,
        ignore: impl Fn(&Path, &Result<&str, ValidationError>) -> bool,
    ) -> Maps {
        let map = Some("MAP".into());

        let mut maps = Maps::default();

        for entry in walkdir::WalkDir::new(base_path)
            .into_iter()
            .flatten()
            .filter(|entry| entry.file_type().is_file())
            .filter(|entry| entry.path().extension().map(|ext| ext.to_ascii_uppercase()) == map)
        {
            let path = entry.path();
            println!("Parsing {path:?}");
            let stripped = path.strip_prefix(base_path).unwrap();
            let map = parse_map(path, self).unwrap();
            //println!("Ok! Header: {:?}, Objects count: {:?}, Tail: {:?}", res.header(), res.objects().all_objects().into_iter().map(|iter| iter.len()).collect::<Vec<_>>(), res.tail().len());

            let res = validate(&map, &maps.valid_maps);
            if (ignore)(stripped, &res) {
                maps.ignored_maps.insert(stripped.into(), (res.err(), map));
            } else {
                match res {
                    Ok(filename) => {
                        maps.valid_maps
                            .insert(filename.into(), (stripped.into(), map));
                    }
                    Err(reason) => {
                        maps.invalid_maps.insert(stripped.into(), (reason, map));
                    }
                }
            }
            //println!("Ok! Header: {:?}, Objects count: {:?}, Tail: {:?}", res.header(), res.objects().all_objects().into_iter().map(|iter| iter.len()).collect::<Vec<_>>(), res.tail().len());
        }
        println!(
            "Parsed {{ valid: {}, invalid: {}, ignored: {} }} maps in {base_path:?}",
            maps.valid_maps.len(),
            maps.invalid_maps.len(),
            maps.ignored_maps.len()
        );

        maps
    }
}

fn validate<'a>(
    map: &'a Map,
    maps: &HashMap<String, (PathBuf, Map)>,
) -> Result<&'a str, ValidationError> {
    let nested_container: Vec<_> = map
        .objects()
        .all_objects()
        .into_iter()
        .flatten()
        .filter(|cont| {
            cont.inventory
                .slots()
                .any(|obj| obj.object().common.inventory_count() != 0)
        })
        .collect();
    if !nested_container.is_empty() {
        return Err(ValidationError::NestedContainers(format!(
            "{:#?}",
            nested_container
        )));
    }

    let tail = map.tail();
    if !(tail.is_empty() || tail == [0; 4] || tail == [0; 8]) {
        return Err(ValidationError::NonNullTail);
    }

    if let Some(filename) = map.header().filename() {
        if let Some((other_path, _)) = maps.get(filename) {
            Err(ValidationError::Duplicate(other_path.clone()))
        } else {
            Ok(filename)
        }
    } else {
        Err(ValidationError::Filename)
    }
}

#[derive(Debug)]
pub enum ValidationError {
    NonNullTail,
    Filename,
    Duplicate(PathBuf),
    NestedContainers(String),
}

#[derive(Default)]
pub struct Maps {
    pub valid_maps: HashMap<String, (PathBuf, Map)>,
    pub invalid_maps: HashMap<PathBuf, (ValidationError, Map)>,
    pub ignored_maps: HashMap<PathBuf, (Option<ValidationError>, Map)>,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(path: &str, ignore: impl Fn(&Path, &Result<&str, ValidationError>) -> bool) -> Maps {
        let path = format!("../f2_dat/dats/{path}");
        let context = Context::init(path.as_ref());
        context.walk_maps(path.as_ref(), ignore)
    }

    fn show_valid(maps: &Maps) {
        for (_name, (path, map)) in &maps.valid_maps {
            eprintln!(
                "Valid map {path:?},\n    Header: {:?}, Objects count: {:?}\n",
                map.header(),
                map.objects()
                    .all_objects()
                    .into_iter()
                    .map(|iter| iter.len())
                    .collect::<Vec<_>>()
            );
        }
    }
    fn show_invalid(maps: &Maps) {
        for (path, (reason, map)) in &maps.invalid_maps {
            eprintln!("Invalid map {path:?},\n    because {reason:?}");
            if let ValidationError::NonNullTail = &reason {
                //let objects: Vec<_> = map.objects().all_objects().into_iter().flatten().collect();
                //let len = objects.len();
                //let last_objects = &objects[(len.saturating_sub(3))..len];

                //eprintln!("    last objects: {:#?}", last_objects);
                eprintln!("    tail: {:?} bytes\n", map.tail().len());
            }
        }
    }
    fn show_ingnored(maps: &Maps) {
        for (path, _map) in &maps.ignored_maps {
            eprintln!("Ignored map {path:?}");
        }
    }

    #[test]
    fn parse_fallout2() {
        let maps = parse("fallout2/master.dat", ignore::fallout2);
        show_valid(&maps);
        show_ingnored(&maps);
        show_invalid(&maps);
        assert_eq!(maps.invalid_maps.len(), 0);
        assert_eq!(maps.ignored_maps.len(), 2);
        assert_eq!(maps.valid_maps.len(), 153);
    }

    #[test]
    fn parse_sonora() {
        let maps = parse("sonora/master.dat", ignore::sonora);
        show_valid(&maps);
        show_ingnored(&maps);
        show_invalid(&maps);
        assert_eq!(maps.invalid_maps.len(), 0);
        assert_eq!(maps.ignored_maps.len(), 0);
        assert_eq!(maps.valid_maps.len(), 140);
    }

    #[test]
    fn parse_nevada() {
        let maps = parse("nevada/master.dat", ignore::nevada);
        show_valid(&maps);
        show_ingnored(&maps);
        show_invalid(&maps);
        assert_eq!(maps.invalid_maps.len(), 0);
        assert_eq!(maps.ignored_maps.len(), 7);
        assert_eq!(maps.valid_maps.len(), 131);
    }

    #[test]
    fn parse_olympus() {
        let maps = parse("olympus/master.dat", ignore::olympus);
        show_valid(&maps);
        show_ingnored(&maps);
        show_invalid(&maps);
    }
}
