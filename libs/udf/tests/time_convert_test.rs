#[cfg(test)]
mod convert_time_test {
    use udf::time_convert::convert_udf_time_to_minute;

    #[test]
    fn convert() {
        let input_test = vec![
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
            "1M".to_string(),
        ];

        let expected = vec![
            1,  //1Min
            3,  //3Min
            5,  //5Min
            15, //15Min
            30, //30Min
            60, //1h
            120,
            240,
            360,
            480,
            720,
            60 * 24,
            3 * 60 * 24,
            1 * 60 * 24 * 7,
            438291, //one month
        ];

        let mut to_test = Vec::new();
        input_test.into_iter().for_each(|input| {
            to_test.push(convert_udf_time_to_minute(input.as_str()).unwrap_or(0))
        });

        assert_eq!(to_test, expected)
    }
}
