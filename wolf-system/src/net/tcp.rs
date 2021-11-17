use super::{
    callback::{OnMessageCallback, OnSocketCallback},
    tls::{load_root_ca, TlsPrivateKeyType},
};
use crate::{chrono::gametime::GameTime, net::tls};
use anyhow::{anyhow, ensure};
use std::{net::SocketAddr, path::Path, str::FromStr, sync::Arc};
use std::{
    sync::{
        mpsc::{Receiver, Sender},
        Mutex,
    },
    time::Duration,
};
use tokio::{
    io::{split, AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    time::Instant,
};
use tokio_rustls::{
    rustls::{self, ServerConfig},
    TlsAcceptor, TlsConnector,
};

const MAX_BUFFER_SIZE: usize = 1024; //1K

async fn accept_connection<T>(
    p_stream: T,
    p_peer_address: SocketAddr,
    p_timeout_in_seconds: f64,
    p_on_accept_callback: OnSocketCallback,
    p_on_msg_callback: OnMessageCallback,
    p_on_close_callback: OnSocketCallback,
) -> anyhow::Result<()>
where
    T: tokio::io::AsyncRead + tokio::io::AsyncWrite,
{
    // check for accept this connection
    p_on_accept_callback.run(&p_peer_address)?;

    // split stream into reader and writer
    let (mut reader, mut writer) = split(p_stream);

    // don't read more than 1K
    let mut buf = [0u8; MAX_BUFFER_SIZE];
    let mut size: usize;
    let mut socket_io_timeout = Instant::now();
    let mut socket_live_time = GameTime::new();
    socket_live_time.set_fixed_time_step(false);

    // let's loop for read and writing to the socket
    loop {
        socket_live_time.tick();
        // read buffer
        size = reader.read(&mut buf).await?;
        if size > 0 {
            socket_io_timeout = Instant::now();
            p_on_msg_callback.run(&socket_live_time, &p_peer_address, &mut size, &mut buf)?;
            if size > 0 {
                let v = buf[0..size].to_vec();
                writer.write(&v).await?;
                writer.flush().await?;
            }
        } else {
            let dur = Instant::now() - socket_io_timeout;
            if dur.as_secs_f64() > p_timeout_in_seconds {
                break;
            }
        }
    }
    let ret = p_on_close_callback.run(&p_peer_address);
    ret
}

async fn tls_acceptor(
    p_tls_certificate_path: Option<&Path>,
    p_tls_private_key_path: Option<&Path>,
    p_tls_private_type: Option<&TlsPrivateKeyType>,
) -> anyhow::Result<TlsAcceptor> {
    //tls-mode
    let err_msg = "Invalid Parameters for tcp::run_server";

    //load certificate
    let crt = p_tls_certificate_path
        .ok_or(anyhow!(err_msg).context("p_tls_certificate_path not provided for tcp server"))?;
    let certs = tls::load_certs(crt).await?;
    ensure!(certs.len() == 0, "missing certificate for TLS tcp server");

    //load private key
    let key = p_tls_private_key_path
        .ok_or(anyhow!(err_msg).context("p_tls_private_key_path not provided for tcp server"))?;
    //load private key type
    let key_type = p_tls_private_type
        .ok_or(anyhow!(err_msg).context("p_tls_private_type not provided for tcp server"))?;
    let mut keys = tls::load_private_keys(key, key_type).await?;
    ensure!(keys.len() == 0, "missing private key for TLS tcp server");

    //create tls config
    let tls_server_config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(certs, keys.remove(0))
        .map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidInput, err))?;

    // run acceptor & listener
    let tls_acceptor = TlsAcceptor::from(Arc::new(tls_server_config));
    Ok(tls_acceptor)
}

pub async fn timeout_for_accept(
    p_timeout_in_secs: f64,
) -> std::io::Result<(TcpStream, SocketAddr)> {
    use std::io::{Error, ErrorKind};

    tokio::time::sleep(Duration::from_secs_f64(p_timeout_in_secs)).await;
    //return Error
    Err(Error::new(
        ErrorKind::TimedOut,
        "timeout for accept connection reached",
    ))
}

pub async fn accept(p_tcp_listener: &TcpListener) -> std::io::Result<(TcpStream, SocketAddr)> {
    p_tcp_listener.accept().await
}

