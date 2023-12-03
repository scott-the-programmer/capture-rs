use pnet::datalink::{self, NetworkInterface};
use pnet::packet::{ethernet::EthernetPacket, Packet};
use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::widgets::{Block, Borders, Widget};
use tui::Terminal;

use std::env;

fn main() {
    let stdout = std::io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    let interface_name = env::args()
        .nth(1)
        .expect("Usage: capture-rs <interface_name>");
    let interface_names_match = |iface: &NetworkInterface| iface.name == interface_name;
    let interfaces = datalink::interfaces();
    let interface = interfaces
        .into_iter()
        .filter(interface_names_match)
        .next()
        .expect("Network interface not found");

    let (_tx, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(datalink::Channel::Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unhandled channel type"),
        Err(e) => panic!("Error occurred when creating the datalink channel: {}", e),
    };

    loop {
        match rx.next() {
            Ok(packet) => match EthernetPacket::new(packet) {
                Some(pkt) => draw(&mut terminal, &pkt),
                None => eprintln!("Failed to parse Ethernet packet"),
            },
            Err(e) => {
                panic!("An error occurred while reading the packet: {}", e);
            }
        }
    }
}

fn draw(terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>, packet: &EthernetPacket) {
    terminal
        .draw(|f| {
            let size = f.size();
            let block = Block::default().title("Packet").borders(Borders::ALL);
            f.render_widget(block, size);
        })
        .unwrap();
}
