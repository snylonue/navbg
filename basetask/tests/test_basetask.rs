#[cfg(test)]
mod tests {
	//use super::*;
	use serde_json::to_string;
use serde_json::from_str;
use ngtools::*;
	use basetask::*;
    use chrono::prelude::*;

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
    fn test_tasks_basetask() {
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
    fn test_json() {
        let te = "2019-11-10T07:00:17.866348700Z".parse::<DateTime<Utc>>().unwrap();
        let bt = Basetask::from_details("WHITE ALBUM2", 0, Progress::new(4, 13), te, 6068359080622533981);
        let json_bt = to_string(&bt).unwrap();
        let json_bt2 = bt.to_json().unwrap();
        assert_eq!(json_bt, "{\"name\":\"WHITE ALBUM2\",\"priority\":0,\"progress\":{\"finished\":4,\"total\":13},\"create_time\":\"2019-11-10T07:00:17.866348700Z\",\"tid\":6068359080622533981}");
        assert_eq!(json_bt, json_bt2);
        let fjson_bt: Basetask = from_str("{\"name\":\"WHITE ALBUM2\",\"priority\":0,\"progress\":{\"finished\":4,\"total\":13},\"create_time\":\"2019-11-10T07:00:17.866348700Z\",\"tid\":6068359080622533981}").unwrap();
        assert_eq!(fjson_bt, bt);
    }
}