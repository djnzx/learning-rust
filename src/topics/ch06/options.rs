fn playground() {
    let x: Option<u32> = Some(33);
    let y: Option<u32> = None;

    match x {
        None => {}
        Some(_) => {}
    }

    let y: u32 = 7;
    let z = x.map(|x| x + 1);

    /// if let syntax
    let config_max: Option<u8> = Some(3u8);

    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }
    //     ==========================
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    // TODO: what about either ???
}
