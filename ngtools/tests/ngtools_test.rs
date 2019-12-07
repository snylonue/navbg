#[cfg(test)]
mod test {
	use ngtools::*;
	use serde_json;
	#[test]
    fn test_progress_finish() {
        let mut prog = Progress::new(1, 3);
        prog.finish();
        assert_eq!(prog, Progress::new(3, 3));
    }
    #[test]
    fn test_progress_set_finished() {
        let mut prog = Progress::new(1, 3);
        prog.set_finished(2).unwrap();
        assert_eq!(prog, Progress::new(2, 3));
    }
    #[test]
    #[should_panic]
    fn test_progress_set_finshed_panic() {
        let mut prog = Progress::new(1, 3);
        prog.set_finished(6).unwrap();
    }
    #[test]
    fn test_progress_set_total() {
        let mut prog = Progress::new(1, 3);
        prog.set_total(4).unwrap();
        assert_eq!(prog, Progress::new(1, 4));
    }
    #[test]
    fn test_timelen() {
        let timl = TimeLen::new(12, 13, 25);
        assert_eq!(timl.second(), 25);
        assert_eq!(timl.minute(), 13);
        assert_eq!(timl.hour(), 12);
    }
    #[test]
    fn test_timelen_simple() {
        let timl = TimeLen::new(16, 72, 93);
        assert_eq!(timl, TimeLen::new(17, 13, 33));
        let timlm = TimeLen::new(-2, -72, -93);
        assert_eq!(timlm, TimeLen::new(-3, -13, -33));
    }
    #[test]
    fn test_timelen_total_seconds() {
        let timl = TimeLen::new(23, 469, 69);
        assert_eq!(timl.total_seconds(), 111009);
        let timlm = TimeLen::new(-2, -69, -93);
        assert_eq!(timlm.total_seconds(), -11433);
    }
    #[test]
    fn test_random_hash() {
        for _ in 1..100 {
            let s1 = random_hash();
            let s2 = random_hash();
            //The test may fail
            assert_ne!(s1, s2);
        }
    }
    #[test]
    fn test_json() {
        let pg = Progress::new(0, 52);
        let json_pg = serde_json::to_string(&pg).unwrap();
        let json_pg2 = pg.to_json().unwrap();
        assert_eq!(json_pg, "{\"finished\":0,\"total\":52}");
        assert_eq!(json_pg, json_pg2);
        let fjson_pg: Progress = serde_json::from_str("{\"finished\":0,\"total\":52}").unwrap();
        assert_eq!(fjson_pg, pg);
        let til = TimeLen::new(11, 22, 33);
        let json_til = serde_json::to_string(&til).unwrap();
        let json_til2 = til.to_json().unwrap();
        assert_eq!(json_til, "{\"hour\":11,\"minute\":22,\"second\":33}");
        assert_eq!(json_til, json_til2);
        let fjson_til: TimeLen = serde_json::from_str("{\"hour\":11,\"minute\":22,\"second\":33}").unwrap();
        assert_eq!(fjson_til, til);
       }
}