#[cfg(test)]
mod test {
	use ngtools::*;
	use basetask::*;
	use chrono::prelude::*;
	#[test]
	fn test_progress_finish() {
		let mut prog = Progress::new(1, 3);
		prog.finish();
		assert_eq!(prog, Progress {finished:3, total: 3});
	}
	#[test]
	fn test_progress_set_progress() {
		let mut prog = Progress::new(1, 3);
		prog.set_progress(2).unwrap();
		assert_eq!(prog, Progress {finished: 2, total: 3});
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
		assert_eq!(prog, Progress {finished: 1, total:4});
	}
	#[test]
	fn test_timelen_simple() {
		let timl = TimeLen::new(16, 72, 93);
		assert_eq!(timl, TimeLen {hour: 17, minute: 13, second: 33});
		let timlm = TimeLen::new(-2, -72, -93);
		assert_eq!(timlm, TimeLen {hour: -3, minute: -13, second: -33});
	}
	#[test]
	fn test_timelen_seconds() {
		let timl = TimeLen::new(23, 469, 69);
		assert_eq!(timl.seconds(), 111009);
		let timlm = TimeLen::new(-2, -69, -93);
		assert_eq!(timlm.seconds(), -11433);
	}
	#[test]
	fn test_random_hash() {
		let s1 = random_hash();
		let s2 = random_hash();
		assert_ne!(s1, s2);
	}
	#[test]
	fn test_basetask() {
		let bt1 = Basetask::new("Fate/Grand Order -絶対魔獣戦線バビロニア".to_string(), 0, Progress::new(5, 22));
		let bt2 = Basetask::new("Fate/Grand Order -絶対魔獣戦線バビロニア".to_string(), 0, Progress::new(5, 22));
		assert_ne!(bt2, bt1);
		let te = Utc::now();
		let tid = random_hash();
		let bt3 = Basetask::from_details("WHITE ALBUM2".to_string(), 0, Progress::new(4, 13), te, tid);
		let bt4 = Basetask::from_details("WHITE ALBUM2".to_string(), 0, Progress::new(4, 13), te, tid);
		assert_eq!(bt4, bt3);
	}
}