use std::ops::Range;

use itertools::Itertools;

#[derive(Debug)]
struct Race {
    time: u64,
    record: u64,
}
impl Race {
    fn find_record_accels(&self) -> Range<u64> {
        // distance travelled (x is seconds of acceleration): x * (time - x) = distance
        // solve for record-breaking x: x^2 - time*x + record_distance
        // -> solve quadratic

        let root = ((self.time.pow(2) - 4 * self.record) as f64).sqrt();
        let res1f = (self.time as f64 + root) / 2_f64;
        let res2f = (self.time as f64 - root) / 2_f64;

        let mut lower = res1f.min(res2f).ceil() as u64;
        if self.get_distance(lower) == self.record {
            lower += 1;
        }
        let mut upper = res1f.max(res2f).floor() as u64;
        if self.get_distance(upper) == self.record {
            upper -= 1;
        }
        lower..upper + 1
    }
    fn get_distance(&self, accel_time: u64) -> u64 {
        accel_time * (self.time - accel_time)
    }
}

const TEST: &[Race] = &[
    Race { time: 7, record: 9 },
    Race {
        time: 15,
        record: 40,
    },
    Race {
        time: 30,
        record: 200,
    },
];

const INPUT: &[Race] = &[
    Race {
        time: 45,
        record: 295,
    },
    Race {
        time: 98,
        record: 1734,
    },
    Race {
        time: 83,
        record: 1278,
    },
    Race {
        time: 73,
        record: 1210,
    },
];

const INPUT2: &[Race] = &[Race {
    time: 45988373,
    record: 295173412781210,
}];

fn main() {
    let record_accels = TEST.iter().map(|r| r.find_record_accels()).collect_vec();
    for range in &record_accels {
        println!("Winning test accel durations: {:?}", range)
    }
    println!(
        "Total test race winning options (mult): {}",
        record_accels
            .iter()
            .map(|r| r.end - r.start)
            .product::<u64>()
    );

    let input_accels = INPUT.iter().map(|r| r.find_record_accels()).collect_vec();
    for range in &input_accels {
        println!("Winning input accel durations: {:?}", range)
    }
    println!(
        "Total input race winning options (mult): {}",
        input_accels
            .iter()
            .map(|r| r.end - r.start)
            .product::<u64>()
    );

    let input2_accels = INPUT2.iter().map(|r| r.find_record_accels()).collect_vec();
    for range in &input2_accels {
        println!("Winning input2 accel durations: {:?}", range)
    }
    println!(
        "Total input2 race winning options (mult): {}",
        input2_accels
            .iter()
            .map(|r| r.end - r.start)
            .product::<u64>()
    )
}
