HomoChecker-Jockey
===

> お前も...ホモなんだろ...？

homochecker-jockeyは、[chitoku-k/HomoChecker](https://github.com/chitoku-k/HomoChecker)をオマージュして作られた、Windowsで動く~~☣️マルウェア☣️~~ホモかどうかをチェックするツールです。</br>
Rustで書かれています。</br>

## Disclaimer
**Code samples are provided for educational purposes. Adequate defenses can only be built by researching attack techniques available to malicious actors. Using this code against target systems without prior permission is illegal in most jurisdictions. The authors are not liable for any damages from misuse of this information or code**.</br>
ここで使われた技術を使用し、何かが起きた際には私は一切の責任を負いかねます。</br>
自己責任で使用してください。</br>

## Details
**⚠️ Windows意外では動作しません。**</br>
</br>
homochecker-jockeyでは、2つの重要なセキュリティメソッドが使用されています。</br>
一つ目は、UAC特権昇格です。</br>
homochecker-jockeyが起動されると、まず自己証明書をルートにインストールする処理が始まります。</br>
その処理の過程で管理者権限が必要ですが、homochecker-jockeyでは最新のUAC Bypass技術(嘘、結構古い、Win7から使える)を使用し、更に洗練されたスクリプトでサイレントに証明書をインストールします。</br>
詳細については時間があれば追記します。</br>
</br>
二つ目は、MITMプロキシーです。</br>
homochecker-jockeyの目玉機能でもあり、MITMプロキシーのおかげであなたがHTTPリクエストをする際にヘッダーのUser-Agentを `Homozilla/5.0 (Checker/1.14.514; homOSeX 8.10)` へ強制的に上書きします。</br>

現在のhomochecker-jockeyでは、証明書のルートへのインストールと、User-Agentの書き換えしか行っていませんが、更なる新機能の搭載を予定しています！</br>

## Usage
バイナリは配布しません。コンパイルにはRustのNightlyが必要です。</br>
私の開発環境では `1.52.0-nightly` でした。</br>
</br>
- `cargo run` : homochecker-jockeyを起動します。
- `cargo run -- --help` : helpを表示します。
- `cargo run -- --im-not-homo` : homochecker-jockeyで変更されたプロキシ設定を元に戻します。
</br>
**重要**: ~~ホモは嘘つき~~

## 自己署名された証明書について
このコマンドで作成されました</br>
` openssl req -x509 -newkey rsa:4096 -keyout key.pem -out cert.pem -days 3650 -passout pass:"114514" -subj "/C=HM/ST=private/L=province/O=city/CN=ore.ha.homo"`

MITMプロキシーである、[campbellC/third-wheel](https://github.com/campbellC/third-wheel)の[examples/sign_cert_for_site](https://github.com/campbellC/third-wheel/blob/master/examples/sign_cert_for_site.rs)を使用して `0.0.0.0` と `127.0.0.1` での署名がしてあります。</br>

## TODO
- [ ] consoleに色を付けたい
- [ ] 例のURLに接続すると、[ホモ](https://twiiter.com/mpyw)にリダイレクトされるやつを書く
- [ ] ↑これをcondigで制御できるようにする
