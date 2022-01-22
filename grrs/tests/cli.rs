use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    
    let mut cmd = Command::cargo_bin("grrs")?;

    cmd.arg("foobar").arg("test/file");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("could not read file"));
    Ok(())
}

// #[test]
// fn test() {
//     let result = 42;
//     let expected = grrs::answer();
//     assert_eq!(expected, result);
// }

// use tempfile::tempdir;
// use std::io::Write;
// #[test]
// #[ignore]
// fn find_a_match() -> Result<()>{
//     let dir = tempdir()?;
//     let filepath = dir.path().join("find_matches_test.txt");
//     let mut file = File::create(filepath)?;
//     write!(file, "lorem ipsum\ndolor sit amet")?;
//     let reader = BufReader::new(file);
//     let mut result = Vec::new();
//     grrs::find_matches(reader, "lorem", &mut result)?;
//     assert_eq!(result, b"lorem ipsum");   

//     Ok(())    
// }