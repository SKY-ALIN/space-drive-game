fn main() {
    const TIME_STEP: f64 = 0.1;
    let mut time: f64 = 0.35;

    loop {
        let mut timedelta = TIME_STEP;
        if time > TIME_STEP {
            time -= timedelta;
        } else if time <= TIME_STEP && time > 0.0 {
            timedelta = time;
            time -= timedelta;
        } else {
            break;
        }

        time = (time * 100.0).round() / 100.0;
        println!("{}", timedelta);
    }
}
