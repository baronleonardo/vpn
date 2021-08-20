pub mod vpn {
    use std::process::Command;
    use std::str;
    use std::env;
    use std::path::Path;

    pub struct Vpn;
    impl Vpn {
        pub fn on(ip: &str, port: &str) -> Result<(), String> {
            Vpn::check_requirments()?;

            let username = env::var("USER").unwrap();
            
            // create a tun interface
            let create_tun = Command::new("ip")
                .args(&["tuntap", "add", "dev", "tun0", "mode", "tun", "user", username.as_str()])
                .output()
                .expect("Failed to run the process");
            if create_tun.status.code().unwrap() != 0 {
                return Err(String::from_utf8(create_tun.stderr).unwrap());
            }

            // create a tun interface
            let prepare_tun = Command::new("ifconfig")
                .args(&["tun0", "10.0.0.1", "netmask", "255.255.255.0"])
                .output()
                .expect("Failed to run the process");
            if prepare_tun.status.code().unwrap() != 0 {
                return Err(String::from_utf8(prepare_tun.stderr).unwrap());
            }
            
            // create a vpn
            let ip_port = format!("{}:{}", ip, port);
            let _ = Command::new("badvpn-tun2socks")
                .args(&["--loglevel", "0", "--tundev", "tun0", "--netif-ipaddr", "10.0.0.2", "--netif-netmask", "255.255.255.0", "--socks-server-addr", ip_port.as_str()])
                .spawn()
                .expect("Failed to run the process");

            // create a tun interface
            let route = Command::new("route")
                .args(&["add", "default", "gw", "10.0.0.2", "metric", "6"])
                .output()
                .expect("Failed to run the process");
            if route.status.code().unwrap() != 0 {
                return Err(String::from_utf8(route.stderr).unwrap());
            }

            Ok(())
        }

        pub fn off() -> Result<(), String> {
            Vpn::check_requirments()?;

            // kill badvpn
            let _ = Command::new("killall")
                .args(&["badvpn-tun2socks"])
                .status()
                .expect("Failed to run the process");

            // delete tun network interface
            let mut del_tun_cmd = Command::new("ip");
            del_tun_cmd.args(&["tuntap", "del", "dev", "tun0", "mode", "tun"]);
            loop {
                let del_tun = del_tun_cmd.output().expect("Failed to run the process");
                if del_tun.status.code().unwrap() == 0 {
                    break;
                }
            }

            Ok(())
        }

        pub fn is_vpn_on() -> bool {
            Path::new("/sys/class/net/tun0").exists()
        }

        fn check_cmd(cmd: &str) -> Result<(), String> {
            let exists = Command::new("which")
                .args(&[cmd])
                .output()
                .expect("Failed to run the process")
                .status
                .code()
                .unwrap();
            if exists != 0 {
                return Err(format!("{} cmd not found", cmd));
            }

            Ok(())
        }

        fn check_requirments() -> Result<(), String>{
            let which = Vpn::check_cmd("which");
            if which.is_err() {
                return which;
            }

            // badvpn
            let badvpn = Vpn::check_cmd("badvpn-tun2socks");
            if badvpn.is_err() {
                return badvpn;
            }

            let ifconfig = Vpn::check_cmd("ifconfig");
            if ifconfig.is_err() {
                return ifconfig;
            }

            // iproute
            let iproute = Vpn::check_cmd("ip");
            if iproute.is_err() {
                return iproute;
            }

            let killall = Vpn::check_cmd("killall");
            if killall.is_err() {
                return killall;
            }

            Ok(())
        }
    }
}
