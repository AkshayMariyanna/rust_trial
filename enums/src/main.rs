fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    let loopback = IpAddrS {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let home = IpAddr::V4(String::from("127.0.0.1"));

    let home_e = IpAddrEnhanced::V4(127, 0, 0, 1);

    let m = Message::Write(String::from("hello"));
    m.call();

    let _y: Option<String> = None;
    let _z = Some(5);
}

enum IpAddrKind {
    V4,
    V6,
}

struct IpAddrS {
    kind: IpAddrKind,
    address: String,
}

enum IpAddr {
    V4(String),
    V6(String),
}

enum IpAddrEnhanced {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn route(ip_type: IpAddrKind) {
    
}



enum Message {
    Quit,
    Move { x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // do something
    }
}
