use std::net::{IpAddr, SocketAddr};

struct Computer {
    id: String,
    description: String,
    ip: Vec<IpAddr>
}

struct Hop {
    device: Computer,
    start: Vec<Flow>,
    end: Vec<Flow>
}

enum Flow {
    TunnelFlow(Tunnel),
    ConnectionFlow(Connection),
}

struct Tunnel {
    listening_sock: SocketAddr,
    connecting_sock: SocketAddr,
    forwarding_sock: SocketAddr
}

struct Connection {
    connecting_sock: SocketAddr,
    forwarded_sock: SocketAddr
}

fn draw_box(xsize: usize, ysize:usize) -> String{
    let mut edge = String::new();
    let mut middle = String::new();
    let mut out = String::new();
    edge.push_str("+");
    edge.push_str(&*"-".repeat(xsize - 2));
    edge.push_str("+");
    middle.push_str("|");
    middle.push_str(&*" ".repeat(xsize - 2));
    middle.push_str("|");
    out.push_str(&*edge);
    out.push_str(&*middle.repeat(ysize - 2));
    out.push_str(&*edge);
    return out;
}

#[test]
fn draw_6x6_box(){
    let mut out = String::new();
    out.push_str(concat!("+-----------+",
                                 "|           |",
                                 "|           |",
                                 "|           |",
                                 "|           |",
                                 "+-----------+"));
    let mut ret = draw_box(6,6);
    assert!(out.eq(&ret));
}