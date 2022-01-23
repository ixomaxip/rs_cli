
// mod common {
//     fn answer() -> i32 {
//         42
//     }
// }


pub mod helpers {
    use anyhow::Result;
    use std::fs::File;
    use std::io::{BufReader, BufRead};
    use log::{warn};

    pub fn find_matches(reader: BufReader<File>, pattern: &str, mut writer: impl std::io::Write) -> Result<()> {
        for (idx, line) in reader.lines().enumerate() {
            let line = match line {
                Ok(l) => l,
                Err(error) => { warn!("line error: {:?}", error); "".to_string() }
            };
            if line.contains(pattern) {
                writeln!(writer, "{}\t{}", idx, line)?;
            }
        }
        Ok(())
    }
}