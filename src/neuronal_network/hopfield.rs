use getopts::Matches;

use crate::input::{get_input, get_test_pattern};
use crate::utils::{ones_standard_input, ones_matrix_transformation};

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

    // Entra en un ciclo para realizar las pruebas de acuerdo a entradas de prueba
    loop {
        let mut index: usize = 1;
        // Matriz donde se almacenarán las Y calculadas
        let mut y_matrices: Vec<(Vec<f64>, String)> = vec![];

        // Se pide la matriz de prueba
        let test_input = ones_standard_input(get_test_pattern(&matches, input[0].0.len()));

        let input_matrix = ones_matrix_transformation(&test_input.0);

        // Se almacena la matriz como Y0
        y_matrices.push((input_matrix, y_matrices.len().to_string()));

        while index <= 50 {
            let last_y_matrix = y_matrices.last().expect("Matrix empty");
            // Se evalúa la matriz de peso con la matriz prueba ya transformada
            let y_matrix = evaluation(&weight_matrix, &last_y_matrix.0);

            // Se verifica si converge Y(n) y Y(n - 1)
            if y_matrix.eq(&last_y_matrix.0) {
                println!("Y{} converge con Y{}", y_matrices.len(), y_matrices.len() - 1);

                // Busque que Y asocie con algún patrón de entrada
                for i in 0..input.len() {
                    if y_matrix.eq(&input[i].0) {
                        println!("Asocia con X{}", i);
                    }
                }

                break;
            }

            // En caso de no converger, se guarda dentro de la matriz para que sea utilizado
            // en la siguiente iteración
            y_matrices.push((y_matrix, y_matrices.len().to_string()));
            index += 1;
        }

        if index > 50 {
            println!("Entra en un bucle");
        }

        println!("Iteraciones: {}", index);
    }
}

fn create_weight_matrix(input: &Vec<(Vec<f64>, String)>) -> Vec<Vec<f64>> {
    let n: usize = input[0].0.len(); // # neuronas
    let m: usize = input.len(); // # patrones

    // Se inicializa una matriz cuadrática con valores de 0
    let mut weight_matrix: Vec<Vec<f64>> = vec![vec![0.0; n]; n];

    for i in 0..n {
        for j in 0..n {
            if i == j || weight_matrix[i][j] != 0.0 {
                continue;
            }

            let mut value: f64 = 0.0;

            // Se hace la sumatoria de la multiplicación de las neuronas
            // del patron indicado con la variable s
            for s in 0..m {
                value += input[s].0[i] * input[s].0[j];
            }

            // Se multiplica la sumatoria por 1/n
            value = (1.0 / (n as f64)) * value;

            // Guardamos el valor en la matriz con su valor espejo
            weight_matrix[i][j] = value;
            weight_matrix[j][i] = value;
        }
    }

    weight_matrix
}

// Haciendo uso de la matriz de peso generada con la función
// create_weight_matrix, genera la matriz que permitirá
// evaluar si es que converge o asocia con algún dato de entrada
fn evaluation(weight_matrix: &Vec<Vec<f64>>, x_matrix: &Vec<f64>) -> Vec<f64> {
    let n = x_matrix.len(); // # neuronas

    // Toma la matriz de prueba (x_matrix) como U0 y la pasa por la
    // función de transferencia para generar Y0
    let y_matrix = ones_matrix_transformation(&x_matrix);

    // Valor resultante de la evaluación
    let mut u_matrix_result: Vec<f64> = vec![0.0; n];

    for i in 0..n {
        let mut value: f64 = 0.0;

        for j in 0..n {
            value += weight_matrix[i][j] * y_matrix[j];
        }

        u_matrix_result[i] = value;
    }

    ones_matrix_transformation(&u_matrix_result)
}