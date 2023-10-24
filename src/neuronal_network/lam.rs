use getopts::Matches;

use crate::input::{get_input, get_test_pattern};
use crate::utils::{ones_matrix_transformation, ones_standard_input, zeros_matrix_transformation};

pub fn execute(matches: &Matches) {
    let is_image_input = matches.opt_present("i");

    println!("Patrones de entrada:");

    // Leemos las matrices de entrada
    let raw_input: Vec<(Vec<i8>, String)> = get_input(&matches);

    println!("Patrones de salida:");

    let raw_output: Vec<(Vec<i8>, String)> = get_input(&matches);

    if raw_output.len() != raw_input.len() {
        panic!("The input length is not equal to output length");
    }

    let mut input: Vec<(Vec<f64>, String)> = vec![];
    let mut output: Vec<(Vec<f64>, String)> = vec![];

    // Normalizamos los valores de la matriz de entrada, obteniendo una matriz de valores 1 o -1
    for item in raw_input {
        input.push(ones_standard_input(item, is_image_input));
    }

    // Normalizamos los valores de la matriz de salida.
    for item in raw_output {
        output.push(ones_standard_input(item, is_image_input));
    }

    let n_input = input[0].0.len();
    let n_output = output[0].0.len();

    // Se calcula la matriz de peso
    let weight_matrix = create_weight_matrix(&input, &output);

    if matches.opt_present("show-weight-matrix") {
        println!("{:?}", weight_matrix);
    }

    // Se calcula la bias
    let bias = get_bias(&weight_matrix, n_input, n_output);

    // Empieza un ciclo para recibir pruebas de entrada y que se encuentre con
    // cual salida estos convergen
    loop {
        // Se pide la entrada entrada de prueba y se estandariza
        let x_matrix = ones_standard_input(get_test_pattern(&matches, n_input), is_image_input);
        // Se calcula la matriz resultado haciendo uso de la matriz peso y la bias
        let result_matrix = evaluation(&weight_matrix, &bias, n_input, n_output, &x_matrix.0);

        // Entra a un ciclo para encontrar con cual matriz de salida
        // la recién matriz calcula llega a converger
        for i in 0..n_output {
            if result_matrix.eq(&output[i].0) {
                println!("La entrada converge con la salida {}", output[i].1);
                break;
            }
        }
    }
}

fn create_weight_matrix(
    input: &Vec<(Vec<f64>, String)>,
    output: &Vec<(Vec<f64>, String)>,
) -> Vec<Vec<f64>> {
    let n_input = input[0].0.len();
    let n_output = input[0].0.len();

    let m = input.len();

    let mut weight_matrix: Vec<Vec<f64>> = vec![vec![0.0; n_output]; n_input];

    for i in 0..n_input {
        for j in 0..n_output {
            let mut value: f64 = 0.0;

            for s in 0..m {
                // Se calcula el valor de la posición de la matriz de peso con la
                // siguiente formula
                value += (2.0 * input[s].0[i] - 1.0) * (2.0 * output[s].0[j] - 1.0);
            }

            weight_matrix[i][j] = value;
        }
    }

    weight_matrix
}

// Calcula la bios utilizada para la evaluación
fn get_bias(weight_matrix: &Vec<Vec<f64>>, n_input: usize, n_output: usize) -> Vec<f64> {
    let mut bias: Vec<f64> = vec![0.0; n_output];

    for i in 0..n_output {
        let mut value: f64 = 0.0;

        for j in 0..n_input {
            // Se debe de sumar dependiendo de si la matriz se encuentra invertida
            value += if weight_matrix.len() != n_output {
                // Matriz invertida
                weight_matrix[j][i]
            } else {
                // Matriz normal
                weight_matrix[i][j]
            }
        }

        // Se guarda el valor de la bias después de que el valor haya sido
        // multiplicado por -1/2
        bias[i] = (-1.0 / 2.0) * value;
    }

    bias
}

// Haciendo uso de la matriz de peso generada con la función
// create_weight_matrix, genera la matriz que permitirá
// evaluar si es que converge o asocia con algún dato de entrada
fn evaluation(
    weight_matrix: &Vec<Vec<f64>>,
    bias: &Vec<f64>,
    n_input: usize,
    n_output: usize,
    x_matrix: &Vec<f64>,
) -> Vec<f64> {
    let mut result: Vec<f64> = vec![0.0; n_output];

    for i in 0..n_output {
        for j in 0..n_input {
            // Se calcula al matriz resultado dependiendo de si la
            // matriz se encuentra invertida
            result[i] = result[i]
                + (x_matrix[j]
                    * if weight_matrix.len() != n_output {
                        weight_matrix[j][i]
                    } else {
                        weight_matrix[i][j]
                    });
        }

        result[i] += bias[i];
    }

    // Dependiendo de los valores que contenga la matriz de prueba, la matriz resultado
    // cambiará entre convertir el resultado en 0 y 1; o -1 y 1.
    return if x_matrix.contains(&0.0) && x_matrix.contains(&1.0) {
        zeros_matrix_transformation(&result)
    } else {
        ones_matrix_transformation(&result)
    };
}
