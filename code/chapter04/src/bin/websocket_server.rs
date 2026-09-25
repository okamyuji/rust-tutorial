// src/bin/websocket_server.rs

use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, tungstenite::Message};
use futures::{SinkExt, StreamExt};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock, Notify};
use std::net::SocketAddr;
use std::time::Duration;

// クライアント情報
#[derive(Debug, Clone)]
pub struct ClientInfo {
    pub id: ClientId,
    pub addr: SocketAddr,
    pub connected_at: std::time::Instant,
}

pub type ClientId = u64;

// メッセージタイプ
#[derive(Debug, Clone)]
pub enum ServerMessage {
    Broadcast(String),
    Direct { target: ClientId, content: String },
    ClientConnected(ClientInfo),
    ClientDisconnected(ClientId),
    Error(String),
}

// WebSocketサーバー
pub struct WebSocketServer {
    clients: Arc<RwLock<HashMap<ClientId, ClientHandle>>>,
    next_client_id: Arc<RwLock<ClientId>>,
    broadcast_sender: mpsc::UnboundedSender<ServerMessage>,
    shutdown_notify: Arc<Notify>,
}

struct ClientHandle {
    info: ClientInfo,
    sender: mpsc::UnboundedSender<Message>,
}

impl WebSocketServer {
    pub fn new() -> (Self, mpsc::UnboundedReceiver<ServerMessage>) {
        let (broadcast_tx, broadcast_rx) = mpsc::unbounded_channel();
        
        let server = WebSocketServer {
            clients: Arc::new(RwLock::new(HashMap::new())),
            next_client_id: Arc::new(RwLock::new(1)),
            broadcast_sender: broadcast_tx,
            shutdown_notify: Arc::new(Notify::new()),
        };
        
        (server, broadcast_rx)
    }
    
    // サーバー開始
    pub async fn start(&self, addr: &str) -> Result<(), Box<dyn std::error::Error>> {
        let listener = TcpListener::bind(addr).await?;
        println!("WebSocket server listening on: {}", addr);
        
        let clients = Arc::clone(&self.clients);
        let next_client_id = Arc::clone(&self.next_client_id);
        let broadcast_sender = self.broadcast_sender.clone();
        let shutdown_notify = Arc::clone(&self.shutdown_notify);
        
        loop {
            tokio::select! {
                accept_result = listener.accept() => {
                    match accept_result {
                        Ok((stream, addr)) => {
                            let client_id = {
                                let mut id = next_client_id.write().await;
                                let current_id = *id;
                                *id += 1;
                                current_id
                            };
                            
                            println!("New client connection: {} (ID: {})", addr, client_id);
                            
                            let clients = Arc::clone(&clients);
                            let broadcast_sender = broadcast_sender.clone();
                            let shutdown_notify = Arc::clone(&shutdown_notify);
                            
                            tokio::spawn(async move {
                                if let Err(e) = Self::handle_client(
                                    stream,
                                    addr,
                                    client_id,
                                    clients,
                                    broadcast_sender,
                                    shutdown_notify,
                                ).await {
                                    eprintln!("Client {} error: {}", client_id, e);
                                }
                            });
                        }
                        Err(e) => {
                            eprintln!("Failed to accept connection: {}", e);
                        }
                    }
                }
                _ = shutdown_notify.notified() => {
                    println!("Server shutdown requested");
                    break;
                }
            }
        }
        
        // 全クライアントを切断
        self.disconnect_all_clients().await;
        
        Ok(())
    }
    
