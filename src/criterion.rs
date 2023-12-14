pub fn get_max(v: &Vec<f32>) -> f32 {
    *(v.iter().max_by(|x, y| x.partial_cmp(y).unwrap()).unwrap())
}

pub fn get_min(v: &Vec<f32>) -> f32 {
    *(v.iter().min_by(|x, y| x.partial_cmp(y).unwrap()).unwrap())
}

fn get_indeces(a: &Vec<f32>, value: f32) -> Vec<usize> {
    (0..a.len()).filter(|index| a[*index] == value).collect()
}

pub mod uncertainty {
    use crate::answer_block::profits_losses_radio::Choise;

    use super::{get_indeces, get_max, get_min};

    pub fn maximax(a: &Vec<Vec<f32>>) -> (f32, Vec<usize>) {
        let z: Vec<_> = a.iter().map(|row| get_max(row)).collect();

        let answer = get_max(&z);

        (answer, get_indeces(&z, answer))
    }

    pub fn minimax(a: &Vec<Vec<f32>>, profits_losses: Choise) -> (f32, Vec<usize>) {
        if profits_losses == Choise::Profits {
            let z: Vec<_> = a.iter().map(|row| get_min(row)).collect();

            let answer = get_max(&z);

            (answer, get_indeces(&z, answer))
        } else {
            let z: Vec<_> = a.iter().map(|row| get_max(row)).collect();

            let answer = get_min(&z);

            (answer, get_indeces(&z, answer))
        }
    }

    pub fn hurwitz(a: &Vec<Vec<f32>>, alpha: f32) -> (f32, Vec<usize>) {
        let z: Vec<_> = a
            .iter()
            .map(|row| {
                let min_a = get_min(row);
                let max_a = get_max(row);

                return alpha * max_a + (1.0 - alpha) * min_a;
            })
            .collect();

        let answer = get_max(&z);

        (answer, get_indeces(&z, answer))
    }

    pub fn savage(a: &Vec<Vec<f32>>) -> (f32, Vec<usize>) {
        let rows = a.len();

        let cols = if rows > 0 { a[0].len() } else { 0 };

        let ys: Vec<_> = (0..cols)
            .map(|col_index| {
                let y = (0..rows).map(|row_index| a[row_index][col_index]).collect();

                get_max(&y)
            })
            .collect();

        let savage_matrix: Vec<_> = a
            .iter()
            .map(|row| {
                return row
                    .iter()
                    .enumerate()
                    .map(|(index, element)| ys[index] - element)
                    .collect();
            })
            .collect();

        let z = savage_matrix.iter().map(|row| get_max(row)).collect();
        let answer = get_min(&z);

        (answer, get_indeces(&z, answer))
    }
}

pub mod risk_condition {
    use super::{get_indeces, get_max, get_min};

    pub fn bayes(a: &Vec<Vec<f32>>, p: &Vec<f32>) -> (f32, Vec<usize>) {
        let z = a
            .iter()
            .map(|row| {
                row.iter()
                    .zip(p.iter())
                    .map(|(value, probability)| value * probability)
                    .sum()
            })
            .collect();

        let answer = get_max(&z);

        (answer, get_indeces(&z, answer))
    }

    pub fn dispersion_minimization(a: &Vec<Vec<f32>>, p: &Vec<f32>) -> (f32, Vec<usize>) {
        let z_squared = a
            .iter()
            .map(|row| {
                row.iter()
                    .zip(p.iter())
                    .map(|(value, probability)| value.powi(2) * probability)
                    .sum::<f32>()
                    - row
                        .iter()
                        .zip(p.iter())
                        .map(|(value, probability)| value * probability)
                        .sum::<f32>()
                        .powi(2)
            })
            .collect();

        let answer_squared = get_min(&z_squared);

        (
            answer_squared.sqrt(),
            get_indeces(&z_squared, answer_squared),
        )
    }

