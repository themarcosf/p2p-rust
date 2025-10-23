use libp2p::futures::StreamExt;
use libp2p::ping::Config;
use libp2p::request_response::json;
use libp2p::swarm::{NetworkBehaviour, SwarmEvent};
use libp2p::{Multiaddr, StreamProtocol, noise, ping, request_response, tcp, yamux};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::{io, select};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct MessageRequest {
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct MessageResponse {
    pub ack: bool,
}

#[derive(NetworkBehaviour)]
struct ChatBehaviour {
    ping: ping::Behaviour,
    messaging: json::Behaviour<MessageRequest, MessageResponse>,
}

/// The async entry point that runs inside the Tokio runtime, and uses anyhow for ergonomic error handling.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let port: u16 = std::env::var("CHAT_P2P_PORT")
        .unwrap_or(String::from("9999"))
        .parse::<u16>()?;

    let peer: Multiaddr = std::env::var("CHAT_PEER")?.parse()?;

    // Create a Swarm to manage
    //   - networking connections (who to connect to)
    //   - protocols (how peers communicate)
    //   - events (what happens when a message is received, a connection is established, etc)
    let mut swarm = libp2p::SwarmBuilder::with_new_identity()
        .with_tokio()
        .with_tcp(
            tcp::Config::default(),
            noise::Config::new,
            yamux::Config::default,
        )?
        .with_behaviour(|_key_pair| {
            Ok(ChatBehaviour {
                ping: ping::Behaviour::new(Config::new().with_interval(Duration::from_secs(10))),
                messaging: json::Behaviour::new(
                    [(
                        StreamProtocol::new("/chat/1.0.0"),
                        request_response::ProtocolSupport::Full,
                    )],
                    request_response::Config::default(),
                ),
            })
        })?
        .with_swarm_config(|config| config.with_idle_connection_timeout(Duration::from_secs(30)))
        .build();

    swarm.listen_on(format!("/ip4/0.0.0.0/tcp/{}", port).parse()?)?;
    swarm.dial(peer.clone())?;

    println!("Peer ID: {:?}", swarm.local_peer_id());

    let mut stdin = BufReader::new(io::stdin()).lines();

    let mut target_peer_id = None;

    // The swarm runs in a loop, listening for events and handling them as they occur.
    loop {
        select! {
            event = swarm.select_next_some() => match event {
                SwarmEvent::NewListenAddr { address, .. } => {
                    println!("Listening on {:?}", address);
                }
                SwarmEvent::ConnectionEstablished { peer_id, .. } => {
                    println!("Connected established with peer {:?}", peer_id);
                    if target_peer_id.is_none() {
                        target_peer_id = Some(peer_id);
                    }
                    swarm.add_peer_address(peer_id, peer.clone());
                }
                SwarmEvent::Behaviour(event) => match event {
                    ChatBehaviourEvent::Ping(event) => {
                        println!("Ping event: {:?}", event);
                    }
                    ChatBehaviourEvent::Messaging(event) => match event {
                        request_response::Event::Message { peer, message } => match message {
                            request_response::Message::Request { request_id, request, channel } => {
                                println!("Received message {:?} from {:?}: {:?}", request_id, peer, request.message);
                                if let Err(error) = swarm.behaviour_mut().messaging.send_response(channel, MessageResponse { ack: true }) {
                                    println!("Failed to send response to {:?} for request {:?}: {:?}", peer, request_id, error);
                                };
                            }
                            request_response::Message::Response { request_id, response } => {
                                println!("Received response {:?} from {:?}: {:?}", request_id, peer, response.ack);
                            }
                        },
                        request_response::Event::OutboundFailure { peer, request_id, error } => {
                            println!("Outbound failure to peer {:?} for request {:?}: {:?}", peer, request_id, error);
                        },
                        request_response::Event::InboundFailure { peer, request_id, error } => {
                            println!("Inbound failure from peer {:?} for request {:?}: {:?}", peer, request_id, error);
                        },
                        request_response::Event::ResponseSent { .. } => {},
                    }
                }
                SwarmEvent::ConnectionClosed { .. } => {
                // SwarmEvent::ConnectionClosed { peer_id, connection_id, endpoint, num_established, cause } => {
                    target_peer_id = None;
                }
                _ => {}
            },
            Ok(Some(line)) = stdin.next_line() => {
                if let Some(peer_id) = target_peer_id {
                    swarm.behaviour_mut().messaging.send_request(&peer_id, MessageRequest {
                        message: line.clone(),
                    });
                    println!("Sent message to {:?}: {line:?}", swarm.local_peer_id());
                }
            }
        }
    }
}