pub async fn server(
    p_address: &str,
    p_port: u16,
    p_ttl: u32,
    p_accept_timeout_in_secs: f64,
    p_read_write_timeout_in_secs: f64,
    p_tls: bool,
    p_tls_certificate_path: Option<&Path>,
    p_tls_private_key_path: Option<&Path>,
    p_tls_private_type: Option<&TlsPrivateKeyType>,
    p_close_sig_channel: &Mutex<(Sender<bool>, Receiver<bool>)>,
    p_on_bind_socket: OnSocketCallback,
    p_on_accept_connection: OnSocketCallback,
    p_on_message: OnMessageCallback,
    p_on_close_connection: OnSocketCallback,
    p_on_close_socket: OnSocketCallback,
) -> anyhow::Result<()> {
    let address = format!("{}:{}", p_address, p_port);
    let socket_addr = SocketAddr::from_str(&address)?;

    // bind to the socket
    let tcp_listener = TcpListener::bind(socket_addr).await?;
    tcp_listener.set_ttl(p_ttl)?;

    p_on_bind_socket.run(&socket_addr)?;

    let mut tls_acc: Option<TlsAcceptor> = None;

    if p_tls {
        //create tls acceptor
        let tls = tls_acceptor(
            p_tls_certificate_path,
            p_tls_private_key_path,
            p_tls_private_type,
        )
        .await?;
        tls_acc = Option::from(tls);
    }

    loop {
        //close current socket, if it was requested by close signal channel
        let close_res = p_close_sig_channel.try_lock();
        let close = match close_res {
            Ok(chan) => {
                if let Ok(b) = chan.1.try_recv() {
                    b
                } else {
                    false
                }
            }
            Err(_) => false,
        };
        if close {
            break;
        }

        //try for accept incomming session within specific timout
        let _res = tokio::select! {
            res1 = timeout_for_accept(p_accept_timeout_in_secs) =>
            {
                res1
            },
            res2 = accept(&tcp_listener) =>
            {
                res2
            },
        };

        if _res.is_err() {
            //timeout reached or could not accept incomming connectoin successfully
            continue;
        }

        if p_tls {
            // start main loop
            if let Ok((stream, peer_addr)) = _res {
                //clone necessary objects
                let tls = tls_acc.clone().unwrap();
                let tls_on_message = p_on_message.clone();
                let tls_on_accept_con = p_on_accept_connection.clone();
                let tls_on_close_con = p_on_close_connection.clone();

                // accept a new tls connection
                tokio::spawn(async move {
                    let res = tls.accept(stream).await;
                    let ret = match res {
                        Ok(tls_stream) => {
                            accept_connection(
                                tls_stream,
                                peer_addr,
                                p_read_write_timeout_in_secs,
                                tls_on_accept_con,
                                tls_on_message,
                                tls_on_close_con,
                            )
                            .await
                        }
                        Err(e) => {
                            anyhow::bail!("cloud not accept tls. because {:?}", e)
                        }
                    };
                    ret
                });
            }
        } else {
            //no tls-mode
            if let Ok((stream, peer_addr)) = _res {
                // accept a new connection
                tokio::spawn(accept_connection(
                    stream,
                    peer_addr,
                    p_read_write_timeout_in_secs,
                    p_on_accept_connection.clone(),
                    p_on_message.clone(),
                    p_on_close_connection.clone(),
                ));
            }
        }
    }

    // on close socket
    let ret = p_on_close_socket.run(&socket_addr);
    ret
}

async fn handle_client_stream<T>(
    p_stream: T,
    p_peer_address: &SocketAddr,
    p_on_message: OnMessageCallback,
) -> anyhow::Result<()>
where
    T: tokio::io::AsyncRead + tokio::io::AsyncWrite,
{
    // don't read more than 1K
    let mut buf = [0u8; MAX_BUFFER_SIZE];
    let mut size = 0;
    let mut socket_live_time = GameTime::new();
    socket_live_time.set_fixed_time_step(false);

    let (mut reader, mut writer) = split(p_stream);

    // let's loop for read and writing to the socket
    loop {
        socket_live_time.tick();
        if p_on_message
            .run(&socket_live_time, &p_peer_address, &mut size, &mut buf)
            .is_err()
        {
            break;
        }
        if size > 0 {
            let v = buf[0..size].to_vec();
            writer.write(&v).await?;
            writer.flush().await?;
        }
        // read buffer
        size = reader.read(&mut buf).await?;
    }

    Ok(())
}

pub async fn client(
    p_domain_address: &str,
    p_port: u16,
    p_tls: bool,
    p_tls_ca_path: Option<&Path>,
    p_on_accept_connection: OnSocketCallback,
    p_on_message: OnMessageCallback,
    p_on_close_connection: OnSocketCallback,
) -> anyhow::Result<()> {
    //create address
    let address = format!("{}:{}", p_domain_address, p_port);
    let socket_addr = SocketAddr::from_str(&address)?;

    p_on_accept_connection.run(&socket_addr)?;

    if p_tls {
        // create root cert store
        let root_cert_store = load_root_ca(p_tls_ca_path).await?;
        // load tls config
        let config = rustls::ClientConfig::builder()
            .with_safe_defaults()
            .with_root_certificates(root_cert_store)
            .with_no_client_auth();
        // create tls connector
        let connector = TlsConnector::from(Arc::new(config));
        // create tcp stream
        let stream = TcpStream::connect(&socket_addr).await?;

        // domain
        let domain = rustls::ServerName::try_from(p_domain_address).map_err(|e| {
            anyhow!("invalid dnsname {:?}", e).context("invalid dnsname tcp::client")
        })?;
        let peer_address = stream.peer_addr()?;

        // create tls tcp stream
        let stream = connector.connect(domain, stream).await?;
        handle_client_stream(stream, &peer_address, p_on_message).await?;
    } else {
        // create tcp stream
        let stream = TcpStream::connect(&socket_addr).await?;
        let peer_address = stream.peer_addr()?;

        handle_client_stream(stream, &peer_address, p_on_message).await?;
    }
    let ret = p_on_close_connection.run(&socket_addr);
    ret
}

