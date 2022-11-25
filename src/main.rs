use rand::{Rng};
use std::{thread, time};
use std::time::Instant;


struct User {
    click_interval: f32, // Seconds
    random_click: f32, // Random Click: 0 for no random time intervals, +j for random time intervals 
}

trait Controls {
    fn left_click(&self);
}

impl Controls for User {
    fn left_click(&self) {
        if self.random_click == 0.0 {return};

        let now = Instant::now(); // Get the current time
        let half_interval = self.random_click as f32 / 2.0; // Get random_click / 2 as a float value
        let random_time = rand::thread_rng().gen_range(self.random_click-half_interval .. self.random_click+half_interval); // Generate a random number from random_click - random to + random
        let duration = time::Duration::new((self.click_interval + random_time) as  u64, 0);

        thread::sleep(duration); // Sleep for passed interval time duration
        println!("Data: \nSlept For: {:?}\nHalf Interval: {}\nRandom_time: {}", now.elapsed(), half_interval, random_time);
    }
}


fn main() {
    let main_user = User{click_interval: 5.0, random_click: 1.0};
    main_user.left_click();
    println!("Hello, world!");
}