    pub fn probability_maximization(
        a: &Vec<Vec<f32>>,
        p: &Vec<f32>,
        b: Option<f32>,
    ) -> Option<(f32, Vec<usize>)> {
        if b.is_none() {
            return None;
        }

        let b = b.unwrap();

        let z = a
            .iter()
            .map(|row| {
                row.iter()
                    .zip(p.iter())
                    .map(|(value, probability)| if value > &b { probability } else { &0.0 })
                    .sum()
            })
            .collect();

        let answer = get_max(&z);

        Some((answer, get_indeces(&z, answer)))
    }

    pub fn modal(a: &Vec<Vec<f32>>, p: &Vec<f32>) -> Option<(f32, Vec<usize>)> {
        let max_probability = get_max(&p);

        if p.iter().filter(|&value| *value == max_probability).count() > 1 {
            return None;
        }

        let max_probability_element_index = p
            .iter()
            .position(|&value| value == max_probability)
            .unwrap();

        let z = a
            .iter()
            .map(|row| row[max_probability_element_index])
            .collect();

        let answer = get_max(&z);

        Some((answer, get_indeces(&z, answer)))
    }
}

#[cfg(test)]
mod tests {
    mod uncertainty {
        use crate::{
            answer_block::profits_losses_radio::Choise,
            criterion::uncertainty::{hurwitz, maximax, minimax, savage},
        };

        fn generate_test_data() -> Vec<Vec<f32>> {
            vec![vec![45.0, 25.0, 50.0], vec![20.0, 60.0, 25.0]]
        }

        #[test]
        fn test_maximax() {
            let a = generate_test_data();

            assert_eq!(maximax(&a).0, 60.0, "Maximax gives incorrect result.");
        }

        #[test]
        fn test_minimax() {
            let a = generate_test_data();

            assert_eq!(
                minimax(&a, Choise::Profits).0,
                25.0,
                "Maximax gives incorrect result."
            );
        }

        #[test]
        fn test_hurwitz() {
            let a = generate_test_data();

            assert_eq!(
                hurwitz(&a, 0.3).0,
                32.5,
                "Hurwitz gives incorrect result for alpha = 0.3"
            );
        }

        #[test]
        fn test_savage() {
            let a = generate_test_data();

            assert_eq!(savage(&a).0, 25.0, "Savage gives incorrect result.");
        }
    }

    mod risk_condition {
        use crate::criterion::risk_condition::{
            bayes, dispersion_minimization, modal, probability_maximization,
        };

        fn generate_test_data() -> (Vec<Vec<f32>>, Vec<f32>) {
            let a = vec![
                vec![100_000.0, -50_000.0, -50_000.0],
                vec![-50_000.0, -50_000.0, 100_000.0],
                vec![15_000.0, 15_000.0, 0.0],
                vec![0.0, 0.0, 0.0],
            ];

            let p = vec![0.5, 0.1, 0.4];

            (a, p)
        }

        #[test]
        fn test_bayes() {
            let (a, p) = generate_test_data();

            assert_eq!(bayes(&a, &p).0, 25_000.0, "Bayes gives incorrect result.")
        }

        #[test]
        fn test_dispersion_minimization() {
            let (a, p) = generate_test_data();

            assert_eq!(
                dispersion_minimization(&a, &p).0,
                0.0,
                "Dispersion minimization gives incorrect result."
            );
        }

        #[test]
        fn test_probability_maximization() {
            let (a, p) = generate_test_data();

            assert_eq!(
                probability_maximization(&a, &p, Some(40_000.0)).unwrap().0,
                0.5,
                "Probability maximization gives incorrect result for a = 40_000."
            );

            assert_eq!(
                probability_maximization(&a, &p, Some(10_000.0)).unwrap().0,
                0.6,
                "Probability maximization gives incorrect result for a = 10_000."
            )
        }

        #[test]
        fn test_modal() {
            let (a, p) = generate_test_data();

            assert_eq!(
                modal(&a, &p).unwrap().0,
                100_000.0,
                "Modal gives incorrect result."
            )
        }
    }
}