#[tokio::main]
#[test]
async fn test() -> () {
    use std::sync::mpsc::channel;
    use std::time::Duration;

    lazy_static::lazy_static! {
        static ref CHANNEL_MUTEX: Mutex<(Sender<bool>, Receiver<bool>)> = Mutex::new(channel::<bool>());
    }

    tokio::spawn(async move {
        // wait for server to be created
        tokio::time::sleep(Duration::from_secs(2)).await;

        let on_accept_connection = OnSocketCallback::new(Box::new(
            |p_socket_address: &SocketAddr| -> anyhow::Result<()> {
                println!("client {:?} just connected to the server", p_socket_address);
                Ok(())
            },
        ));
        let on_close_connection = OnSocketCallback::new(Box::new(
            |p_socket_address: &SocketAddr| -> anyhow::Result<()> {
                println!("client {:?} just closed", p_socket_address);
                //wait for 1 seconds and then close the server
                std::thread::sleep(Duration::from_secs(1));
                //send request to close the server socket
                let _ = CHANNEL_MUTEX.lock().and_then(|channel| {
                    let _ = channel.0.send(true).and_then(|_| Ok(())).or_else(|e| {
                        println!("could not send data to close_sig_channel. error: {:?}", e);
                        Err(e)
                    });
                    Ok(())
                });
                Ok(())
            },
        ));

        let on_msg_callback = OnMessageCallback::new(Box::new(
            |p_socket_live_time: &GameTime,
             p_peer_address: &SocketAddr,
             p_size: &mut usize,
             p_buffer: &mut [u8]|
             -> anyhow::Result<()> {
                println!(
                    "client: number of received byte(s) from {:?} is {}. socket live time {}",
                    p_peer_address,
                    *p_size,
                    p_socket_live_time.get_total_elapsed_seconds()
                );

                if *p_size > 0 {
                    let msg = std::str::from_utf8(&p_buffer)?;
                    println!("client: received buffer is {}", msg);
                }
                //now store new bytes for write
                let msg = "hello...world!\0"; //make sure append NULL terminate
                p_buffer[0..msg.len()].copy_from_slice(msg.as_bytes());
                *p_size = msg.len();

                if p_socket_live_time.get_total_elapsed_seconds() > 10.0 {
                    anyhow::bail!("closing socket");
                }
                Ok(())
            },
        ));
        let ret = client(
            "0.0.0.0",
            8000,
            false,
            None,
            on_accept_connection,
            on_msg_callback,
            on_close_connection,
        )
        .await;
        assert!(ret.is_ok(), "{:?}", ret);
    });

    // block main thread with tcp server
    let on_bind_socket = OnSocketCallback::new(Box::new(
        |p_socket_address: &SocketAddr| -> anyhow::Result<()> {
            println!("server: socket {:?} just binded", p_socket_address);
            Ok(())
        },
    ));

    let on_accept_connection = OnSocketCallback::new(Box::new(
        |p_socket_address: &SocketAddr| -> anyhow::Result<()> {
            println!(
                "server: remote address with peer id {:?} just connected",
                p_socket_address
            );
            Ok(())
        },
    ));

    let on_msg_callback = OnMessageCallback::new(Box::new(
        |p_socket_live_time: &GameTime,
         p_peer_address: &SocketAddr,
         p_size: &mut usize,
         p_buffer: &mut [u8]|
         -> anyhow::Result<()> {
            println!(
                "server: number of received byte(s) from {:?} is {}. socket live time {}",
                p_peer_address,
                *p_size,
                p_socket_live_time.get_total_elapsed_seconds()
            );
            if *p_size > 0 {
                let msg = std::str::from_utf8(&p_buffer)?;
                println!("server: received buffer is {}", msg);

                //now store new bytes for write
                let msg = "hello client!\0"; //make sure append NULL terminate
                p_buffer[0..msg.len()].copy_from_slice(msg.as_bytes());
                *p_size = msg.len();
            }
            Ok(())
        },
    ));

    let on_close_connection = OnSocketCallback::new(Box::new(
        |p_socket_address: &SocketAddr| -> anyhow::Result<()> {
            println!(
                "server: remote address with peer id {:?} just disconnected",
                p_socket_address
            );
            Ok(())
        },
    ));

    let on_close_socket = OnSocketCallback::new(Box::new(
        |p_socket_address: &SocketAddr| -> anyhow::Result<()> {
            println!("server: socket {:?} just closed", p_socket_address);
            Ok(())
        },
    ));

    let ret = server(
        "0.0.0.0",
        8000,
        100,
        0.3, //300 milliseconds
        2.0, // 2seconds
        false,
        None,
        None,
        None,
        &CHANNEL_MUTEX,
        on_bind_socket,
        on_accept_connection,
        on_msg_callback,
        on_close_connection,
        on_close_socket,
    )
    .await;

    println!("tcp tests done {:?}", ret);
}