<?php
    $version='1.04';

    $s=file_get_contents("php://input");
    $json=(array)json_decode($s);

    // если ошибка
    if(empty($json)) er('json','wrong input'); // {"error":"json", "error_message":"wrong input"}
    $price=$json['price'];
    $OrderID=$json['OrderID'];
    $about=$json['about'];

    // если нулевая цена
    if($json['price']==0) er('json','wrong price'); // {"error":"json", "error_message":"wrong price"}

    // иногда платеж уже готов
    if(rand(0,10)>5) ok('payd',array()); // {"result":"payd"}

    // а иногда вот вам адрес для оплаты
    ok('ready',array('account'=>'0xd619041b935c9697d37259e408b1de0e3f5fd9663f49e1b4654bbae1e0fe6962'));
// {"result":"ready","account":"0xd619041b935c9697d37259e408b1de0e3f5fd9663f49e1b4654bbae1e0fe6962"}


function er($err,$mes) { send(array('error'=>$err,'error_message'=>$mes)); }
function ok($res,$a) { $a['result']=$res; send($a); }
function send($a) {
    $a['version']=$GLOBALS['version'];
    $add=explode(' ',"version price OrderID about"); foreach($add as $l) if(isset($GLOBALS[$l])) $a[$l]=$GLOBALS[$l];
    header('Content-Type: application/json');
    die(json_encode($a,JSON_UNESCAPED_UNICODE));
}

?>