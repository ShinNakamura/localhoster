# What is this? #

Windows でブラウザをUIにしてプログラムを実行できるようにするのが目的

EXE が localhost(127.0.0.1:(port)) を起動し

HTML から GET/POST などすると

裏でプログラムが起動して仕事をする



## 開発環境 ##

Windows 10 Home

Rust 1.30.1



## テスト案内文 ##

```cargo build --release```
として実行ファイル(localhoster.exe)を作成し、テストしてもらいたいユーザに共有。

以下は作者がテストユーザに送信した案内文の一部。

----

テストAPP(EXE) localhoster.exe  

テスト手順:  

1.localhoster.exe をダブルクリックして起動してください。 

    コマンドプロンプトが立ち上がって、 "Serving on http://127.0.0.1:31764 ..." というメッセージが表示されるはずです。

    このままコマンドプロンプトは放置して手順(2)に進んでください。  


    * コマンドプロンプトを閉じてしまうと、 localhoster.exe も終了してしまいます。  

    * 手順(1)で可能性のあるエラーとしては、そもそも Windows が  localhoster.exe の実行を許してくれない。 
    その場合は「管理者で実行」などを試してみてください。   


1. ~~ブラウザ(なんでもいいです)で  http://127.0.0.1:31764 に接続してください。 簡単なHTMLが表示されるはずです。~~

    * localhoster.exe 起動の時点で自動的にデフォルトブラウザを起動してページを表示するように変更


1. ページ内の```cmd_test```をクリックしてください。 


このリンクをクリックすると裏でコマンドプロンプトを呼び出し```hello```という文字を表示させるコマンドを実行します(実際は見えません)。 

その実行内容を localhoster.exe が内部で受け取ってブラウザに返します。 

正常に実行が完了すると下記のようなページが表示されるはずです。  

URL: http://127.0.0.1:31764/cmd_test 

```
    cmd_test  

    home  

    Output {
        status: ExitStatus(         
            ExitStatus(             
                0         
            )     
        ),     
        stdout: "hello\r\n",     
        stderr: "" 
    }   
```

* この段階でこの通りにならなかった場合は、外部からインストールしたEXEに対して コマンドプロンプトの実行を許可しない設定がどこかでされている可能性があります。 
* ここの解決法は未知なので、もしこの手順で躓いたら、状況を教えてください。  
        

正常に実行できたら```home```リンクで最初のページに戻ります。   
    

    
1. 最後に、localhoster.exe の停止テストです。 

```quit```というリンクがあるのでクリックしてください。 
すると、```ブラウザには正常に接続できませんでした```というメッセージが表示されるはずです。(ブラウザによってメッセージ内容は異なるかも) 
    
    
また、手順(1)で立ち上がったコマンドプロンプトは閉じられるはずです。    

