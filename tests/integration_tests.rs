// integration_tests.rs

use assert_cmd::Command;


#[test]
fn run_with_defaults() -> Result<(), Box<dyn std::error::Error>> {

    Command::cargo_bin("vadersay")
        .expect("Binary does not exist")
        .assert()
        .stdout(predicates::str::contains("Inspired by danlogs!"))
        .success();
    
    Ok(())
}


#[test]
fn fail_on_non_existing_file() -> Result<(), Box<dyn std::error::Error>> {
    
    Command::cargo_bin("vadersay")
        .expect("Binary does not exist")
        .args(&["-f", "/1797131648794664674987/no/such/file.txt"])
        .assert()
        .failure();

    Ok(())
}