    // クライアントハンドリング
    async fn handle_client(
        stream: TcpStream,
        addr: SocketAddr,
        client_id: ClientId,
        clients: Arc<RwLock<HashMap<ClientId, ClientHandle>>>,
        broadcast_sender: mpsc::UnboundedSender<ServerMessage>,
        shutdown_notify: Arc<Notify>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let ws_stream = accept_async(stream).await?;
        let (mut ws_sender, mut ws_receiver) = ws_stream.split();
        
        let (client_tx, mut client_rx) = mpsc::unbounded_channel();
        
        let client_info = ClientInfo {
            id: client_id,
            addr,
            connected_at: std::time::Instant::now(),
        };
        
        // クライアントを登録
        {
            let mut clients_map = clients.write().await;
            clients_map.insert(client_id, ClientHandle {
                info: client_info.clone(),
                sender: client_tx,
            });
        }
        
        // 接続通知をブロードキャスト
        let _ = broadcast_sender.send(ServerMessage::ClientConnected(client_info.clone()));
        
        // メッセージ送信タスク
        let send_task = {
            let shutdown_notify = Arc::clone(&shutdown_notify);
            tokio::spawn(async move {
                loop {
                    tokio::select! {
                        msg = client_rx.recv() => {
                            match msg {
                                Some(message) => {
                                    if let Err(e) = ws_sender.send(message).await {
                                        eprintln!("Failed to send message to client {}: {}", client_id, e);
                                        break;
                                    }
                                }
                                None => break,
                            }
                        }
                        _ = shutdown_notify.notified() => {
                            break;
                        }
                    }
                }
            })
        };
        
        // メッセージ受信タスク
        let receive_task = {
            let broadcast_sender = broadcast_sender.clone();
            let shutdown_notify = Arc::clone(&shutdown_notify);
            
            tokio::spawn(async move {
                loop {
                    tokio::select! {
                        msg = ws_receiver.next() => {
                            match msg {
                                Some(Ok(message)) => {
                                    if let Err(e) = Self::handle_client_message(
                                        client_id,
                                        message,
                                        broadcast_sender.clone(),
                                    ).await {
                                        eprintln!("Error handling message from client {}: {}", client_id, e);
                                        break;
                                    }
                                }
                                Some(Err(e)) => {
                                    eprintln!("WebSocket error for client {}: {}", client_id, e);
                                    break;
                                }
                                None => {
                                    println!("Client {} disconnected", client_id);
                                    break;
                                }
                            }
                        }
                        _ = shutdown_notify.notified() => {
                            break;
                        }
                    }
                }
            })
        };
        
        // いずれかのタスクが終了するまで待機
        tokio::select! {
            _ = send_task => {},
            _ = receive_task => {},
        }
        
        // クライアントを削除
        {
            let mut clients_map = clients.write().await;
            clients_map.remove(&client_id);
        }
        
        // 切断通知をブロードキャスト
        let _ = broadcast_sender.send(ServerMessage::ClientDisconnected(client_id));
        
        println!("Client {} cleanup completed", client_id);
        Ok(())
    }
    
    // クライアントメッセージの処理
    async fn handle_client_message(
        client_id: ClientId,
        message: Message,
        broadcast_sender: mpsc::UnboundedSender<ServerMessage>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match message {
            Message::Text(text) => {
                println!("Message from client {}: {}", client_id, text);
                
                // メッセージをブロードキャスト
                let broadcast_msg = format!("Client {}: {}", client_id, text);
                let _ = broadcast_sender.send(ServerMessage::Broadcast(broadcast_msg));
            }
            Message::Close(_) => {
                println!("Close message from client {}", client_id);
            }
            Message::Ping(_data) => {
                // Pongは自動的に送信される
                println!("Ping from client {}", client_id);
            }
            _ => {
                // 他のメッセージタイプは無視
            }
        }
        
        Ok(())
    }
    
    // メッセージをブロードキャスト
    pub async fn broadcast_message(&self, content: String) {
        let clients = self.clients.read().await;
        let message = Message::Text(content);
        
        for (id, client) in clients.iter() {
            if client.sender.send(message.clone()).is_err() {
                println!("Failed to send broadcast to client {}", id);
            }
        }
    }
    
