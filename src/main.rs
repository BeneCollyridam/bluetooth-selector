use std::{
    io::{self, Read},
    process::Command,
    str::FromStr,
    thread::sleep,
    time::Duration,
};

#[derive(Debug)]
struct Device<'a> {
    mac_addr: &'a str,
    name: &'a str,
}

impl<'a> std::fmt::Display for Device<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

fn main() {
    Command::new("stty")
        .arg("cbreak")
        .arg("-echo")
        .status()
        .expect("Could not run stty");

    let cmd_output = Command::new("bluetoothctl")
        .arg("devices")
        .output()
        .unwrap();
    let devices = {
        let mut devices = Vec::new();
        for l in std::str::from_utf8(&cmd_output.stdout)
            .expect("Not valid utf-8")
            .lines()
            .filter(|x| !x.is_empty())
        {
            let mut parts = l.split(' ');
            let mut name_start = parts.next().unwrap().len() + 1;
            let mac_addr = parts.next().expect("Bad input");
            name_start += mac_addr.len() + 1;
            let name = &l[name_start..];

            devices.push(Device { mac_addr, name })
        }
        devices
    };

    if devices.is_empty() {
        println!("No devices found");
        return;
    }

    println!("Choose a device:");
    for (i, device) in devices.iter().enumerate() {
        println!("Device {}: {device}", i)
    }

    let n: usize = {
        let mut stdin = io::stdin();
        loop {
            let mut num = String::new();
            let mut buf = [0];
            loop {
                stdin.read_exact(&mut buf).unwrap();
                num.push(buf[0] as char);
                if devices.len() <= 10_usize.pow(num.len() as u32) {
                    break;
                }
            }
            if let Ok(n) = usize::from_str(&num) {
                if n < devices.len() {
                    break n;
                }
            }

            println!("Input a number in the range 0-{}", devices.len() - 1)
        }
    };

    let mut connect_process = Command::new("bluetoothctl")
        .args(["connect", devices[n].mac_addr])
        .spawn()
        .unwrap();

    let _ = connect_process.wait();
    sleep(Duration::from_secs(2));
}
