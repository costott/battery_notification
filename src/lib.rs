use std::ptr::null_mut as NULL;
use winapi::um::winuser;

/// Gets the `battery percentage` (between 0.0 and 1.0) and `state` of the battery
/// 
/// # Errors
/// - returns an error if a battery manager can't be created
/// - return an error if the battery manager's batteries can't be found
/// - returns an error if the battery manager's battery has an error
pub fn get_battery_info() -> Result<(f32, battery::State), battery::Error> {
    let manager = battery::Manager::new()?;
    
    for (_idx, maybe_battery) in manager.batteries()?.enumerate() {
        let battery = maybe_battery?;
        return Ok((battery.state_of_charge().value, battery.state()))
    }

    Ok((0f32, battery::State::Unknown)) // shouldn't happen
}

/// Creates a popup if the battery information needs an alert
/// 
/// Returns whether a popup was given or not
/// 
/// # Examples
/// ```
/// use battery_notification::popup_decision;
/// 
/// assert_eq!(true, popup_decision(0.3, battery::State::Discharging));
/// assert_eq!(false, popup_decision(0.55, battery::State::Discharging));
/// assert_eq!(true, popup_decision(0.9, battery::State::Charging))
/// ```
pub fn popup_decision(percentage: f32, state: battery::State) -> bool {
    if percentage <= 0.4f32 && state == battery::State::Discharging {
        open_popup_window(format!("Battery at {}% - plug in charger", (percentage*100f32).round()));
        true
    } else if percentage >= 0.8f32 && state == battery::State::Charging {
        open_popup_window(format!("Battery at {}% - take out charger", (percentage*100f32).round()));
        true
    } else {
        false
    }
}

/// Creates a popup with the given prompt
fn open_popup_window(prompt: String) {
    let l_msg: Vec<u16> = (prompt + "\0").encode_utf16().collect();
    let l_title: Vec<u16> = "Battery notification\0".encode_utf16().collect();

    unsafe {
        winuser::MessageBoxW(NULL(), l_msg.as_ptr(), l_title.as_ptr(), winuser::MB_OK | winuser::MB_ICONINFORMATION);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_popup_charging() {
        assert_eq!(false, popup_decision(0.1, battery::State::Charging));
        assert_eq!(false, popup_decision(0.5, battery::State::Charging));
    }

    #[test]
    fn no_popup_discharging() {
        assert_eq!(false, popup_decision(0.9, battery::State::Discharging));
        assert_eq!(false, popup_decision(0.5, battery::State::Discharging));
    }

    #[test]
    fn popup_discharging() {
        assert_eq!(true, popup_decision(0.38, battery::State::Discharging));
    }

    #[test]
    fn popup_charging() {
        assert_eq!(true, popup_decision(0.88, battery::State::Charging));
    }
}