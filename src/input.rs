use getopts::{Matches};
use std::{fs, io};

use crate::neuronal_network::hopfield;

pub fn get_input(matches: &Matches) -> Vec<(Vec<f64>, String)> {
    let mut input: Vec<(Vec<f64>, String)> = vec![];
    let mut n: usize = 0;

    // Si es verdad, el programa esperará que la entrada de la red neuronal
    // será un conjunto de imágenes que tiene que leer de dos formas:
    // -> Leyéndolas desde un directorio (cuya ruta es especificada como argumento en la CLI
    // -> Haciendo una iteración para pedir las rutas de la imágenes
    if matches.opt_present("i") {
        // Verifica si se tiene que agregar desde la lectura
        // de los archivos dentro de un directorio
        if matches.opt_present("idir") {
            let dir_path = matches.opt_str("idir").expect("The dir path is invalid");
            let dir_path = dir_path.trim();

            let images_path = fs::read_dir(dir_path).expect("The dir path is invalid");

            for file in images_path {
                let file = file.expect("Error reading file");
                let img = image::open(file.path()).expect("The file is not an image");
                let mut input_matrix = vec![];

                if  matches.opt_present("hopfield") {
                    input_matrix = hopfield::image_transformation(&img.to_luma8().into_vec());
                }

                if n == 0 {
                    n = input_matrix.len();
                } else if input_matrix.len() != n {
                    panic!("The image has to has the same number of neurons");
                }

                let file_name = file.file_name().into_string().expect("Invalid file name");

                input.push((input_matrix, file_name));
            }
            // Itera (hasta que el usuario lo quiera) para pedir la ruta de las imágenes
            // que se desean usar de manera individual
        } else {
            loop {
                let mut value = String::new();

                println!("Ruta de la imagen ({}):", input.len());

                io::stdin()
                    .read_line(&mut value)
                    .expect("Error reading user input");

                let value = value.trim();

                if value == ".done" {
                    break;
                }

                let img = image::open(value).expect("The file is not an image");

                let file_name: Vec<&str> = value.split(&['/', '\\']).collect();
                let file_name = file_name[file_name.len() - 1];
                let mut input_matrix = vec![];

                if  matches.opt_present("hopfield") {
                    input_matrix = hopfield::image_transformation(&img.to_luma8().into_vec());
                }

                if n == 0 {
                    n = input_matrix.len();
                } else if input_matrix.len() != n {
                    panic!("The image must has the same number of neurons");
                }

                input.push((input_matrix, file_name.into()));
            }
        }
        // Recibe un conjunto de números (1 o -1) separada por un espacio que guardará como
        // un patrón de entrada n veces (hasta que el usuario detenga la iteración)
    } else {
        loop {
            println!("Patron ({}):", input.len());

            let mut value = String::new();

            io::stdin()
                .read_line(&mut value)
                .expect("Error reading the pattern");

            let value = value.trim();

            if value == ".done" {
                break;
            }

            let neurons: Vec<&str> = value.split(' ').collect();
            let neurons: Vec<_> = neurons
                .iter()
                .map(|n| n.parse::<f64>().expect("Neuron value invalid"))
                .collect();

            if n == 0 {
                n = neurons.len();
            } else if neurons.len() != n {
                panic!("The pattern must has the same number of neurons");
            }

            input.push((neurons, input.len().to_string()));
        }
    }

    input
}

pub fn get_test_pattern(matches: &Matches, n: usize) -> (Vec<f64>, String) {
    return if matches.opt_present("i") {
        let mut value = String::new();

        println!("Ruta de la imagen:");

        io::stdin()
            .read_line(&mut value)
            .expect("Error reading user input");

        let value = value.trim();

        let img = image::open(value).expect("The file is not an image");

        let file_name: Vec<&str> = value.split(&['/', '\\']).collect();
        let file_name = file_name[file_name.len() - 1];

        let input_matrix = hopfield::image_transformation(&img.to_luma8().into_vec());

        if input_matrix.len() != n {
            panic!("The image must has the same number of neurons");
        }

        (input_matrix, file_name.to_string())
    } else {
        let mut value = String::new();

        println!("Patron:");

        io::stdin()
            .read_line(&mut value)
            .expect("Error reading the pattern");

        let value = value.trim();

        let neurons: Vec<&str> = value.split(' ').collect();
        let neurons: Vec<_> = neurons
            .iter()
            .map(|n| n.parse::<f64>().expect("Neuron value invalid"))
            .collect();

        if neurons.len() != n {
            panic!("The pattern must has the same number of neurons");
        }

        (neurons, "Y".to_string())
    }
}
