use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Builder, Switch, Entry};

mod vpn;
use vpn::vpn::Vpn;

fn build_ui(app: &gtk::Application) {

    let ui_str = include_str!("main.ui");
    let builder = Builder::from_string(ui_str);
    let window: ApplicationWindow = builder.object("main_window").expect("no main window");
    window.set_application(Some(app));

    let switch: Switch = builder.object("switch").expect("no switch object");
    if Vpn::is_vpn_on() {
        switch.set_state(true);
    }
    switch.connect_state_set(move |_, state| {
        let ip_entry: Entry = builder.object("ip").expect("no switch object");
        let port_entry: Entry = builder.object("port").expect("no switch object");
        let ip = ip_entry.text();
        let port = port_entry.text();

        match state {
            true => Vpn::on(&ip, &port).unwrap(),
            false => Vpn::off().unwrap()
        };

        gtk::Inhibit(false)
    });

    window.show_all();
}

fn main() {
    let application = Application::builder()
        .application_id("com.baronleonardo.vpn")
        .build();

    application.connect_activate(build_ui);

    application.run();
}