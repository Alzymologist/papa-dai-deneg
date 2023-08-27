use crate::Context; // мешок Забайкалья

// для построения транзакций
use parity_scale_codec::{Compact, Encode}; // CompactLen, Encode, Decode,

#[derive(Debug)]
pub struct Tran { ara: Vec<u8>, }

impl Tran {

    fn sign(&mut self, a: Signature) {
        self.ara.extend_from_slice(&a.0);
    }

    fn bytes(&mut self, a: &[u8]) {
        self.ara.extend_from_slice(&a);
    }

    fn add_len(&mut self) {
        let ara2 = self.ara.clone();
        self.ara=[].to_vec();
        self.compact(ara2.len().try_into().unwrap());
        self.bytes(&ara2);
    }

    fn hex(&mut self, hex: &str) {
        let a: &[u8] = &hex::decode(hex).unwrap();
        for x in a { self.ara.push(*x); }
    }

    fn compact(&mut self, x: u128) {
        let a = Compact(x).encode();
        for x in a { self.ara.push(x); }
    }

    fn add_user(&mut self, name: &str) {
        self.hexstring( &user_name(name) );
    }

    fn hexstring(&mut self, name: &String) {
        let data = remove0x(name);
        let a: &[u8] = &hex::decode(data).unwrap();
        for x in a { self.ara.push(*x); }
    }

    fn clone(&mut self) -> Tran {
        let mut x = Tran { ara: [].to_vec() };
        x.ara.extend_from_slice(&self.ara);
        x
    }

    fn u32(&mut self, x: &u32) {
        self.ara.push( ((x)       & 0xFF) as u8 );
        self.ara.push( ((x >> 8)  & 0xFF) as u8 );
        self.ara.push( ((x >> 16) & 0xFF) as u8 );
        self.ara.push( ((x >> 24) & 0xFF) as u8 );
    }

}

// ================ подпись блокчейна =====================
use sp_core::{crypto::Pair, sr25519, sr25519::Signature};
pub fn singme(message: &[u8], account: &str) -> Option<Signature> {
    let pair = match sr25519::Pair::from_string(&account, None) {
            Ok(val) => val,
            Err(_) => {
                sr25519::Pair::from_string(&format!("//Alice"), None).ok()?
            },
    };
    Some(pair.sign(&message[..]))
}


// ================ добыть нужные параметры ================
use serde_json::Value;

pub async fn get_chain_ws(cx: &mut Context) {
    if cx.cl.is_none() { // установить соединение и взять нужные данные

        log!("Connecting to ", &cx.ws_url);
        cx.cl = Option::Some( neh!(WsClientBuilder::default().build(cx.ws_url.clone()).await) );
        let cl = cx.cl.as_ref().unwrap();

        // genesis_hash
        let s: Value = neh!( cl.request("chain_getBlockHash",rpc_params![0]).await);
        cx.genesis_hash = s.to_string().trim_matches('"').to_string();

        // specVersion и transactionVersion
        let s: Value = neh!( cl.request("chain_getRuntimeVersion",rpc_params![]).await);
        cx.spec_version = nah!(s["specVersion"].as_u64()) as u32;
        cx.transaction_version = nah!(s["transactionVersion"].as_u64()) as u32;
    
        // chain_getBlockHash
        let s: Value = neh!( cl.request("chain_getBlockHash",rpc_params![]).await);
        cx.block_hash = s.to_string().trim_matches('"').to_string();

            log!(format!(" spec_version={} transaction_version={}", &cx.spec_version, &cx.transaction_version) );
            log!(" genesis_hash", &cx.genesis_hash ); 
            // log!(" transaction_version", &cx.transaction_version );
            log!(" block_hash", &cx.block_hash );
    }
}

// ================ послать денег ==========================
use jsonrpsee::core::client::ClientT;
use jsonrpsee::{rpc_params, ws_client::WsClientBuilder};

