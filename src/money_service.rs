use toy_arms::external::read;
use std::mem::size_of;
use toy_arms::external::process::Process;

use std::ffi::CStr;
use std::os::raw::c_char;

use crate::offsets::{ DW_ENTITY_LIST, DW_LOCAL_PLAYER_PAWN, M_ISZ_PLAYER_NAME, M_I_ACCOUNT, M_I_TEAM_NUM, M_P_IN_GAME_MONEY_SERVICES};

pub fn money_service_request() -> (Vec<String>, Vec<i32>) {

    let mut names : Vec<String> = Vec::new();
    let mut money : Vec<i32> = Vec::new();

    let process = Process::from_process_name("cs2.exe").unwrap();
    let client = process.get_module_info("client.dll").unwrap();

    let mut entity_list: u64 = 0;

    let _read_entity_list = read::<u64>(
        &process.handle,
        client.base_address + DW_ENTITY_LIST,
        size_of::<u64>(),
        &mut entity_list as *mut u64,
    );

    let mut localplayer: u64 = 0;
    let _read_localplayer = read::<u64>(
        &process.handle,
        client.base_address + DW_LOCAL_PLAYER_PAWN as usize,
        size_of::<u64>(),
        &mut localplayer as *mut u64,
    );  

    for i in 1..32 {
        if entity_list == 0 {
            continue;
        }

        let mut entry_list: u64 = 0;
            

        let _read_entry_list = read::<u64>(
            &process.handle,
            entity_list as usize + ((8 * (i & 0x7FF) >> 9) as usize + 16),
            size_of::<u64>(),
            &mut entry_list as *mut u64,
        );
        if entry_list == 0 {
            continue;
        }
        

        let mut entity_controller: u64 = 0;

        let _read_entity_controller = read::<u64>(
            &process.handle,
            entry_list as usize + 120 * (i & 0x1FF),
            size_of::<u64>(),
            &mut entity_controller as *mut u64,
        );
        if entity_controller == 0 { continue; }

        let mut money_service: u64 = 0;

        let _read_money_service = read::<u64>(
            &process.handle,
            entity_controller as usize + M_P_IN_GAME_MONEY_SERVICES as usize,
            size_of::<u64>(),
            &mut money_service as *mut u64,
        );
        if money_service == 0 { continue; }

        let mut controller_money: i32 = 0;

        let _read_controller_money = read::<i32>(
            &process.handle,
            money_service as usize + M_I_ACCOUNT as usize,
            size_of::<i32>(),
            &mut controller_money as *mut i32,
        );

        let mut controller_team: i32 = 0;

        let _read_controller_team = read::<i32>(
            &process.handle,
            entity_controller as usize + M_I_TEAM_NUM as usize,
            size_of::<i32>(),
            &mut controller_team as *mut i32,
        );

        let mut localplayer_team: i32 = 0;

        let _read_localplayer_team = read::<i32>(
            &process.handle,
            localplayer as usize + M_I_TEAM_NUM as usize,
            size_of::<i32>(),
            &mut localplayer_team as *mut i32,
        );

        if controller_team == localplayer_team {
            continue;
        }

        unsafe {
            let mut buffer: Vec<u8> = Vec::with_capacity(100);
            buffer.set_len(100);

            let _read_controller_name = read::<c_char>(
                &process.handle,
                entity_controller as usize + M_ISZ_PLAYER_NAME as usize,
                buffer.len(),
                buffer.as_mut_ptr() as *mut c_char,
            );
            
            let c_str = CStr::from_ptr(buffer.as_ptr() as *const c_char);

            let name = c_str.to_string_lossy().into_owned();

            names.push(name);
            money.push(controller_money);
        }

    }
    return (names, money);
}


