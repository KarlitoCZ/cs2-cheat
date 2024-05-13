use std::{sync::mpsc::{self, Sender}, thread};

mod antiflash;
mod glow;
mod offsets;
mod money_service;
mod gui;

struct MyApp {
    glow_check: bool,
    antiflash_check: bool,
    tx_antiflash_data: Sender<bool>,
    tx_glow_data: Sender<bool>,
}


fn main() {
    
    let (tx_antiflash, rx_antiflash) = mpsc::channel::<bool>();

    let (tx_glow, rx_glow) = mpsc::channel::<bool>();

    let data = MyApp {
        glow_check : false,
        antiflash_check : false,
        tx_antiflash_data : tx_antiflash,
        tx_glow_data: tx_glow
    };

    println!("[+] Main function started");

    // Handle the hack loops
    let handle_glow = thread::spawn(|| {
        glow::glow_init(rx_glow);
    });


    let handle_antiflash = thread::spawn(move || {
        antiflash::antiflash_init(rx_antiflash);
    });

    gui::start_gui(data);
    handle_glow.join().unwrap();
    handle_antiflash.join().unwrap();

 
}


