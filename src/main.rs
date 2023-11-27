extern crate image;

mod input;
mod neuronal_network;
mod utils;

use getopts::Options;
use neuronal_network::{hamming, hopfield, lam, perceptron};
use std::env;

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
    opts.optflag("", "hamming", "Execute hamming neuronal network");
    opts.optflag("", "lam", "Execute lam neuronal network");
    opts.optflag("", "perceptron", "Execute perceptron neuronal network");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            panic!("{}", f.to_string())
        }
    };

    if matches.opt_present("hopfield") {
        hopfield::execute(&matches);
    } else if matches.opt_present("hamming") {
        hamming::execute(&matches);
    } else if matches.opt_present("lam") {
        lam::execute(&matches);
    } else if matches.opt_present("perceptron") {
        perceptron::execute(&matches);
    }
}
