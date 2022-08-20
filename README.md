# mountix

## Getting Started

### DB initialization

データベースは [MongoDB Atlas Database](https://www.mongodb.com/ja-jp/atlas/database) を使用します。

事前にプロジェクトおよびクラスターを作成し [Connection string](https://www.mongodb.com/docs/manual/reference/connection-string/) を取得する必要があります。

- Project Name: `mountix`
- Cluster Name: `Mountix-Cluster0`

```shell
cd ./migrations/ && ./migrate.sh && cd ../
```

> note
> macOS Monterey version 12.5 でのみ動作確認済みです。 
> 
> `mongoimport`については[MongoDB Database Tools 公式ドキュメント](https://www.mongodb.com/docs/database-tools/)を確認してください。
> 

### Run the web application

```shell
cargo run
```

## Architecture

- mountix-driver (driver or controller)
  - ルーターとサーバーの起動を実装する
  - Axum の機能を利用してエンドポイントとサーバーの起動までを実装する
  - 部的に行われた処理の結果、どのようなステータスコードを返すかをハンドリングしたり、JSON のシリアライズ・デシリアライズも担当する
- mountix-app (app or usecase)
  - ユースケースのレイヤーで、アプリケーションを動作させるために必要なロジックを記述する
  - 複数リポジトリをまたいでアプリケーションに必要なデータ構造を返すなどをおこなう
- mountix-kernel (kernel or domain)
  - ドメインのレイヤーで、アプリケーションのコアとなる実装を記述する
  - ドメインモデルの生成の記述などをおこなう
- mountix-adapter (adapter or infrastructure)
  - 外部サービスとの連携のレイヤー
  - RDS との接続やクエリの発行、MongoDB との接続や操作の実装を記述する

このリストの上側は上位レイヤー、下側は下位レイヤーになることです。
上位のレイヤーは下位のレイヤーを呼び出したり利用したりできますが、逆の呼び出しは許されません。
例えば、driver は app のモジュールを呼び出せますが、app は driver のモジュールを呼び出せません。

kernel と adapter の間にはDIP (Dependency Inversion Principle) が適用されます。例えば、kernel のリポジトリにはtraitの定義があるだけで、その実装は adapter にあります。

driver には Axum の定義程度しかありません。 Axum の`Router`、ハンドラ、サーバの起動、Web アプリケーションの定義や設定に関することは、このレイヤーの中で定義する必要があります。

app はいわゆるユースケースのレイヤーです。このレイヤーはアプリケーションのプロセス全体を制御し、ロジックはこの範囲内で定義する必要があります。

kernel はいわゆるドメインのレイヤーです。このレイヤーはアプリケーションの中核となるコンテキストです。

adapter はインフラストラクチャに関係します。このレイヤーは外部のミドルウェアやサービス、APIに接続し、アクセスすることができます。 アクセスや接続の処理は、このレイヤーに定義されなければなりません。

## License

This project is licensed under the [MIT license](LICENSE).
