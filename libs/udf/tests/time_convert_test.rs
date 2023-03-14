#[cfg(test)]
mod convert_time_test {
    use udf::time_convert::convert_udf_time_to_seconds;

    #[test]
    fn convert() {
        let input_test = vec![
            "1s".to_string(),
            "1".to_string(),
            "3".to_string(),
            "5".to_string(),
            "15".to_string(),
            "30".to_string(),
            "60".to_string(),
            "120".to_string(),
            "240".to_string(),
            "360".to_string(),
            "480".to_string(),
            "720".to_string(),
            "1D".to_string(),
            "3D".to_string(),
            "1W".to_string(),
        ];

        let expected = vec![
            1,
            60,
            60 * 3,  //3Min
            60 * 5,  //5Min
            60 * 15, //15Min
            60 * 30, //30Min
            60 * 60, //1h
            60 * 120,
            60 * 240,
            60 * 360,
            60 * 480,
            60 * 720,
            60 * 60 * 24, //1D
            60 * 3 * 60 * 24, //3D
            60 * 1 * 60 * 24 * 7, //1W
        ];

        let mut to_test = Vec::new();
        input_test.into_iter().for_each(|input| {
            to_test.push(convert_udf_time_to_seconds(Some(input.clone())).unwrap_or(0))
        });

        assert_eq!(to_test, expected)
    }
}