    // 特定クライアントにメッセージ送信
    pub async fn send_to_client(&self, client_id: ClientId, content: String) -> Result<(), String> {
        let clients = self.clients.read().await;
        
        if let Some(client) = clients.get(&client_id) {
            let message = Message::Text(content);
            client.sender.send(message)
                .map_err(|_| format!("Failed to send message to client {}", client_id))?;
            Ok(())
        } else {
            Err(format!("Client {} not found", client_id))
        }
    }
    
    // 接続中のクライアント一覧
    pub async fn get_clients(&self) -> Vec<ClientInfo> {
        let clients = self.clients.read().await;
        clients.values().map(|handle| handle.info.clone()).collect()
    }
    
    // 全クライアント切断
    async fn disconnect_all_clients(&self) {
        let clients = self.clients.read().await;
        
        for (id, client) in clients.iter() {
            let _ = client.sender.send(Message::Close(None));
            println!("Sent close message to client {}", id);
        }
    }
    
    // サーバーシャットダウン
    pub async fn shutdown(&self) {
        self.shutdown_notify.notify_waiters();
    }
}

// 使用例
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (server, mut message_receiver) = WebSocketServer::new();
    
    // メッセージ処理タスク
    let server_for_messages = Arc::new(server);
    let message_handler = {
        let server = Arc::clone(&server_for_messages);
        tokio::spawn(async move {
            while let Some(message) = message_receiver.recv().await {
                match message {
                    ServerMessage::Broadcast(content) => {
                        server.broadcast_message(content).await;
                    }
                    ServerMessage::Direct { target, content } => {
                        if let Err(e) = server.send_to_client(target, content).await {
                            eprintln!("Failed to send direct message: {}", e);
                        }
                    }
                    ServerMessage::ClientConnected(info) => {
                        println!("Client connected: {:?}", info);
                        let welcome_msg = format!("Welcome client {}! Server time: {:?}", 
                            info.id, std::time::SystemTime::now());
                        let _ = server.send_to_client(info.id, welcome_msg).await;
                    }
                    ServerMessage::ClientDisconnected(id) => {
                        println!("Client {} disconnected", id);
                    }
                    ServerMessage::Error(e) => {
                        eprintln!("Server error: {}", e);
                    }
                }
            }
        })
    };
    
    // 定期的なブロードキャストタスク
    let periodic_broadcast = {
        let server = Arc::clone(&server_for_messages);
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(30));
            
            loop {
                interval.tick().await;
                
                let clients = server.get_clients().await;
                let status_msg = format!("Server status: {} clients connected at {:?}", 
                    clients.len(), std::time::SystemTime::now());
                
                server.broadcast_message(status_msg).await;
            }
        })
    };
    
    // Ctrl+C ハンドリング
    let shutdown_server = Arc::clone(&server_for_messages);
    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.expect("Failed to listen for ctrl-c");
        println!("Ctrl+C received, shutting down server...");
        shutdown_server.shutdown().await;
    });
    
    // サーバー開始
    let result = server_for_messages.start("127.0.0.1:8080").await;
    
    // タスクを停止
    message_handler.abort();
    periodic_broadcast.abort();
    
    result
}
#[cfg(test)]
mod broadcast_tests {
    use super::*;

    fn handle(id: ClientId, sender: mpsc::UnboundedSender<Message>) -> ClientHandle {
        ClientHandle {
            info: ClientInfo {
                id,
                addr: "127.0.0.1:1".parse().unwrap(),
                connected_at: std::time::Instant::now(),
            },
            sender,
        }
    }

    #[tokio::test]
    async fn broadcast_reaches_live_clients_and_tolerates_closed_ones() {
        let (server, _events) = WebSocketServer::new();
        let (live_tx, mut live_rx) = mpsc::unbounded_channel();
        let (closed_tx, closed_rx) = mpsc::unbounded_channel();
        drop(closed_rx);
        {
            let mut clients = server.clients.write().await;
            clients.insert(1, handle(1, live_tx));
            clients.insert(2, handle(2, closed_tx));
        }

        server.broadcast_message("hello".to_string()).await;

        assert_eq!(live_rx.try_recv().unwrap(), Message::Text("hello".to_string()));
    }
}
