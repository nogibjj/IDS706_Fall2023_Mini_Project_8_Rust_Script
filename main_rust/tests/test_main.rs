#[cfg(test)]

fn data_filter(df: &Vec<Vec<String>>) -> Vec<Vec<String>> {
    let mut filtered_data = Vec::new();
    for row in df.iter() {
        println!("{:?}", row[1]);
        if row[1].trim_start().parse::<f32>().unwrap_or(0.0) >= 80.0 {
            filtered_data.push(row.clone());
        }
    }
    filtered_data
}

mod tests {
    use crate::data_filter;
    #[test]
    fn test_data_filter() {
        let mock_csv_file = vec![
            vec!["Alice".to_string(), "83".to_string()],
            vec!["Bob".to_string(), "82".to_string()],
            vec!["Charlie".to_string(), "81".to_string()],
            vec!["David".to_string(), "80".to_string()],
            vec!["Eve".to_string(), "79".to_string()],
            vec!["Frank".to_string(), "78".to_string()],
        ];
        let res = data_filter(&mock_csv_file);
        assert_eq!(res.len(), 4);
    }
}