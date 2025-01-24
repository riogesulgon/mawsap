use enigo::{Enigo, Settings, Mouse, Coordinate};  
use rand::Rng;  
use std::{thread, time::Duration};  
use device_query::{DeviceQuery, DeviceState, Keycode};  

fn main() -> Result<(), Box<dyn std::error::Error>> {  
    let mut enigo = Enigo::new(&Settings::default()).map_err(|e| e.to_string())?;  
    let mut rng = rand::thread_rng();  
    let device_state = DeviceState::new();  
    
    println!("Mouse jiggler started. Press ESC to exit...");  
    
    loop {  
        // Check for ESC key press  
        let keys = device_state.get_keys();  
        if keys.contains(&Keycode::Escape) {  
            println!("ESC pressed. Exiting...");  
            break;  
        }  

        // Generate random movement deltas (-1 to 1 pixels)  
        let delta_x: i32 = rng.gen_range(-1..1);  
        let delta_y: i32 = rng.gen_range(-1..1);  
        
        // Move mouse relative to current position  
        enigo.move_mouse(delta_x, delta_y, Coordinate::Rel)  
            .map_err(|e| e.to_string())?;  
        
        // Sleep for 100ms to reduce CPU usage  
        thread::sleep(Duration::from_millis(100));  
    }  
    
    Ok(())  
}
