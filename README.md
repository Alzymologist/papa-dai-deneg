# papa-dai-deneg

A command-line utility that allows the user to send any amount of funds from //Alice on a Substrate-based test blockchain (we know she has inexhaustible wealth).

## Installation

```bash
cargo build
```
Build for my server variant:

```bash
RUSTFLAGS='-C target-cpu=x86-64-v3'
cargo +nightly build --target x86_64-unknown-linux-gnu --release
```
## Usage

```bash
./papa-dai-deneg 0x8094a91dc4d98a6112374c599d4ed6592a1862d7cda654ee74ecb649ca427a4c 10000 --please wss://node-shave.zymologia.fi:443
```

Arguments:

1. User account in the format 0x...;
2. Amount of hypothetical money;
3. Mandatory flag --please for compatibility with stubborn UNIX users and others who can't live without such magic in command lines;
4. Address of your blockchain with a mandatory port specification, even if it's 80 or 443;

Debugging launch:

```bash
cargo watch -c -x "run -- 0x8094a91dc4d98a6112374c599d4ed6592a1862d7cda654ee74ecb649ca427a4c 10000 --please wss://node-shave.zymologia.fi:443"

```

## Other:

Example backend call in json.php:

```php
<?php
$j=file_get_contents('php://input');
$j=(array)json_decode($j);
$user=$j['user'];
if(preg_match("/[^0-9abcdefx]+/si",$user)) ejie('wrong user');
$money=7000; // $j['money'];
unset($o); exec("/home/nodes/INSTALL/papa-dai-deneg ".$user." ".(1*$money)." --please wss://node-shave.zymologia.fi:443 2>&1",$o);
die(json_encode($o));
?>
```
