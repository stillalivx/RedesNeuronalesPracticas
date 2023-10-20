use getopts::Matches;

use crate::input::{get_input, get_test_pattern};
use crate::utils::{ones_standard_input};

pub fn execute(matches: &Matches) {
    // Leemos las matrices de entrada
    let raw_input: Vec<(Vec<i8>, String)> = get_input(&matches);
    let mut input: Vec<(Vec<f64>, String)> = vec![];

    // Normalizamos los valores de la matriz, obteniendo una matriz de valores 1 o -1
    for item in raw_input {
        input.push(ones_standard_input(item));
    }

    // En base a las entradas creamos la matriz de peso
    let weight_matrix = create_weight_matrix(&input);

    if matches.opt_present("show-weight-matrix") {
        println!("{:?}", weight_matrix);
    }

    let n = input.len();

    loop {
        // Se pide matriz de prueba
        let test_pattern = ones_standard_input(get_test_pattern(&matches, input[0].0.len()));

        // Hacemos la evaluación de la matriz de prueba en conjunto con la
        // matriz de peso
        let evaluation = evaluation(&weight_matrix, &test_pattern.0);
        let mut reps = 1;

        println!("{:?}", test_pattern);

        // Guardamos el resultado de la evaluación dentro de una matriz
        // con el identificador "0"
        let mut last_matrix: Vec<f64> = evaluation;

        // Entramos en un ciclo en busca de converger y asociar. En caso de que esto no sea
        // posible en menos de 50 iteraciones, el programa da por hecho que la red neuronal
        // entró en un ciclo infinito y deja de ejecutarse la iteración
        while reps < 50 {
            let mut u = vec![0.0; n];
            // Tomamos la última matriz calculada
            let evaluation = &last_matrix;

            for i in 0..n {
                let mut value = 0.0;

                for j in 0..n {
                    // Nos saltamos el valor de la variable dependiendo del
                    // índice de u que se esté calculando. En caso de que sea diferente
                    // se sumará
                    value += if j != i {
                        evaluation[j]
                    } else {
                        0.0
                    }
                }

                // Se aplica formula para calcular el valor del índice de u
                u[i] = neuron_transformation(evaluation[i] - (1.0/((n as f64) - 1.0)) * value);
            }

            let mut num_pos = 0;
            let mut idx: usize = 0;

            // Se verifica que solo haya un valor positivo sumando la cantidad de veces
            // que se encuentra uno y guardando el índice donde se encuentra
            for i in 0..n {
                if u[i] > 0.0 {
                    idx = i + 1;
                    num_pos += 1;
                }
            }

            println!("{:?}", u);

            // Si hay solo un numero positivo, esto quiere decir que la red neuronal
            // convergió y asocia con el índice del número positivo dentro de la matriz
            // u que se esta calculando
            if num_pos == 1 {
                println!("Converge y asocia con {}", idx);
                break;
            } else {
                println!("No converge");
            }

            // Si no converge, la matriz se guarda para ser usado en la siguiente iteración.
            last_matrix = u;
            reps += 1;
        }

        println!("Iteraciones: {}", reps);
    }
}

fn create_weight_matrix(input: &Vec<(Vec<f64>, String)>) -> Vec<Vec<f64>> {
    let n: usize = input[0].0.len();
    let m: usize = input.len();

    let mut weight_matrix: Vec<Vec<f64>> = vec![vec![0.0; n]; m];

    for i in 0..m {
        for j in 0..n {
            // La matriz de prueba se calcula dividiendo todos las neuronas de
            // los patrones entre dos
            weight_matrix[i][j] = input[i].0[j] / 2.0;
        }
    }

    weight_matrix
}

// Calcula la bios, utilizada para la evaluación
fn get_bios(n: usize, m: usize) -> Vec<f64> {
    vec![(n as f64)/2.0; m]
}

// Evalúa una matriz de prueba haciendo uso de la matriz de peso
fn evaluation(weight_matrix: &Vec<Vec<f64>>, x_matrix: &Vec<f64>) -> Vec<f64> {
    let n = x_matrix.len();
    let m = weight_matrix.len();
    let bios = get_bios(n, m);

    let mut u_matrix_result: Vec<f64> = vec![0.0; m];

    for i in 0..m {
        let mut value: f64 = 0.0;

        for j in 0..n {
            value += weight_matrix[i][j] * x_matrix[j];
        }

        u_matrix_result[i] = value;
    }

    for i in 0..m {
        u_matrix_result[i] = (u_matrix_result[i] + bios[i]) * (1.0/(n as f64));
    }

    u_matrix_result
}

fn neuron_transformation(value: f64) -> f64 {
    return if value < 0.0 {
        0.0
    } else if value >= 1.0 {
        1.0
    } else {
        value
    }
}