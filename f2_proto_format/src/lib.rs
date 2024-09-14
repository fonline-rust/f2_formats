use std::path::Path;

pub mod proto;

pub fn parse_proto(path: &Path) -> Result<proto::Proto, f2_common_format::reader::F2ReaderError> {
    let bytes = std::fs::read(path).unwrap();
    f2_common_format::reader::F2Reader::read(&bytes, ())
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::parse_proto;
    fn parse(path: &Path) {
        let res = parse_proto(path);
        println!("{path:?}: {res:#?}");
        res.unwrap();
    }
    fn walk(path: &str) {
        let mut walked = 0;
        let pro = Some("PRO".into());
        for entry in walkdir::WalkDir::new(format!("/home/qthree/gamedev/fonline/f2_dat/{path}"))
            .into_iter()
            .flatten()
            .filter(|entry| entry.file_type().is_file())
            .filter(|entry| entry.path().extension().map(|ext| ext.to_ascii_uppercase()) == pro)
        {
            parse(entry.path());
            walked += 1;
        }
        println!("Parsed {walked} protos in {path}");
    }
    #[test]
    fn parse_fallout2() {
        walk("fallout2/master.dat");
    }

    #[test]
    fn parse_sonora() {
        walk("sonora/master.dat");
    }

    #[test]
    fn parse_nevada() {
        walk("nevada/master.dat");
    }

    #[test]
    fn parse_olympus() {
        walk("olympus/master.dat");
    }
}
