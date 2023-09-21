extern crate gtk;
use gtk::prelude::*;
use gtk::{Button, Window, WindowType, Image};
use std::process::Command;

fn main() {
    gtk::init().expect("Failed to initialize GTK.");

    let window = Window::new(WindowType::Toplevel);
    window.set_title("Time Saver");
    window.set_default_size(200, 200);

    let button_box = gtk::Box::new(gtk::Orientation::Vertical, 0);
    window.add(&button_box);

    let btn_update = Button::new();
    btn_update.set_label("Update Computer");
    btn_update.set_tooltip_text(Some("This will update your computer."));
    let image_update = Image::from_icon_name(Some("system-software-update"), gtk::IconSize::Button.into());
    btn_update.set_image(Some(&image_update));

    let btn_upgrade = Button::new();
    btn_upgrade.set_label("Upgrade Computer");
    btn_upgrade.set_tooltip_text(Some("This will upgrade your computer."));
    let image_upgrade = Image::from_icon_name(Some("system-upgrade"), gtk::IconSize::Button.into());
    btn_upgrade.set_image(Some(&image_upgrade));

    let btn_autoremove = Button::new();
    btn_autoremove.set_label("Autoremove");
    btn_autoremove.set_tooltip_text(Some("This will autoremove unused packages."));
    let image_autoremove = Image::from_icon_name(Some("edit-delete"), gtk::IconSize::Button.into());
    btn_autoremove.set_image(Some(&image_autoremove));

    let btn_autoclean = Button::new();
    btn_autoclean.set_label("Autoclean");
    btn_autoclean.set_tooltip_text(Some("This will clean the package cache."));
    let image_autoclean = Image::from_icon_name(Some("edit-clear"), gtk::IconSize::Button.into());
    btn_autoclean.set_image(Some(&image_autoclean));

    let btn_open_firefox = Button::new();
    btn_open_firefox.set_label("Open Firefox");
    btn_open_firefox.set_tooltip_text(Some("This will launch Firefox browser."));
    let image_firefox = Image::from_icon_name(Some("web-browser"), gtk::IconSize::Button.into());
    btn_open_firefox.set_image(Some(&image_firefox));

    let btn_open_chromium = Button::new();
    btn_open_chromium.set_label("Open Chromium");
    btn_open_chromium.set_tooltip_text(Some("This will launch Chromium browser."));
    let image_chromium = Image::from_icon_name(Some("web-browser"), gtk::IconSize::Button.into());
    btn_open_chromium.set_image(Some(&image_chromium));

    let btn_system_info = Button::new();
    btn_system_info.set_label("Show System Info");
    btn_system_info.set_tooltip_text(Some("This will show system information."));
    let image_system_info = Image::from_icon_name(Some("computer"), gtk::IconSize::Button.into());
    btn_system_info.set_image(Some(&image_system_info));

    let btn_network_scan = Button::new();
    btn_network_scan.set_label("Network Scan");
    btn_network_scan.set_tooltip_text(Some("This will perform a network scan."));
    let image_network_scan = Image::from_icon_name(Some("network-workgroup"), gtk::IconSize::Button.into());
    btn_network_scan.set_image(Some(&image_network_scan));

    button_box.add(&btn_update);
    button_box.add(&btn_upgrade);
    button_box.add(&btn_autoremove);
    button_box.add(&btn_autoclean);
    button_box.add(&btn_open_firefox);
    button_box.add(&btn_open_chromium);
    button_box.add(&btn_system_info);
    button_box.add(&btn_network_scan);

    btn_update.connect_clicked(|_| {
        execute_command("update", "You chose to update your computer.");
    });

    btn_upgrade.connect_clicked(|_| {
        execute_command("upgrade", "You chose to upgrade your computer.");
    });

    btn_autoremove.connect_clicked(|_| {
        execute_command("autoremove", "You chose to autoremove.");
    });

    btn_autoclean.connect_clicked(|_| {
        execute_command("autoclean", "You chose to autoclean.");
    });

    btn_open_firefox.connect_clicked(|_| {
        println!("You chose to open Firefox.");
        Command::new("firefox")
            .spawn()
            .expect("Failed to open Firefox");
    });

    btn_open_chromium.connect_clicked(|_| {
        println!("You chose to open Chromium.");
        Command::new("chromium")
            .spawn()
            .expect("Failed to open Chromium");
    });

    btn_system_info.connect_clicked(|_| {
        println!("You chose to show system info.");
        Command::new("neofetch")
            .spawn()
            .expect("Failed to run neofetch");
    });

    btn_network_scan.connect_clicked(|_| {
        println!("You chose to perform a network scan.");
        Command::new("nmap")
            .arg("-sn")
            .arg("192.168.1.0/24")
            .spawn()
            .expect("Failed to perform network scan");
    });

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    window.show_all();
    gtk::main();
}

fn execute_command(arg: &str, message: &str) {
    println!("{}", message);
    let _status = Command::new("sudo")
        .arg("apt")
        .arg(arg)
        .status()
        .expect("Failed to execute command");
}
