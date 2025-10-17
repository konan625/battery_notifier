use battery::Manager; //imports the battery manager from the battery crate for accessing battery information
use notify_rust::Notification; // this will import the desktop notification functionality from the notify_rust crate
use std::{thread,time}; //for threading and time operations

fn main() {
    // println!("Hello, world!");
    let manager=Manager::new().expect("Failed to create battery manager"); //create a new battery manager instance, .expect()  will panic with the error message if creation fails
    loop{
        let batteries = manager.batteries().expect("Could not access batteries"); // retrieves information about all available batteries, returns an iterator over battery results, panics if battery access fails
        for maybe_battery in batteries{ //iterates through all detected batteries maybe_battery is a result type that could contain either a battery or an error.
            if let Ok(battery) =maybe_battery{ //pattern matching , if the battery was successfully retrieved (Ok variant), extract the battery object
                let percent = (battery.state_of_charge().value*100.0) as u8; //cast to unsigned 8 bit interger
                let plugged_in = matches!(battery.state(),battery::State::Charging | battery::State::Full); // uses matches! macro to check if battery is either charging or full, return true if plugged_in

                if plugged_in && percent>=90 {
                    Notification::new()
                        .summary("Battery FUll 🔥")
                        .body("Charging complete! Unplug now 🔋")
                        .show()
                        .unwrap();

                    thread::sleep(time::Duration::from_secs(5)) //wait a 10min before next alert
                }

            }
        }

        thread::sleep(time::Duration::from_secs(30)); //pause for 30 seconds between battery checks
    }
}
