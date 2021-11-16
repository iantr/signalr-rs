use futures::io;
use actix_rt;
use url::Url;
use signalr_rs::hub::client::{HubClientBuilder, HubClientError, HubClientHandler, HubQuery, PendingQuery};
use serde_json::Value;
use actix::System;
use openssl::ssl::{SslMethod, SslConnector};
use serde::de::DeserializeOwned;
// use serde::Deserialize;
use libflate::deflate::Decoder;
use std::io::Read;

struct ChatHubHandler {
    hub: String,
}

impl ChatHubHandler {
    fn deflate<T>(binary: &str) -> Result<T, HubClientError>
    where
        T: DeserializeOwned,
    {
        let decoded = base64::decode(binary)?;
        let mut decoder = Decoder::new(&decoded[..]);
        let mut decoded_data: Vec<u8> = Vec::new();
        decoder.read_to_end(&mut decoded_data)?;
        let v: &[u8] = &decoded_data;
        serde_json::from_slice::<T>(v).map_err(HubClientError::ParseError)
    }

    fn deflate_array<T>(a: &Value) -> Result<T, HubClientError>
    where
        T: DeserializeOwned,
    {
        let data: Vec<String> = serde_json::from_value(a.clone())?;
        let binary = data.first().ok_or(HubClientError::MissingData)?;
        ChatHubHandler::deflate::<T>(binary)
    }

    fn deflate_string<T>(a: &Value) -> Result<T, HubClientError>
    where
        T: DeserializeOwned,
    {
        let binary: String = serde_json::from_value(a.clone())?;
        ChatHubHandler::deflate::<T>(&binary)
    }
}

impl HubClientHandler for ChatHubHandler {
    fn on_connect(&self) -> Vec<Box<dyn PendingQuery>> {
        println!("Connected!");

        let pairs = vec![
            "ReceiveMessage",
            "USDT-BCC",
            "USDT-BTC",
            "USDT-DASH",
            "USDT-ETC",
            "USDT-ETH",
            "USDT-LTC",
            "USDT-NEO",
            "USDT-OMG",
            "USDT-XMR",
            "USDT-XRP",
            "USDT-ZEC",
        ];
        pairs
            .into_iter()
            .enumerate()
            .map(|(i, p)| {
                Box::new(HubQuery::new(
                    self.hub.to_string(),
                    "SubscribeToExchangeDeltas".to_string(),
                    vec![p.to_string()],
                    (i + 1).to_string(),
                )) as Box<dyn PendingQuery>
            })
            .collect()
    }

    fn error(&self, _id: Option<&str>, _msg: &Value) {
        println!("Error: {:?}", _msg)
    }

    fn handle(&mut self, method: &str, message: &Value) {
            println!("Message: '{:?}'  method: {:?}", message, method)
    }
}


#[actix_rt::main]
async fn main() -> io::Result<()> {
    env_logger::init();

    println!("SignalR Console Rust Client");

    // var url = "https://localhost:57173/chathub"; // was :5001
    let url = Url::parse("https://localhost:5001/");

    // var hubConnection = new HubConnectionBuilder()
    // .WithUrl(url)
    // .WithAutomaticReconnect()
    // .Build();

    let hub = "chathub";
    let handler = Box::new(ChatHubHandler { hub: hub.to_string() });

    // Set up the SSL connector so verification can be turned off for now
    let mut builder = SslConnector::builder(SslMethod::tls()).unwrap();
    builder.set_verify(openssl::ssl::SslVerifyMode::NONE);
    let connector = builder.build();

    let client = HubClientBuilder::with_hub_and_url(hub, url.unwrap())
        .ssl_connector(connector)
        .start_supervised(handler)
        .await;

    // Avoid certificate errors for now
    // ServicePointManager.ServerCertificateValidationCallback += (sender, cert, chain, sslPolicyErrors) => true;

    println!("Client started");

    match client {
        Ok(addr) => {
            addr.do_send(HubQuery::new(
                hub.to_string(),
                "SendMessage".to_string(),
                vec!["Rust Console App"],
                "Message from the Rust console app".to_string(),
            ));
        }
        Err(e) => {
            println!("Hub client error : {:?}", e);
            System::current().stop();
        }
    }

    actix_rt::signal::ctrl_c().await
}
