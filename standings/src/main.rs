use chrono::prelude::*;
use std::fmt::{self};

fn main() {
    let now = Utc::now();
    let current_year = now.year();
    

    let main_url = fmt::format(format_args!("https://www.formula1.com/en/results.html/{}/", current_year));

    let driver_url = "drivers.html";
    let team_url: &str = "teams.html";

    println!("driver = {}", main_url.clone()+driver_url);
    println!("team = {}", main_url.clone()+team_url);
}
