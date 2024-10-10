use mavlink::common::MavMessage;

fn main() {
    print_telemetry();
}

fn print_telemetry() {
    //let mavlink_url = "udpin:127.0.0.1:14540";
    let mavlink_url = "udpin:127.0.0.1:14030";
    let mavlink = mavlink::connect(mavlink_url).unwrap();

    println!("Connected to mavlink: {:?}", mavlink.get_protocol_version());
    loop {
        match mavlink.recv() {
            Ok((_header, msg)) => match msg {
                MavMessage::HEARTBEAT(heartbeat) => {
                    println!("Heartbeat - System status: {:?}", heartbeat.system_status);
                }
                MavMessage::GLOBAL_POSITION_INT(position) => {
                    println!(
                        "Position - Lat: {}, Lon: {}, Alt: {} m",
                        position.lat as f32 / 1e7,
                        position.lon as f32 / 1e7,
                        position.alt as f32 / 1000.0
                    );
                }
                MavMessage::VFR_HUD(vfr) => {
                    println!(
                        "VFR HUD - Airspeed: {} m/s, Groundspeed: {} m/s, Heading: {}°",
                        vfr.airspeed, vfr.groundspeed, vfr.heading
                    );
                }
                MavMessage::BATTERY_STATUS(sys_status) => {
                    println!(
                        "System Status - Consumed Current: {} V, Temperature: {}C , Remaining battery: {}%",
                        sys_status.current_consumed as f32 / 1000.0,
                        sys_status.temperature as f32 / 1000.0,
                        sys_status.battery_remaining
                    );
                }
                _ => {}
            },
            Err(e) => {
                println!("Error: {:?}", e);
                break;
            }
        }
    }
}
