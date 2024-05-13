

use toy_arms::external::{read, write};
use std::mem::size_of;
use std::sync::mpsc::Receiver;
use toy_arms::external::process::Process;
use std::thread;
use std::time::Duration;

use crate::offsets;


pub fn antiflash_init(rx : Receiver<bool>) {
    
    let process = Process::from_process_name("cs2.exe").unwrap();

    let client = process.get_module_info("client.dll").unwrap();
    println!("[+] client.dll name: {}", client.name);
    // Offsets
    const DW_LOCAL_PLAYER_PAWN: usize = offsets::DW_LOCAL_PLAYER_PAWN;
    const M_FL_FLASH_BANG_TIME: usize = offsets::M_FL_FLASH_BANG_TIME;

    let mut last_checked = false;
    
    
    loop {
        thread::sleep(Duration::from_millis(100));
        
        match rx.try_recv() {
            Ok(value) => last_checked = value,
            Err(_) => {}, // Do nothing if no new data; retain last known state
        }

        if last_checked == false {
            continue;
        }   

        let mut localplayer: usize = 0;

        let _ok = read::<usize>(
            &process.handle,
            client.base_address + DW_LOCAL_PLAYER_PAWN as usize,
            size_of::<usize>(),
            &mut localplayer as *mut usize,
        );

        let mut flash_duration: f64 = 0.0; // 0 - 1
        let _read_flash = read::<f64>(
            &process.handle,
            localplayer + M_FL_FLASH_BANG_TIME,
            size_of::<f64>(),
            &mut flash_duration as *mut f64
        );
        

        if flash_duration > 0.0 {
            println!("Flash Evaded");
            let _write_flash = write::<f32>(
                &process.handle,
                localplayer as usize + M_FL_FLASH_BANG_TIME,
                &mut 0.0
            );
        }
    }
    
}