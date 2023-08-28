# get-caller-file

Консольная утилита, которая позволяет перечислить пользователю тестового блокчейна типа Substrate любую сумму денег от //Alice (мы знаем, у неё денег неисчерпаемо).

## Installation

```bash
cargo build
```

Сборка для моего варианта сервера:

```bash
RUSTFLAGS='-C target-cpu=x86-64-v3'
cargo +nightly build --target x86_64-unknown-linux-gnu --release
```

## Usage

Given:

```bash
./papa-dai-deneg 0x8094a91dc4d98a6112374c599d4ed6592a1862d7cda654ee74ecb649ca427a4c 10000 --please wss://node-shave.zymologia.fi:443
```

Аргументы:

1. аккаунт юзера в формате 0x...;
2. сумма условных денег;
3. обязательный флаг --please для совместимости с красноглазыми юниксоидами и прочими занудами, которые без подобной магии в консольных строках жить не умеют;
4. адрес вашего блокчейна с обязательным указанием порта, даже если это 80 или 443;

Зупуск во время отладки:

```bash
cargo watch -c -x "run -- 0x8094a91dc4d98a6112374c599d4ed6592a1862d7cda654ee74ecb649ca427a4c 10000 --please wss://node-shave.zymologia.fi:443"

```

## Other:

Пример вызова в бэкенде json.php:

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
