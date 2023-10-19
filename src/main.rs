extern crate image;

mod neuronal_network;
mod input;

use getopts::Options;
use std::{env};
use neuronal_network::hopfield;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();

    // Indica un directorio que contiene imágenes que serán procesados como entrada
    opts.optopt("", "idir", "Input images directory", "IMG_DIR");
    // Indica que la entrada serán imágenes. No es necesario indicarlo si se utiliza
    // la opción idir. En caso de no usar la opción idir, el programa pedirá que se
    // inserte la ruta de las imágenes que se procesarán 1 por 1.
    opts.optflag("i", "image", "Allows to insert images as inputs");
    // Permite mostrar la matriz de peso en consola
    opts.optflag(
        "",
        "show-weight-matrix",
        "Show weight matrix in the console",
    );

    opts.optflag("", "hopfield", "Execute hopfield neuronal network");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            panic!("{}", f.to_string())
        }
    };

    if matches.opt_present("hopfield") {
        hopfield::execute(&matches);
    }
}
