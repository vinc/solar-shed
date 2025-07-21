use chrono::prelude::*;
use serde_json::json;
use std::{env, println};
use tokio_stream::StreamExt;
use victron_ble::DeviceState::SolarCharger;
use victron_ble::SolarChargerState;

fn round(num: f32, digits: usize) -> f64 {
    let precision = 10.0 * digits as f64;
    (num as f64 * precision).trunc() / precision
}

fn print_state(state: &SolarChargerState) {
    let date: DateTime<Utc> = Utc::now();
	let state = json!({
        "date": date,
		"mode": format!("{}", state.mode).to_lowercase(),
		"load": {
		    "A": round(state.load_current_a, 2),
		},
		"solar": {
		    "W": round(state.pv_power_w, 2),
		},
		"battery": {
		    "V": round(state.battery_voltage_v, 2),
		    "A": round(state.battery_current_a, 2),
		},
		"yield": {
		    "Wh": round(state.yield_today_kwh * 1000.0, 2),
		}
    });
    println!("{state:#}");
}


#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: victron <dev> <key>");
        return;
    }

    let dev = args.get(1).unwrap();
    let key = hex::decode(args.get(2).unwrap()).expect("Invalid device encryption key, it should be hex encoded.");
    let mut stream = victron_ble::open_stream(dev.into(), key).unwrap();
    while let Some(res) = stream.next().await {
        match res {
            Ok(SolarCharger(state)) => print_state(&state),
            Ok(state) => println!("{state:?}"),
            Err(error) => println!("{error}"),
        }
        break; // TODO: Add --continuous or --once option
    }
}
