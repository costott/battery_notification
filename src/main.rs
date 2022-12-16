use std::{process, thread, time};

fn main(){
    loop {
        // get the battery information
        let (percentage, state) = battery_notification::get_battery_info().unwrap_or_else(|err| {
            eprintln!("{err}");
            process::exit(1);
        });
        
        // create a popup if required
        let notified =  battery_notification::popup_decision(percentage, state);
        
        // sleep until the next check for battery information
        thread::sleep(time::Duration::from_secs(match notified{
            true => 2,
            false => 120,
        }));
    }
}