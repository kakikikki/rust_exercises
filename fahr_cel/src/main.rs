fn main() {
    // Header
    println!("===================");
    println!("|  Fahr  |  Cels  |");

    for fahr in (0..101).step_by(10) {
        let cels = fahr_cels(fahr);
        println!("| {:5}  | {:5.1}  |",
                    fahr,  cels);
    }
    // Footer
    println!("===================");

    // Header again
    println!("===================");
    println!("|  Cels  |  Fahr  |");

    for cels in (0..101).step_by(10) {
        let fahr = cels_fahr(cels);
        println!("| {:5}  | {:5.1}  |",
                    cels,  fahr);
    }

    // Footer again
    println!("===================");
}

fn fahr_cels(fahr: i32) -> f64 {
    let ratio = 5_f64 / 9_f64;
    let cels: f64 = (fahr - 32) as f64 * ratio;
    return cels
}

fn cels_fahr(cels: i32) -> f64 {
    let ratio = 9_f64 / 5_f64;
    let fahr: f64 = (cels as f64 * ratio) + 32_f64;
    return fahr
}
