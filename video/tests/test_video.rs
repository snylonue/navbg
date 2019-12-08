#[cfg(test)]
mod tests {
    use ngtools::Progress;
    use ngtools::random_hash;
    use ngtools::Json;
    use video::*;
    use video::episode::*;
    use basetask::*;
    use chrono::prelude::*;

    #[test]
    fn test_eps() {
        let ep1 = Episode::new("1", "season 1", "伝統ある古典部の再生", Status::Watched);
        let ep2 = Episode::new("2", "season 1", "名誉ある古典部の活動", Status::Watched);
        let ep3 = Episode::new("3", "season 1", "事情ある古典部の末裔", Status::Watched);
        let sp1 = Episode::new("11.5", "OVA", "持つべきものは", Status::Watched);
        let mut eps1 = Episodes::new();
        let in1 = eps1.insert(ep1.clone());
        assert_eq!(in1, None);
        let in2 = eps1.insert(ep1.clone());
        assert_eq!(in2, Some(ep1.clone()));
        eps1.insert(ep2.clone());
        eps1.insert(ep3.clone());
        let eps2 = Episodes::from_vec(vec![ep1.clone(), ep2.clone(), ep3.clone()]);
        assert_eq!(eps2, eps1);
        let mut eps3 = eps1.clone();
        let out1 = eps1.remove(&Epinfo::with_ep("1"));
        assert_eq!(out1, Some(ep1.clone()));
        let out2 = eps1.remove(&Epinfo::with_ep("10"));
        assert_eq!(out2, None);
        eps3.insert(sp1.clone());
        let keys = eps3.types();
        assert_eq!(keys, &vec!["season 1".to_string(), "OVA".to_string()]);
        assert_eq!(eps3.len(), 4);
        assert_eq!(eps3.watched(), 4);
        eps3.remove(&Epinfo::new("OVA", "11.5"));
        assert_eq!(eps2, eps3);
    }
    #[test]
    fn test_eps_iter() {
        let ep1 = Episode::new("1", "season 1", "伝統ある古典部の再生", Status::Watched);
        let ep2 = Episode::new("2", "season 1", "名誉ある古典部の活動", Status::Watched);
        let ep3 = Episode::new("3", "season 1", "事情ある古典部の末裔", Status::Watched);
        let vec_eps = vec![ep1.clone(), ep2.clone(), ep3.clone()];
        let eps = Episodes::from_vec(vec_eps.clone());
        for (ep, vep) in eps.iter().zip(vec_eps.iter()) {
            assert_eq!(ep, vep);
        }
        for (ep, vep) in eps.into_iter().zip(vec_eps.iter()) {
            assert_eq!(ep.chap, vep.chap);
        }
    }
    #[test]
    fn test_tasks_video() {
        type Ep = Episode;
        let te = Utc::now();
        let bt1 = Video::with_details("WHITE ALBUM2", Episodes::from_vec(
                                      vec![Ep::new("1", "season 1", "WHITE ALBUM", Status::Watching), Ep::new("2", "season 1", "隣り合わせのピアノとギター", Status::Watching)]), Status::Watching, te, random_hash());
        let bt2 = Video::with_details("涼宮ハルヒの憂鬱", Episodes::from_vec(
                                      vec![Ep::new("1", "season 1", "満月は照らす獣を選んでる", Status::Watching), Ep::new("2", "season 1", "学園の心臓部は庭園にあり", Status::Watching)]), Status::Watching, te, random_hash());
        let bt3 = Video::with_details("BEASTARS", Episodes::from_vec(
                                      vec![Ep::new("1", "season 1", "朝比奈ミクルの冒険", Status::Watching), Ep::new("2", "season 1", "涼宮ハルヒの憂鬱 I", Status::Watching)]), Status::Watching, te, random_hash());
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
        let out1 = bts4.remove(&bt3.tid());
        assert_eq!(out1, Some(bt3.clone()));
        assert_eq!(bts2, bts4);
        assert_eq!(bts4.len(), 2);
    }
    #[test]
    fn test_video() {
        let ep1 = Episode::new("1", "season 1", "伝統ある古典部の再生", Status::Watched);
        let ep2 = Episode::new("2", "season 1", "名誉ある古典部の活動", Status::Watched);
        let ep3 = Episode::new("3", "season 1", "事情ある古典部の末裔", Status::Watched);
        let sp1 = Episode::new("11.5", "OVA", "持つべきものは", Status::Watched);
        let vec_eps = vec![ep1.clone(), ep2.clone(), ep3.clone(), sp1];
        let eps = Episodes::from_vec(vec_eps.clone());
        let mut v1 = Video::new("氷菓", eps);
        let finished = v1.progress().finished();
        assert_eq!(finished, 4);
        let out1 = v1.remove(&Epinfo::with_ep("1"));
        assert_eq!(out1, Some(ep1.clone()));
        assert_eq!(v1.len(), 3);
        let in1 = v1.insert(ep1.clone());
        assert_eq!(in1, None);
        assert_eq!(v1.len(), 4);
    }
    #[test]
    fn test_tasks_video_iter() {
        let ep1 = Episode::new("1", "season 1", "伝統ある古典部の再生", Status::Watched);
        let ep2 = Episode::new("2", "season 1", "名誉ある古典部の活動", Status::Watched);
        let ep3 = Episode::new("3", "season 1", "事情ある古典部の末裔", Status::Watched);
        let sp1 = Episode::new("11.5", "OVA", "持つべきものは", Status::Watched);
        let vec_eps = vec![ep1.clone(), ep2.clone(), ep3.clone(), sp1];
        let eps = Episodes::from_vec(vec_eps.clone());
        let v1 = Video::new("氷菓", eps);
        for (i, i2) in v1.into_iter().zip(vec_eps.iter()) {
            assert_eq!(i, i2);
        }
    }
    #[test]
    fn test_json() {
        let ep1 = Episode::new("1", "season 1", "伝統ある古典部の再生", Status::Watched);
        let ep2 = Episode::new("2", "season 1", "名誉ある古典部の活動", Status::Watched);
        let ep3 = Episode::new("3", "season 1", "事情ある古典部の末裔", Status::Watched);
        let sp1 = Episode::new("11.5", "OVA", "持つべきものは", Status::Watched);
        let vec_eps = vec![ep1.clone(), ep2.clone(), ep3.clone(), sp1];
        let eps = Episodes::from_vec(vec_eps.clone());
        let v1 = Video::new("氷菓", eps);
        let json_v1 = v1.to_json().unwrap();
        let v2 = Video::from_str(&json_v1).unwrap();
        assert_eq!(v1, v2);
    }
}