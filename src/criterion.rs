fn get_max(v: &Vec<f32>) -> f32 {
    return *(v.iter().max_by(|x, y| x.partial_cmp(y).unwrap()).unwrap());
}

fn get_min(v: &Vec<f32>) -> f32 {
    return *(v.iter().min_by(|x, y| x.partial_cmp(y).unwrap()).unwrap());
}

fn get_indeces(a: &Vec<f32>, value: f32) -> Vec<usize> {
    return (0..a.len()).filter(|index| a[*index] == value).collect();
}

pub fn maximax(a: &Vec<Vec<f32>>) -> (f32, Vec<usize>) {
    let z: Vec<_> = a.iter().map(|row| get_max(row)).collect();

    let answer = get_max(&z);

    return (answer, get_indeces(&z, answer));
}

pub fn minimax(a: &Vec<Vec<f32>>) -> (f32, Vec<usize>) {
    let z: Vec<_> = a.iter().map(|row| get_min(row)).collect();

    let answer = get_max(&z);

    return (answer, get_indeces(&z, answer));
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

    return (answer, get_indeces(&z, answer));
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

    return (answer, get_indeces(&z, answer));
}

#[cfg(test)]
mod tests {
    use crate::criterion::{hurwitz, maximax, minimax, savage};

    fn generate_data() -> Vec<Vec<f32>> {
        vec![vec![45.0, 25.0, 50.0], vec![20.0, 60.0, 25.0]]
    }

    #[test]
    fn test_maximax() {
        let a = generate_data();

        assert_eq!(maximax(&a).0, 60.0, "Maximax gives incorrect result.");
    }

    #[test]
    fn test_minimax() {
        let a = generate_data();

        assert_eq!(minimax(&a).0, 25.0, "Maximax gives incorrect result.");
    }

    #[test]
    fn test_hurwitz() {
        let a = generate_data();

        assert_eq!(
            hurwitz(&a, 0.3).0,
            32.5,
            "Hurwitz gives incorrect result for alpha = 0.3"
        );
    }

    #[test]
    fn test_savage() {
        let a = generate_data();

        assert_eq!(savage(&a).0, 25.0, "Savage giver incorrect result.");
    }
}
