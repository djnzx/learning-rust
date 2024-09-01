#[test]
fn standard() {
    let config_max = Some(3u8);
    // let config_max: Option<u32> = None;
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => println!("The maximum is not configured"),
    }
}

fn cleanish() {
    let config_max = Some(3u8);

    // partial function ), covering only one case
    // lose the exhaustive checking
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }
}
