pub fn parse_data(a: &Vec<Vec<String>>) -> Result<Vec<Vec<f32>>, &str> {
    let mut parsed_data = Vec::with_capacity(a.len());

    for row in a {
        let mut parsed_row = Vec::with_capacity(row.len());
        for cell in row {
            if let Ok(parsed_cell_value) = cell.parse::<f32>() {
                parsed_row.push(parsed_cell_value);
            } else {
                return Err("Matrix is invalid.");
            }
        }

        parsed_data.push(parsed_row);
    }

    return Ok(parsed_data);
}

#[cfg(test)]
mod tests {
    use crate::utils::parse_data;

    #[test]
    fn test_parse_data() {
        let a = vec![
            vec!["1".to_string(), "2.3".to_string(), "3.4".to_string()],
            vec!["-1.0".to_string(), "2".to_string(), "0".to_string()],
        ];

        let parsed_a: Vec<Vec<f32>> = vec![vec![1.0, 2.3, 3.4], vec![-1.0, 2.0, 0.0]];

        assert_eq!(parse_data(&a), Ok(parsed_a), "Data parsed incorrectly.");
    }

    #[test]
    fn test_parse_data_invalid_f32() {
        let a = vec![
            vec!["1.1.".to_string(), "2.3".to_string(), "3.4".to_string()],
            vec!["-1.0".to_string(), "2".to_string(), "0".to_string()],
        ];

        assert_eq!(parse_data(&a), Err("Matrix is invalid."), "First value in matrix is invalid float.");
    }

    #[test]
    fn test_parse_data_letters() {
        let a = vec![
            vec!["a".to_string(), "2.3".to_string(), "3.4".to_string()],
            vec!["-1.0".to_string(), "a".to_string(), "".to_string()],
        ];

        assert_eq!(parse_data(&a), Err("Matrix is invalid."), "First value in matrix is letter.");
    }

    #[test]
    fn test_parse_data_empty_space() {
        let a = vec![
            vec!["".to_string(), "2.3".to_string(), "3.4".to_string()],
            vec!["-1.0".to_string(), "a".to_string(), "".to_string()],
        ];

        assert_eq!(parse_data(&a), Err("Matrix is invalid."), "First value in matrix is empty space.");
    }
}
