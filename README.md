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
追記: 自前のMITMプロキシー搭載により、依存関係が多くなったのでビルドに少し時間がかかります。</br>
</br>
- `cargo run` : homochecker-jockeyを起動します。
- `cargo run -- --help` : helpを表示します。
- `cargo run -- --im-not-homo` : homochecker-jockeyで変更されたプロキシ設定を元に戻します。
</br>
**重要**: ~~ホモは嘘つき~~

## 自己署名された証明書について
argusという自作のMITMプロキシーにあるcertツールで作られました。</br>
X509証明書と、RSAキー長は4096bit、パスワードは**いつもの**です。

## TODO
- [ ] consoleに色を付けたい
- [ ] localhostに接続すると例のURLへリダイレクトされるやつ
