use std::{env, fs::File, io};

use day_01::process;
fn main() -> Result<(), &'static str> {
    let filename = env::args()
        .nth(1)
        .ok_or("Il faut un nom de fichier d'input !")?;
    let file = File::open(filename).map_err(|_| "Impossible d'ouvrir le fichier")?;
    let result = process(io::BufReader::new(file));

    println!("Number obtained: {}", result);
    Ok(())
}
