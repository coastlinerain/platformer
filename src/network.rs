use crossbeam_channel::{Receiver, Sender};
use laminar::{Packet, Socket, SocketEvent};
use std::net::SocketAddr;

pub struct NetworkClient {
    pub sender: Sender<Packet>,
    pub receiver: Receiver<SocketEvent>,
    pub server_addr: SocketAddr,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub enum GamePacket {
    JoinRequest,
    JoinResponse {
        assigned_id: u64,
    },
    PlayerPos {
        id: u64,
        x: f32,
        y: f32,
        dir: f32,
        level_x: u8,
        level_y: u8,
    },
    Action {
        id: u64,
        kind: String,
        dir: f32,
    },
    Leave {
        id: u64,
    },
}

impl NetworkClient {
    pub fn new(server_ip: &str) -> Self {
        // socket (puerto 0 para que la maquina asigne uno libre)
        let mut socket =
            Socket::bind("0.0.0.0:0").expect("No se pudo bindear el socket de cliente");

        let sender = socket.get_packet_sender();
        let receiver = socket.get_event_receiver();
        let server_addr = server_ip.parse().expect("IP de servidor inválida");

        // Lanzamos el hilo de red (polling)
        std::thread::spawn(move || {
            socket.start_polling();
        });

        Self {
            sender,
            receiver,
            server_addr,
        }
    }
}
