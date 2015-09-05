use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    // We are keeping last 100 valid events
    // And attack for us means more than 10x
    // Difference between trendline and real event
    let mut monitor = Monitor::new(100, 10.0);

    // Reading from file for simplicity
    let mut file = File::open("input.txt").unwrap();
    let mut text = String::new();

    file.read_to_string(&mut text).unwrap();

    // Assuming that data is valid and making no checks
    // Don't like this but format is not very handy
    // Removing "[[" and "]]" at start and end
    text = text.trim().to_string();
    text.remove(0);
    text.remove(0);
    text.pop();
    text.pop();

    let mut attacks = vec![];
    let mut epoch = None;
    let mut requests = None;
    for tuple in text.split("],[") {
        for value in tuple.split(",") {
            if epoch.is_none() {
                epoch = Some(value.trim().parse().unwrap());
            } else {
                requests = Some(value.trim().parse().unwrap());
            }
        }

        if monitor.add(Data{
            epoch: epoch.unwrap(),
            requests: requests.unwrap(),
        }) {
            attacks.push(epoch.unwrap());
        }

        epoch = None;
        requests = None;
    }

    let mut start = 0;
    let mut end = None;

    print!("[");
    for attack in attacks {
        match end {
            Some(e) => {
                if attack - e > 1 {
                    print!("[{}, {}], ", start, e);
                } else {
                    end = Some(attack);
                    continue;
                }
            },
            None => {}
        }

        start = attack;
        end = Some(attack);
    }

    if end.is_some() {
        print!("[{}, {}]", start, end.unwrap());
    }

    print!("]")
}

#[derive(Eq, PartialEq, PartialOrd)]
struct Data {
    epoch: usize,
    requests: usize,
}

impl Ord for Data {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.epoch == other.epoch {
            Ordering::Equal
        } else if self.epoch > other.epoch {
            // BinaryHeap is max heap
            // So we invert the order
            // To extract the oldest
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}

struct Monitor {
    size: usize,
    diff: f64,
    data: BinaryHeap<Data>,
}

impl Monitor {
    fn new(size: usize, diff: f64) -> Monitor {
        Monitor {
            size: size,
            diff: diff,
            data: BinaryHeap::with_capacity(size),
        }
    }

    fn add(&mut self, new_data: Data) -> bool {
        let y = self.calc_y(new_data.epoch as f64);
        if y > 0.0 && new_data.requests > (y * self.diff) as usize {
            // If difference between what we expect and real data
            // is bigger than configured value
            // then this data is marked as attack
            // For simplicity we don't track attacks
            return true;
        }

        // Extract old data and make a realtime detection
        if self.data.len() == self.size {
            self.data.pop();
        }

        self.data.push(new_data);
        false
    }

    // Using trendline to determine attack
    // Some problems can be if dataset starts with ddos
    // We will miss them but when traffic will be normal
    // Next attack will be detected
    fn calc_y(&self, epoch: f64) -> f64 {
        let mut count = 0.0;
        let mut x_sum = 0.0;
        let mut y_sum = 0.0;
        let mut a = 0.0;
        let mut c = 0.0;

        for item in &self.data {
            count += 1.0;
            a += (item.epoch * item.requests) as f64;
            x_sum += item.epoch as f64;
            y_sum += item.requests as f64;
            c += (item.epoch * item.epoch) as f64;
        }

        if count == 0.0 {
            return 0.0;
        }

        a *= count;
        let b = x_sum * y_sum;
        c *= count;
        let d = x_sum * x_sum;
        let m = (a - b) / (c - d);
        let e = m * x_sum;
        let f = (y_sum - e) / count;

        m * epoch + f
    }
}
