#[cfg(test)]
mod tests {
	use ngtools::random_hash;
	use video::*;
	use basetask::*;
	use chrono::prelude::*;

    #[test]
    fn test_ep() {
        let mut ep = Episode::new("1", "あるぴんはいます！", Status::Watched, "ep");
        ep.set_number(1);
        assert_eq!(ep.number(), 1);
    }
    #[test]
    fn test_eps() {
        let ep1 = Episode::new("1", "伝統ある古典部の再生", Status::Watched, "ep");
        let ep2 = Episode::new("2", "名誉ある古典部の活動", Status::Watched, "ep");
        let ep3 = Episode::new("3", "事情ある古典部の末裔", Status::Watched, "ep");
        let mut eps1 = Episodes::new();
        let in1 = eps1.insert(ep1.clone());
        assert_eq!(in1, None);
        let in2 = eps1.insert(ep1.clone());
        assert_eq!(in2, Some(ep1.clone()));
        eps1.insert(ep2.clone());
        eps1.insert(ep3.clone());
        let eps2 = Episodes::from_vec(vec![ep1.clone(), ep2.clone(), ep3.clone()]);
        assert_eq!(eps2, eps1);
        let out1 = eps1.pop(&1);
        assert_eq!(out1, Some(ep1.clone()));
        let out2 = eps1.pop(&2);
        assert_eq!(out2, None);
        let keys = eps1.types();
        for i in keys {
            match i {
                1 ... 3 => (),
                _ => panic!()
            };
        }
        assert_eq!(eps2.len(), 3);
    }
    #[test]
    fn test_tasks_video() {
        type Ep = Episode;
        let te = Utc::now();
        let bt1 = Video::from_details("WHITE ALBUM2", Episodes::from_vec(
                                      vec![Ep::new("1", "WHITE ALBUM", Status::Watching, "ep"), Ep::new("2", "隣り合わせのピアノとギター", Status::Watching, "ep")]), Status::Watching, te, random_hash());
        let bt2 = Video::from_details("涼宮ハルヒの憂鬱", Episodes::from_vec(
                                      vec![Ep::new("1", "満月は照らす獣を選んでる", Status::Watching, "ep"), Ep::new("2", "学園の心臓部は庭園にあり", Status::Watching, "ep")]), Status::Watching, te, random_hash());
        let bt3 = Video::from_details("BEASTARS", Episodes::from_vec(
                                      vec![Ep::new("1", "朝比奈ミクルの冒険", Status::Watching, "ep"), Ep::new("2", "涼宮ハルヒの憂鬱 I", Status::Watching, "ep")]), Status::Watching, te, random_hash());
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
}