use getopts::Matches;
use std::{fs, io};

pub fn get_input(matches: &Matches) -> Vec<(Vec<i8>, String)> {
    let mut input: Vec<(Vec<i8>, String)> = vec![];
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

                // Convertimos la imágen en un Vec<u8> para posteriormente
                // convertirlo en Vec<i8>
                let input_matrix: Vec<i8> =
                    img.to_luma8().into_vec().iter().map(|&p| p as i8).collect();

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

                // Convertimos la imágen en un Vec<u8> para posteriormente
                // convertirlo en Vec<i8>
                let input_matrix: Vec<i8> =
                    img.to_luma8().into_vec().iter().map(|&p| p as i8).collect();

                let input_test = img.to_luma8().into_vec();

                println!("{:?}", input_test);

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
            // Convertimos la entrada a valores i8
            let neurons: Vec<_> = neurons
                .iter()
                .map(|n| n.parse::<i8>().expect("Neuron value invalid"))
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

pub fn get_test_pattern(matches: &Matches, n: usize) -> (Vec<i8>, String) {
    // En caso de que -i haya sido invocado se esperará que el patrón de prueba
    // sea la ruta de una imágen
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

        // Convertimos la imágen en un Vec<u8> para posteriormente
        // convertirlo en Vec<i8>
        let input_matrix: Vec<i8> = img.to_luma8().into_vec().iter().map(|&p| p as i8).collect();

        if input_matrix.len() != n {
            panic!("The image must has the same number of neurons");
        }

        (input_matrix, file_name.to_string())
    // En caso de que no se espere que el patrón de prueba no sea una imágen
    // se esperará la matriz como entrada
    } else {
        let mut value = String::new();

        println!("Patron:");

        io::stdin()
            .read_line(&mut value)
            .expect("Error reading the pattern");

        let value = value.trim();

        let neurons: Vec<&str> = value.split(' ').collect();
        // Convertimos la entrada a valores i8
        let neurons: Vec<_> = neurons
            .iter()
            .map(|n| n.parse::<i8>().expect("Neuron value invalid"))
            .collect();

        if neurons.len() != n {
            panic!("The pattern must has the same number of neurons");
        }

        (neurons, "Y".to_string())
    };
}
