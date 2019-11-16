#[cfg(test)]
mod test {
    use ngtools::*;
    use basetask::*;
    use chrono::prelude::*;
    use serde_json::*;
    use video::*;
    #[test]
    fn test_progress_finish() {
        let mut prog = Progress::new(1, 3);
        prog.finish();
        assert_eq!(prog, Progress::new(3, 3));
    }
    #[test]
    fn test_progress_set_progress() {
        let mut prog = Progress::new(1, 3);
        prog.set_progress(2).unwrap();
        assert_eq!(prog, Progress::new(2, 3));
    }
    #[test]
    #[should_panic]
    fn test_progress_set_progress_panic() {
        let mut prog = Progress::new(1, 3);
        prog.set_progress(6).unwrap();
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
        let s1 = random_hash();
        let s2 = random_hash();
        //The test may fail
        assert_ne!(s1, s2);
    }
    #[test]
    fn test_basetask() {
        let bt1 = Basetask::new("Fate/Grand Order -絶対魔獣戦線バビロニア", 0, Progress::new(5, 22));
        let bt2 = Basetask::new("Fate/Grand Order -絶対魔獣戦線バビロニア", 0, Progress::new(5, 22));
        assert_ne!(bt2, bt1);
        let te = Utc::now();
        let tid = random_hash();
        let bt3 = Basetask::from_details("WHITE ALBUM2", 0, Progress::new(4, 13), te, tid);
        let bt4 = Basetask::from_details("WHITE ALBUM2", 0, Progress::new(4, 13), te, tid);
        assert_eq!(bt4, bt3);
    }
    #[test]
    fn test_json() {
        let pg = Progress::new(0, 52);
        let json_pg = to_string(&pg).unwrap();
        assert_eq!(json_pg, "{\"finished\":0,\"total\":52}");
        let fjson_pg: Progress = from_str("{\"finished\":0,\"total\":52}").unwrap();
        assert_eq!(fjson_pg, pg);
        let til = TimeLen::new(11, 22, 33);
        let json_til = to_string(&til).unwrap();
        assert_eq!(json_til, "{\"hour\":11,\"minute\":22,\"second\":33}");
        let fjson_til: TimeLen = from_str("{\"hour\":11,\"minute\":22,\"second\":33}").unwrap();
        assert_eq!(fjson_til, til);
        let te = "2019-11-10T07:00:17.866348700Z".parse::<DateTime<Utc>>().unwrap();
        let bt = Basetask::from_details("WHITE ALBUM2", 0, Progress::new(4, 13), te, 6068359080622533981);
        let json_bt = to_string(&bt).unwrap();
        assert_eq!(json_bt, "{\"name\":\"WHITE ALBUM2\",\"priority\":0,\"progress\":{\"finished\":4,\"total\":13},\"create_time\":\"2019-11-10T07:00:17.866348700Z\",\"tid\":6068359080622533981}");
        let fjson_bt: Basetask = from_str("{\"name\":\"WHITE ALBUM2\",\"priority\":0,\"progress\":{\"finished\":4,\"total\":13},\"create_time\":\"2019-11-10T07:00:17.866348700Z\",\"tid\":6068359080622533981}").unwrap();
        assert_eq!(fjson_bt, bt);
    }
    #[test]
    fn test_tasks() {
        let te = Utc::now();
        let bt1 = Basetask::from_details("WHITE ALBUM2", 0, Progress::new(4, 13), te, random_hash());
        let bt2 = Basetask::from_details("涼宮ハルヒの憂鬱", 0, Progress::new(2, 14), te, random_hash());
        let bt3 = Basetask::from_details("BEASTARS", 0, Progress::new(1, 12), te, random_hash());
        let mut bts1 = Tasks::new();
        let in1 = bts1.insert(bt1.clone());
        assert_eq!(in1, None);
        let in2 = bts1.insert(bt1.clone());
        assert_eq!(in2, Some(bt1.clone()));
        bts1.insert(bt2.clone());
        let bts2 = Tasks::from_vec(vec![bt1.clone(),bt2.clone()]);
        assert_eq!(bts2, bts1);
        let bts3 = Tasks::from_array(&[bt1.clone(),bt2.clone()]);
        assert_eq!(bts3, bts2);
        let mut bts4 = Tasks::from_vec(vec![bt1.clone(),bt2.clone(),bt3.clone()]);
        let out1 = bts4.pop(&bt3.tid());
        assert_eq!(out1, Some(bt3.clone()));
        assert_eq!(bts2, bts4);
        assert_eq!(bts4.len(), 2);
    }
    #[test]
    fn test_ep() {
        let mut ep = Ep::new("2", "あるぴんはいます！", Status::Watched, "ep");
        ep.set_number("1");
        assert_eq!(*ep.number(), "1");
    }
    #[test]
    fn test_eps() {
        let ep1 = Ep::new("1", "伝統ある古典部の再生", Status::Watched, "ep");
        let ep2 = Ep::new("2", "名誉ある古典部の活動", Status::Watched, "ep");
        let ep3 = Ep::new("3", "事情ある古典部の末裔", Status::Watched, "ep");
        let mut eps1 = Eps::new();
        let in1 = eps1.insert(ep1.clone());
        assert_eq!(in1, None);
        let in2 = eps1.insert(ep1.clone());
        assert_eq!(in2, Some(ep1.clone()));
        eps1.insert(ep2.clone());
        eps1.insert(ep3.clone());
        let eps2 = Eps::from_vec(vec![ep1.clone(), ep2.clone(), ep3.clone()]);
        assert_eq!(eps2, eps1);
        let out1 = eps1.pop(&"1".to_string());
        assert_eq!(out1, Some(ep1.clone()));
        let out2 = eps1.pop(&"4".to_string());
        assert_eq!(out2, None);
    }
}