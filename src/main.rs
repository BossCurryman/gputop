use sysinfo::{System, SystemExt};

fn main() {
    let mut sys = System::new_all();
    sys.refresh_all();
    for disk in sys.disks() {
        println!("{:?}", disk);
    }
}
