extern crate btleplug;
extern crate rand;

use btleplug::api::{Central, CentralEvent, EventHandler, Peripheral, UUID};
#[cfg(target_os = "linux")]
use btleplug::bluez::{adapter::ConnectedAdapter, manager::Manager};
#[cfg(target_os = "macos")]
use btleplug::corebluetooth::{adapter::Adapter, manager::Manager};
#[cfg(target_os = "windows")]
use btleplug::winrtble::{adapter::Adapter, manager::Manager};
use rand::{thread_rng, Rng};
use std::thread;
use std::time::Duration;
use async_std::{
    prelude::{FutureExt, StreamExt},
    sync::{channel, Receiver},
    task,
};
use std::sync::Arc;

// adapter retrieval works differently depending on your platform right now.
// API needs to be aligned.

#[cfg(any(target_os = "windows", target_os = "macos"))]
fn get_central(manager: &Manager) -> Adapter {
    let adapters = manager.adapters().unwrap();
    adapters.into_iter().nth(0).unwrap()
}

#[cfg(target_os = "linux")]
fn get_central(manager: &Manager) -> ConnectedAdapter {
    let adapters = manager.adapters().unwrap();
    let adapter = adapters.into_iter().nth(0).unwrap();
    adapter.connect().unwrap()
}

pub fn main() {
    let manager = Manager::new().unwrap();

    // get the first bluetooth adapter
    // connect to the adapter
    let central = get_central(&manager);

    // start scanning for devices
    central.start_scan().unwrap();
    // instead of waiting, you can use central.on_event to be notified of
    // new devices

    let central = Arc::new(central);
    let central_clone = central.clone();

    // https://stackoverflow.com/questions/1061005/calling-objective-c-method-from-c-member-function
    // https://www.transpire.com/insights/blog/writing-objective-c-wrapper/
    // https://nachtimwald.com/2015/12/18/interop-objective-c-objects-in-c-using-arc/
    // https://forum.juce.com/t/mixing-objective-c-c-and-objective-c-copied-article/15807
    // #include <objc/objc-runtime.h>
    // https://www.sitepoint.com/using-c-and-c-code-in-an-android-app-with-the-ndk/
    // https://www.sitepoint.com/using-c-and-c-in-an-ios-app-with-objective-c/
    // https://hyperpolyglot.org/cpp
    // https://github.com/Moret84/rs-mijiabt/blob/master/src/btleplug_ble_repo.rs
    // https://github.com/buttplugio/buttplug-rs/blob/master/buttplug/src/server/comm_managers/btleplug/btleplug_internal.rs#L62

    let (event_sender, event_receiver) = channel(256);
    // Add ourselves to the central event handler output now, so we don't
    // have to carry around the Central object. We'll be using this in
    // connect anyways.
    let on_event = move |event: CentralEvent| match event {
        CentralEvent::DeviceDiscovered(bd_addr) => {
            println!("DeviceDiscovered: {:?}", bd_addr);
            /*
            match &central_clone.peripheral(bd_addr) {
                Some(p) => {
                    println!("Resolved the address {:?} to peripheral {:?}", bd_addr, p);
                },
                None => {
                    println!("Could not resolve the address {:?}", bd_addr);
                }
            }
            */
            let s = event_sender.clone();
            let e = event.clone();
            task::spawn(async move {
                s.send(e).await;
            });
        }
        CentralEvent::DeviceLost(bd_addr) => {
            println!("DeviceLost: {:?}", bd_addr);
            let s = event_sender.clone();
            let e = event.clone();
            task::spawn(async move {
                s.send(e).await;
            });
        }
        CentralEvent::DeviceConnected(bd_addr) => {
            println!("DeviceConnected: {:?}", bd_addr);
            let s = event_sender.clone();
            let e = event.clone();
            task::spawn(async move {
                s.send(e).await;
            });
        }
        CentralEvent::DeviceUpdated(bd_addr) => {
            println!("DeviceUpdated: {:?}", bd_addr);
            let s = event_sender.clone();
            let e = event.clone();
            task::spawn(async move {
                s.send(e).await;
            });
        }
        CentralEvent::DeviceDisconnected(bd_addr) => {
            println!("DeviceDisconnected: {:?}", bd_addr);
            let s = event_sender.clone();
            let e = event.clone();
            task::spawn(async move {
                s.send(e).await;
            });
        }
    };

    central.on_event(Box::new(on_event));

    let mut count = 0;
    loop {
        count += 1;
        println!("Count = {}", count);
        thread::sleep(Duration::from_secs(1));
    }

    /*
    CentralEvent {
        DeviceDiscovered(BDAddr),
        DeviceLost(BDAddr),
        DeviceUpdated(BDAddr),
        DeviceConnected(BDAddr),
        DeviceDisconnected(BDAddr),
    }
    */

    /*
    let mut count = 0;
    while count < 30 {
        println!("Count = {}", count);
        thread::sleep(Duration::from_secs(1));
        count += 1;
    }

    for p in central.peripherals() {
        match p.discover_characteristics() {
            Ok(v) => println!("Got characteristics: {:?}", v),
            Err(e) => println!("Got errors when querying characteristics: {:?}", e),
        };
        println!("p.properties = {:?}", p.properties());
        println!("p.characteristics = {:?}", p.characteristics());
    }
    */
}
