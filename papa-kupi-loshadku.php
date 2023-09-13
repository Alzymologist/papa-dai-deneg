#!/usr/bin/php
<?php

$node='wss://node-shave.zymologia.fi:443';
$url='http://lleo.me/dot.php';
$sleep = 2;

$version='1.04';

if(isset($argv[1])) $node=$argv[1];
if(isset($argv[2]) && $argv[2]) $url=$argv[2];
if(isset($argv[3]) && 1*$argv[3]) $sleep=$argv[3];

echo "php ./papa-kupi-loshadku.php wss://node-shave.zymologia.fi:443 http://lleo.me/dot.php 2\n";
echo "Make a payment every ".$sleep." sec with node: ".$node." and ajax-daemon: ".$url."\n";

while(1) {
    echo "\n";
    echo "          _   _                                      _\n";
    echo "         | \ | | _____      __  _ __  _   _ _ __ ___| |__   __ _ ___  ___"."\n";
    echo "         |  \\| |/ _ \\ \\ /\\ / / | '_ \\| | | | '__/ __| '_ \\ / _` / __|/ _ \\"."\n";
    echo "         | |\\  |  __/\\ V  V /  | |_) | |_| | | | (__| | | | (_| \\__ \\  __/"."\n";
    echo "         |_| \\_|\\___| \\_/\\_/   | .__/ \\__,_|_|  \\___|_| |_|\\__,_|___/\\___|"."\n";
    echo "                               |_|"."\n";
    echo "\n";

    $OrderID=time();
    list($price,$about) = creative();

    $json='{
	"OrderID":'.$OrderID.',
	"price":'.$price.',
	"about":"'.$about.'"
    }';

    $kk=0;

    $money_transferred = 0;

    while(1) { // цикл работы с одной штукой
	$kk++;

        $r = ajax($json);

        // если пустое
	if(sizeof($r)==0) { echo "ERROR: empty result!"; break; }

        // если нет версии
	if(!isset($r['version'])) { echo "ERROR: version not found!"; break; }

        // если чего-то не хватает
	$x='version';
	    if(!isset($r[$x])) { echo "[  ERROR  ]: `$x` not found\n\n"; break; }
	    if($$x != $r[$x]) { echo "[  ERROR  ]: error $x: [".$$x."] != [".$r[$x]."]\n\n"; break; }

        $x='price';
	    if(!isset($r[$x])) { echo "[  ERROR  ]: `$x` not found\n\n"; break; }
	    if($$x != $r[$x]) { echo "[  ERROR  ]: error $x: [".$$x."] != [".$r[$x]."]\n\n"; break; }

        $x='OrderID';
	    if(!isset($r[$x])) { echo "[  ERROR  ]: `$x` not found\n\n"; break; }
	    if($$x != $r[$x]) { echo "[  ERROR  ]: error $x: [".$$x."] != [".$r[$x]."]\n\n"; break; }

        if(isset($r['error'])) {
	    echo "[ error ] произошла ошибка `".$r['error']."` : "
	    .(isset($r['error_message']) ? $r['error_message'] : "`error_message` not set")."\n\n";
	    break;
	}

        if($r['result']=='payd') {
	    echo "[ OK ] Покупка завершена!\n\n";
	    break;
	}

        if($r['result']=='ready') {
	    $x='account'; if(!isset($r[$x])) { echo "[  ERROR  ]: `$x` not found\n\n"; break; }
	    $account = $r['account'];

	    if($money_transferred) {
		    echo "[ DO:".$kk." ] А $price денег на аккаунт $account мы уже переводили!\n\n";
	    } else {
		    ////////////////////////////
		    echo "[ DO:".$kk." ] Преводим $price денег на аккаунт $account\n\n";
		    $act = "./papa-dai-deneg ".$account." ".$price." --please ".$node."";
		    echo "[ BLOCKCHAIN ] ".$act." \n\n";
		    unset($o); exec($act,$o);

		    $itogo = $o[sizeof($o)-1]; if(substr($itogo,0,3)!='OK ') {
			echo "[ System Error ./papa-dai-deneg ]\n\t\t".implode("\n\t\t",$o)."\n\n";
			exit;
		    }

		    $money_transferred++;
		    echo "\t\t".implode("\n\t\t",$o)."\n\n";
		    ////////////////////////////
	    }

	    sleep($sleep);
	    continue;
	}

        echo "[  ERROR  ]: unknown result=[".$r['result']."] not found\n\n"; break;

    }
    sleep($sleep);
}

function ajax($json) {
    if(gettype($json)!='string') {
	    $json=json_encode($json,JSON_UNESCAPED_UNICODE);
	    if(empty($json)) return array( 'error' => 'json', 'error_message' => 'Wrong INPUT' );
    }
    $json=json_encode(json_decode($json),JSON_UNESCAPED_UNICODE);

    echo "--> [ajax] ".$json."\n\n";

    $ch = curl_init(  );
    curl_setopt_array($ch, array(
        CURLOPT_POSTFIELDS => $json,
        CURLOPT_HTTPHEADER => array('Content-Type:application/json'),
        CURLOPT_RETURNTRANSFER => true,
        CURLOPT_FAILONERROR => true,
        CURLOPT_CONNECTTIMEOUT => 3, // only spend 3 seconds trying to connect
        CURLOPT_TIMEOUT => 10, // 30 sec waiting for answer
	CURLOPT_URL => $GLOBALS['url']
    ));
    $result = curl_exec($ch);

    echo "<-- [result] ".$result."\n\n";

    if (curl_errno($ch)) return array( 'error' => 'connect', 'error_message' => curl_error($ch) );
    $array = json_decode($result);
    if(empty($array)) return array( 'error' => 'json', 'error_message' => 'Wrong json format' );
    curl_close($ch);
    return (array) $array;
}


function creative() {

$m1 = explode("\n","бубен
плащ
стул
вибратор
спонжик
набор отверток
алтарь
карбюратор
чапельник
мастерок
кран");

$m2=explode("\n","зеленый
синий
коричневый
карманный
свежий
для зубной эмали
длинный
для интимной гигиены
с компасом
по скидочке
Б/У
раскладной
на запчасти
для мышей");

return array(
    rand(0,100)*100+1000,
    mb_strtoupper( $m1[rand(0,sizeof($m1)-1)].' '.$m2[rand(0,sizeof($m2)-1)] )
);

}

?>