#[cfg(test)]
mod convert_time_test {
    use udf::time_convert::convert_udf_time_to_sec;

    #[test]
    fn convert() {
        let input_test = vec![
            "100S".to_string(),
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
            100,     //100S
            1 * 60,  //1Min
            3 * 60,  //3Min
            5 * 60,  //5Min
            15 * 60, //15Min
            30 * 60, //30Min
            60 * 60, //1h
            120 * 60,
            240 * 60,
            360 * 60,
            480 * 60,
            720 * 60,
            1 * 60 * 60 * 24,
            3 * 60 * 60 * 24,
            1 * 60 * 60 * 24 * 7,
            2629746, //one month
        ];

        let mut to_test = Vec::new();
        input_test
            .into_iter()
            .for_each(|input| to_test.push(convert_udf_time_to_sec(input.as_str()).unwrap_or(0)));

        assert_eq!(to_test, expected)
    }
}