pub async fn moke_send_money(cx: &mut Context, from: &str, to: &str, money: u128 ) -> Option<String> {  
    log!("Money transfer", &money);
    log!("FROM", &from);
    log!("TO  ", &to);
    // Не сходить ли нам установить соединение и скачать нужные параметры?
    get_chain_ws(cx).await;
    let cl = cx.cl.as_ref().unwrap();

    // Получить nonce для Alice
    let name = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY";
    let s: Value = neh!( cl.request("system_accountNextIndex",rpc_params![&name]).await );
    cx.nonce = nah!( s.as_u64() ) as u128;
    log!(" nonce",cx.nonce);

    // Сама операция, пример: 0500 00 8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48 04
    let mut t = Tran { ara: [].to_vec() };
    t.hex("0500"); // код транзакции
    t.hex("00"); // ноль
    t.add_user(to); // кому деньги
    t.compact(money); // сколько денег

    // Теперь делаем расширенную версию операции для подписи:
    let mut full = t.clone(); // само тело транзакции
    full.hex("00"); // Era: можно 00
    // full.hex("0501"); // Era: можно 00
    full.compact(cx.nonce); // Nonce:
    full.hex("00"); // Tip: чаевые, можно 00
    full.u32(&cx.spec_version); // SpecVersion: просто u32, без compact
    full.u32(&cx.transaction_version); // TransactionVersion: просто u32, без compact
    full.hexstring(&cx.genesis_hash.to_string()); // GenesisHash
    // full.hexstring(&cx.block_hash); // BlockHash
    full.hexstring(&cx.genesis_hash.to_string()); // GenesisHash

    // теперь бы ее суку как-то подписать Алисой:
    // let sign = singme( &full.ara, from )?;
    let sign = nah!( singme( &full.ara, from ) );

    // Так, теперь формируем целиком всю посылку:
    let mut tr = Tran { ara: [].to_vec() };
    tr.hex("8400"); // Начало 84 (compact от 33?) 00
    tr.add_user("Alice"); // From: Alice
    tr.hex("01"); // код 01
    tr.sign(sign); // здесь вставляем подпись
    tr.hex("00"); // Эра: можно 00, НЕ compact! TR.compact(era);
    // tr.hex("0501");
    tr.compact(cx.nonce); // Nonce
    tr.hex("00"); // Код 00
    tr.bytes(&t.ara); // сама операция (короткая версия)
    tr.add_len(); // и в начало добавим compact-длину всей этой мандулы
    // END

    // Выполнить транзакцию
    let str = format!("0x{}", &hex::encode(&tr.ara) );
    let s: Value = neh!( cl.request("author_submitAndWatchExtrinsic",rpc_params![&str]).await );
    // закрываем соединение
    drop( cl );
    Some(s.to_string().trim_matches('"').to_string())
}


// =============== user name ======================
use std::collections::HashMap;

pub fn user_name(name: &str) -> String {

    let oligarch = HashMap::from([
        ("d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","ALICE"),
        ("be5ddb1579b72e84524fc29e78609e3caf42e85aa118ebfe0b0ad404b5bdd25f","ALICE_STASH"),
        ("8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48","BOB"),
        ("fe65717dad0447d715f660a0a58411de509b42e6efb8375f562f58a554d5860e","BOB_STASH"),
        ("90b5ab205c6974c9ea841be688864633dc9ca8a357843eeacf2314649965fe22","CHARLIE"),
        ("306721211d5404bd9da88e0204360a1a9ab8b87c66c1bc2fcdd37f3c2222cc20","DAVE"),
        ("e659a7a1628cdd93febc04a4e0646ea20e9f5f0ce097d9a05290d4a9e054df4e","EVE"),
        ("1cbd2d43530a44705ad088af313e18f80b53ef16b36177cd4b77b846f2a5f07c","FERDIE"),
    ]);

    let data = remove0x(name);

    if 64 == data.len() { // это хэш
        let s = name.to_lowercase();
        return s;
    } else { // это name типа Alice
        let s = name.to_uppercase();
        for (a, b) in &oligarch {
            if b == &s { return format!("{a}"); }
        }
        return String::from("«{name}» _unknown_");
    }
}

pub fn remove0x(addr: &str) -> String {
    let s = addr.to_string();
    if &s[0..2] == "0x" { return (s[2..]).to_string() }
    s
}
// ===================================================================================
