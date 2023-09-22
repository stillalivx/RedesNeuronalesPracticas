use std::{io, vec};

fn main() {
  let mut n: String = String::new();
  let mut m: String = String::new();

  println!("# Neuronas (N):");
  
  io::stdin()
    .read_line(&mut n)
    .expect("Failed to read line");

  let n: usize = n.trim().parse().expect("You must provide a integer value");

  println!("# Patrones (M):");

  io::stdin()
    .read_line(&mut m)
    .expect("Failed to read line");

  let m: usize = m.trim().parse().expect("You must provide a integer value");

  let mut c: Vec<Vec<f32>> = vec![vec![0.0; n]; m];

  'm_loop: for index_m in 0..m {
    for index_n in 0..n {
      let mut value = String::new();

      println!("Para C{}-{index_n}:", index_m + 1);

      io::stdin()
        .read_line(&mut value)
        .expect("Failed to read line");

      let value: f32 = value.trim().parse().expect("You must provide a integer value");

      if value != -1.0 && value != 1.0 {
        break 'm_loop;
      }

      c[index_m][index_n] = value;
    }
  }

  let mut result_matrix: Vec<Vec<f32>> = vec![vec![0.0; n]; n];

  for i in 0..n {
    for j in 0..n {
      if i == j || result_matrix[i][j] != 0.0 {
        continue;
      }

      let mut value: f32 = 0.0;

      for s in 0..m {
        value += c[s][i] * c[s][j];

        println!("{} * {} = {}", c[s][i], c[s][j], c[s][i] * c[s][j]);
      }

      
      value = value * (1.0/(n as f32)) ;

      println!("Value: {}", value);

      result_matrix[i][j] = value;
      result_matrix[j][i] = value;
    }
  }

  println!("{:?}", result_matrix);
}
