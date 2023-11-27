use getopts::Matches;
use rand;
use rand::Rng;
use std::io;

use crate::input::{get_input, get_test_pattern};
use crate::utils::{ones_standard_input};

pub fn execute(matches: &Matches) {
    let is_image_input = matches.opt_present("i");

    println!("Patrones de entrada:");

    // Leemos las matrices de entrada
    let raw_input: Vec<(Vec<i8>, String)> = get_input(&matches);
    let mut input: Vec<(Vec<f64>, String)> = vec![];

    for i in 0..raw_input.len() {
        let mut single_input: Vec<f64> = vec![];

        for j in 0..raw_input[i].0.len() {
            single_input.push(raw_input[i].0[j].into());
        }

        input.push((single_input, raw_input[i].1.clone()));
    }

    let n_patterns = input.len();
    let n_inputs = input[0].0.len();

    let raw_output = get_test_pattern(&matches, n_patterns);
    let mut output: Vec<f64> = vec![];

    for item in raw_output.0 {
        output.push(item.into());
    }

    println!("Matriz de peso:");

    let mut value = String::new();

    io::stdin()
        .read_line(&mut value)
        .expect("Error reading the pattern");

    let value = value.trim();

    let neurons: Vec<&str> = value.split(' ').collect();
    // Convertimos la entrada a valores f64
    let mut weight_matrix: Vec<f64> = neurons
        .iter()
        .map(|n| n.parse::<f64>().expect("Neuron value invalid"))
        .collect();

    if weight_matrix.len() != n_inputs + 1 {
        panic!("The pattern must has the same number of neurons");
    }

    let mut alpha = String::new();

    println!("Valor de alfa:");

    io::stdin()
        .read_line(&mut alpha)
        .expect("Alfa invalido");

    let alpha: f64 = alpha.trim().parse().expect("Alfa invalido");

    let bias: f64 = 1.0;

    for i in 0..n_patterns {
        input[i].0.insert(0, bias);
    }

    let n_inputs = input[0].0.len();

    let mut i = 0;
    let mut error: Vec<f64> = vec![0.0; n_patterns];

    loop {
        println!("Matriz de entrada: {:?}", input[i].0);

        let mut y: f64 = 0.0;

        for j in 0..n_inputs {
            y += input[i].0[j] * weight_matrix[j];

            println!("i * w = {} * {} = {}", input[i].0[j], weight_matrix[j], y);
        }

        println!("Valor Y: {}", y);

        y = if y >= 0.0 { 1.0 } else { 0.0 };

        println!("Valor fh(Y) : {}", y);

        let error = output[i] - y;

        println!("error = {} - {} = {}", y, output[i], error);

        if error != 0.0 {
            let actual_input = input[i].0.clone();

            for j in 0..n_inputs {
                weight_matrix[j] = actual_input[j] * alpha * error + weight_matrix[j];
            }

            println!("Matriz de peso nueva: {:?}", weight_matrix);

            continue;
        }

        if i == n_patterns - 1 {
            break;
        } else {
            i += 1;
        }
    }

    let mut sum_prom: f64 = 0.0;

    for i in 0..n_patterns {
        sum_prom += error[i];
    }
}

fn create_weight_matrix(n_input: usize) -> Vec<f64> {
    let mut weight_matrix: Vec<f64> = vec![0.0; n_input + 1];

    for i in 0..(n_input + 1) {
        weight_matrix[i] = rand::thread_rng().gen_range(-1.0..1.0);
    }

    weight_matrix
}