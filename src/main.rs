use serde_json;
use std::net::TcpStream;
use tungstenite::stream::MaybeTlsStream;
use tungstenite::{connect, Message, WebSocket};
use url::Url;

mod config;

use config::create_trade_pairs;

// wss://stream.binance.com:9443/ws/btcusdt@trade
static BINANCE_WS_API: &str = "wss://stream.binance.com:9443/ws/btcusdt@trade";

//multiple stream ->  static BINANCE_WS_API: &str = "wss://stream.binance.com:9443/ws/ethbtc@depth5@100ms";

fn read_message(socket: &mut WebSocket<MaybeTlsStream<TcpStream>>) {
    let msg = socket.read_message().expect("Error reading message");
    let msg = match msg {
        Message::Text(s) => s,
        _ => {
            panic!("Error getting text");
        }
    };

    let parsed: serde_json::Value = serde_json::from_str(&msg).expect("Can't parse to JSON");
    println!("{:?}", parsed);
}

fn main() {
    // Connect to the binance WS server

    let binance_url = format!("{}", BINANCE_WS_API);

    let (mut socket, _response) =
        connect(Url::parse(&binance_url).unwrap()).expect("Can't connect");

    println!("Connected to binance stream.");
    println!("HTTP status code: {}", _response.status());
    println!("Response headers:");

    for (ref header, header_value) in _response.headers() {
        println!("- {}: {:?}", header, header_value);
    }

    let all_trade_pairs = create_trade_pairs();

    let mut all_socket_streams: Vec<WebSocket<MaybeTlsStream<TcpStream>>> = Vec::new();

    for trade_pair in all_trade_pairs {
        let connection_url = format!(
            "wss://stream.binance.com:9443/ws/{}",
            trade_pair.stream_name
        );

        let (socket, _response) =
            connect(Url::parse(&connection_url).unwrap()).expect("Can't connect");

        all_socket_streams.push(socket);

        println!("Listening to stream {}", trade_pair.stream_name);
        println!("HTTP status code: {}", _response.status());

        // read_message(&mut socket);
    }

    // Loop forever, handling parsing each message
    loop {
        // for mut single_socket in &all_socket_streams {
        read_message(&mut socket);
        // }
    }
}

/*


 let connection_url = "wss://stream.binance.com:9443/stream?streams=btcusdt@trade/ethbtc@trade";

    let (mut socket, _response) =
        connect(Url::parse(connection_url).unwrap()).expect("Can't connect");

    println!("Connected to binance stream.");
    println!("HTTP status code: {}", _response.status());

// Write a message containing "Hello, Test!" to the server
    // socket
    //     .write_message(Message::Text("Hello, Test!".into()))
    //     .unwrap();

tungstenite = "0.16.0"
serde_json = "1.0.78"
url = "2.2.2"



tungstenite = { version="0.14.0", features = ["rustls-tls"]}
*/
