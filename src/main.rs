use hidapi::HidApi;

fn main() {
    // Create a new instance of the HidApi
    let api = HidApi::new().expect("Failed to initialize HID API");

    // Get a list of all connected HID devices
    let devices = api.device_list();

    // Iterate over the devices and find the one with the desired Product string
    let mut arduino_micro_device = None;
    for device in devices {
        if let Some(product_string) = device.product_string() {
            if product_string == "Arduino Micro" {
                arduino_micro_device = Some(device);
                break;
            }
        }
    }

    // Check if the Arduino Micro device was found
    if let Some(arduino_micro_device) = arduino_micro_device {
        // Read data from the Arduino Micro device in a loop
        loop {
            let mut buf = [0u8; 64];
            let bytes_read = arduino_micro_device
                .open_device(&api)
                .expect("Failed to open Arduino Micro device")
                .read(&mut buf)
                .expect("Failed to read data from Arduino Micro");

            // Process the read data
            let quat: [u8; 63] = [
                buf[0], buf[1], buf[2], buf[3], buf[4], buf[5], buf[6], buf[7], buf[8], buf[9],
                buf[10], buf[11], buf[12], buf[13], buf[14], buf[15], buf[16], buf[17], buf[18],
                buf[19], buf[20], buf[21], buf[22], buf[23], buf[24], buf[25], buf[26], buf[27],
                buf[28], buf[29], buf[30], buf[31], buf[32], buf[33], buf[34], buf[35], buf[36],
                buf[37], buf[38], buf[39], buf[40], buf[41], buf[42], buf[43], buf[44], buf[45],
                buf[46], buf[47], buf[48], buf[49], buf[50], buf[51], buf[52], buf[53], buf[54],
                buf[55], buf[56], buf[57], buf[58], buf[59], buf[60], buf[61], buf[62],
            ];

            // Round the numbers to two decimal places and force the format "x.xx"
            let rounded_quat: Vec<String> = quat.iter().map(|&num| format!("{:.2}", num)).collect();

            // Clear the terminal screen
            print!("\x1B[2J\x1B[1;1H");

            println!("Read {} bytes from Arduino Micro:", bytes_read);
            for i in 0..4 {
                println!("quat[{}]: {}", i, rounded_quat[i]);
            }
        }
    } else {
        println!("Arduino Micro device not found");
    }
}
