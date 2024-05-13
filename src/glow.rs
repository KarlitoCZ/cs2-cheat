use toy_arms::external::{read, write};
use std::{ mem::size_of, sync::mpsc::Receiver};
use toy_arms::external::process::Process;
use std::thread;
use std::time::Duration;

use crate::offsets::{self, DW_ENTITY_LIST, DW_LOCAL_PLAYER_PAWN, M_H_PLAYER_PAWN};

pub fn glow_init(rx : Receiver<bool>) {

    //get process
    let process = Process::from_process_name("cs2.exe").unwrap();
    let client = process.get_module_info("client.dll").unwrap();

    const M_FL_DETECTED_BY_ENEMY_SENSOR_TIME: usize = offsets::M_FL_DETECTED_BY_ENEMY_SENSOR_TIME;  
    println!("Glow Init");

    let mut localplayer: u64 = 0;
    let _read_localplayer = read::<u64>(
        &process.handle,
        client.base_address + DW_LOCAL_PLAYER_PAWN as usize,
        size_of::<u64>(),
        &mut localplayer as *mut u64,
    );

    let mut last_checked = false;

    loop {

        let mut entity_list: u64 = 0;

        let _read_entity_list = read::<u64>(
            &process.handle,
            client.base_address + DW_ENTITY_LIST,
            size_of::<u64>(),
            &mut entity_list as *mut u64,
        );

        for i in 1..64 {
            if entity_list == 0 {
                continue;
            }

            let mut entry_list: u64 = 0;


            match rx.try_recv() {
                Ok(value) => last_checked = value,
                Err(_) => {}, // Do nothing if no new data; retain last known state
            }
    
              
            

            let _read_entry_list = read::<u64>(
                &process.handle,
                entity_list as usize + ((8 * (i & 0x7FF) >> 9) as usize + 16),
                size_of::<u64>(),
                &mut entry_list as *mut u64,
            );
            if entry_list == 0 { continue; }

            let mut entity_controller: u64 = 0;

            let _read_entity_controller = read::<u64>(
                &process.handle,
                entry_list as usize + 120 * (i & 0x1FF),
                size_of::<u64>(),
                &mut entity_controller as *mut u64,
            );
            if entity_controller == 0 { continue; }

            let mut entity_controller_pawn: u64 = 0;

            let _read_entity_controller_pawn = read::<u64>(
                &process.handle,
                entity_controller as usize + M_H_PLAYER_PAWN,
                size_of::<u64>(),
                &mut entity_controller_pawn as *mut u64,
            );
            if entity_controller_pawn == 0 { continue; }

            let mut entity: u64 = 0;

            let _read_entity = read::<u64>(
                &process.handle,
                entry_list as usize + 120 * (entity_controller_pawn & 0x1FF) as usize,
                size_of::<u64>(),
                &mut entity as *mut u64,
            );
            if entity == 0 { continue; }

            let mut entry_list2: u64 = 0;

            

            let _read_entry_list2 = read::<u64>(
                &process.handle,
                entity_list as usize + 0x8 * ((entity_controller_pawn & 0x7FF) >> 9) as usize + 16,
                size_of::<u64>(),
                &mut entry_list2 as *mut u64,
            );
            if entry_list2 == 0 { continue; }

            let mut p_csplayer_pawn: u64 = 0;

            let _readp_csplayer_pawn = read::<u64>(
                &process.handle,
                entry_list2 as usize + 120 * (entity_controller_pawn & 0x1FF) as usize,
                size_of::<u64>(),
                &mut p_csplayer_pawn as *mut u64,
            );
            if p_csplayer_pawn == 0 { continue; }
            
            if p_csplayer_pawn == localplayer { continue; }

            let mut set_value: f32 = if last_checked {
                86400.0
            } else {
                0.0
            };

            

            let _write_glow = write::<f32>(
                &process.handle,
                p_csplayer_pawn as usize + M_FL_DETECTED_BY_ENEMY_SENSOR_TIME,
                &mut set_value
            );

            thread::sleep(Duration::from_millis(300));


        }
    }

}  
