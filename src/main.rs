#[warn(non_camel_case_types)]

// === три полезных макроса, без которых писать простые модельки в Rust невозможно

macro_rules! log {
    ($s:expr, $p:expr) => { println!("[ LOG ] {} : {:#?}",$s,$p); };
    ($s:expr) => { println!("[ LOG ] {}",$s); };
}

macro_rules! nah {
    ($s:expr) => {
        match $s {
            Some(l) => { l },
            None => { panic!("[ Error ]"); }
        }
    }
}

macro_rules! neh {
    ($s:expr) => {
        match $s {
            Ok(l) => { l },
            Err(l) => { panic!("[ Error ] {:#?}",l); },
        }
    }
}

// ===================

use reqwest::Error;
mod mokele_mbember;
use mokele_mbember::{moke_send_money};

#[derive(Debug)]
pub struct Context {
    nonce: u128,
    genesis_hash: String,  // выясняется раз и навсегда
    spec_version: u32, //  выясняется раз и навсегда
    transaction_version: u32, //  выясняется раз и навсегда
    block_hash: String,
    ws_url: String, // ws://localhost:9944
    cl: Option<jsonrpsee::ws_client::WsClient>,
}

use std::env;
use tokio::runtime::Runtime;

fn main() -> Result<(), Error> {

    // по диким степям Забайкалья бродяга тащился от отсутствия GLOBAL
    let mut cx = Context {
            genesis_hash: "".to_string(),
            spec_version: 0,
            transaction_version: 0,
            block_hash: "".to_string(),
            nonce: 0,
            ws_url: "".to_string(), // "wss://node-shave.zymologia.fi:443".to_string(),
            cl: None,
    };

    // Аргументы командной строки
    let args: Vec<String> = env::args().collect();
    if args.len() != 5 { panic!("USE:\n\n./papa-dai-deneg 0x8094a91dc4d98a6112374c599d4ed6592a1862d7cda654ee74ecb649ca427a4c 9999 --please ws://localhost:9944"); }
    if &args[3] != "--please" {
	// panic!("Ты просишь денег, но делаешь это без уважения.");
	panic!("But now you come to me and say Don Corleone, you must give me money. And you don't ask in respect or friendship.");
    }
    let user_to = args[1].as_str();
    let money_to = neh!( args[2].parse::<u128>() );

    cx.ws_url = args[4].clone(); // "wss://node-shave.zymologia.fi:443".to_string(),

    let rt = Runtime::new().unwrap();

    let x = nah!( rt.block_on( moke_send_money(&mut cx, "Alice", user_to, money_to ) ) );
    println!("OK {}",&x);

    Ok(())
}