mod lib;

use dioxus::prelude::*;

/**
 * Helper Functions
 */

fn search_for_devices() -> Vec<(std::path::PathBuf, evdev::Device)> {
    let all_devs = evdev::enumerate();
    
    let mut available_devs = Vec::new();
    
    for dev in all_devs {
        let cur_dev = dev.1.supported_absolute_axes().map(|ins| ins.iter().collect::<Vec<_>>().contains(&evdev::AbsoluteAxisType::ABS_X));
        if cur_dev.is_some() && cur_dev.unwrap() == true {
            available_devs.push(dev);
        }
    }
    
    return available_devs;
}

/**
 * UI Functions
 */
fn controller_list(cx: Scope) -> Element {
    let device_list = search_for_devices();
    cx.render(rsx!(
        option {
            label: "Choose A Device: ",
            value: -1
        }
        for i in 0..device_list.len() {
            option {
                label: "{device_list[i].1.name().unwrap()}",
                value: i as i64,
            }
        }
    ))
}

fn Client_UI(cx: Scope) -> Element {
    let mut devices = search_for_devices();
    
    let mut device = std::rc::Rc::new(std::cell::RefCell::new(search_for_devices().pop().unwrap()));
    
    let mut dev_clone1 = device.clone();
    let dev_clone2 = device.clone();
    
    let mut server = use_state(cx, || String::from("localhost:0"));
    
    cx.render(rsx! {
        // Controller Select
        div {
			text_align: "center",
            "--Controller Select--"
        }
        
        br {}
        
        div {
            text_align: "center",
            
            select {
                oninput: move |event| {
                    event.stop_propagation();
                    let index = event.clone().value.parse::<i32>().unwrap();
                    
                    if index >= 0 {
                        dev_clone1.replace(devices.remove(event.clone().value.parse::<usize>().unwrap()));
                    
                        // Rescan Devices
                        devices = search_for_devices();
                    }
                },
                controller_list {}
            }
            
            // TODO Add Device Rescanning
        }
        
        br {}
        
        // Server IP and Port
        div {
			text_align: "center",
            "--Server Select--"
        }
        form {
            div {
                text_align: "center",
                input {
                    oninput: move |ip| {
                        server.set(ip.value.clone());
                    },
                    r#type: "text",
                    value: "{server}"
                }
            }
        }
        
        br {}
        
        // Connect
        div {
            text_align: "center",
            button {
                onclick: move |event| {
                    println!("{:?}", device.borrow().1.name());
                    println!("{:?}", server.get())
                },
                "Connect"
            }
        }
    })
}

fn main () {
	let desktop_window = dioxus_desktop::tao::window::WindowBuilder::new().with_title("NetJoystick - Client");
	let desktop_config = dioxus_desktop::Config::new().with_window(desktop_window);
	dioxus_desktop::launch_cfg(Client_UI, desktop_config);
}