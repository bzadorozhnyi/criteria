fn get_max(v: &Vec<f32>) -> f32 {
    return *(v.iter().max_by(|x, y| x.partial_cmp(y).unwrap()).unwrap());
}

fn get_min(v: &Vec<f32>) -> f32 {
    return *(v.iter().min_by(|x, y| x.partial_cmp(y).unwrap()).unwrap());
}

fn maximax(a: &Vec<Vec<f32>>) -> f32 {
    let z: Vec<_> = a.iter().map(|row| get_max(row)).collect();

    return get_max(&z);
}

fn minimax(a: &Vec<Vec<f32>>) -> f32 {
    let z: Vec<_> = a.iter().map(|row| get_min(row)).collect();

    return get_max(&z);
}

fn hurwitz(a: &Vec<Vec<f32>>, alpha: f32) -> Result<f32, &str> {
    if 0.0 <= alpha && alpha <= 1.0 {
        let z: Vec<_> = a
            .iter()
            .map(|row| {
                let min_a = get_min(row);
                let max_a = get_max(row);

                return alpha * max_a + (1.0 - alpha) * min_a;
            })
            .collect();

        return Ok(get_max(&z));
    } else {
        Err("Alpha must be in range [0; 1].")
    }
}

fn savage(a: &Vec<Vec<f32>>) -> f32 {
    let savage_matrix: Vec<_> = a
        .iter()
        .map(|row| {
            let max_in_row = get_max(row);
            return row.iter().map(|element| max_in_row - element).collect();
        })
        .collect();

    let z = savage_matrix.iter().map(|row| get_max(row)).collect();

    return get_min(&z);
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

        assert_eq!(maximax(&a), 60.0, "Maximax gives incorrect result.");
    }

    #[test]
    fn test_minimax() {
        let a = generate_data();

        assert_eq!(minimax(&a), 25.0, "Maximax gives incorrect result.");
    }

    #[test]
    fn test_hurwitz() {
        let a = generate_data();

        assert_eq!(
            hurwitz(&a, 0.3),
            Ok(32.5),
            "Hurwitz gives incorrect result for alpha = 0.3"
        );

        assert_eq!(
            hurwitz(&a, 0.8),
            Ok(52.0),
            "Hurwitz gives incorrect result for alpha = 0.8"
        );

        assert_eq!(
            hurwitz(&a, -1.0),
            Err("Alpha must be in range [0; 1]."),
            "Hurwitz cannot give rust for alpha < 0"
        );

        assert_eq!(
            hurwitz(&a, 10.0),
            Err("Alpha must be in range [0; 1]."),
            "Hurwitz cannot give rust for alpha > 1"
        );
    }

    #[test]
    fn test_savage() {
        let a = generate_data();

        assert_eq!(savage(&a), 25.0, "Savage giver incorrect result.");
    }
}
