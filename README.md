<div align="center">
  <img src="assets/mountix-logo.png" alt="mountix api"/>
  <p>日本の山岳一覧・百名山 API</p>
</div>

## Getting Started

### DB initialization

データベースは [MongoDB Atlas Database](https://www.mongodb.com/ja-jp/atlas/database) を使用します。

事前にプロジェクト、クラスター、データベースを作成し [Connection string](https://www.mongodb.com/docs/manual/reference/connection-string/) を取得する必要があります。

Examples:

- Project Name: `mountix`
- Cluster Name: `Mountix-Cluster0`
- Database Name: `mountix_db`

```shell
cd ./migrations/
./migrate.sh
cd ../
```

Note:

- macOS Monterey version 12.5 でのみ動作確認済みです。
- `mongoimport`については [MongoDB Database Tools 公式ドキュメント](https://www.mongodb.com/docs/database-tools/)を確認してください。


### Run the web application

Create `.env`.

```text:.env
RUST_LOG=debug
HOST=127.0.0.1
PORT=8080

# More information here https://www.mongodb.com/docs/manual/reference/connection-string/
# defaultauthdb is `mountix_db`
DATABASE_URL={mongodb connection url}
DATABASE_NAME=mountix_db
MOUNTAINS_URL=http://127.0.0.1:8080/api/v1/mountains

# See https://github.com/codemountains/mountix-docs
DOCUMENTS_URL=http://127.0.0.1:3000
DEFAULT_DISTANCE=5000
MAX_DISTANCE=100000

# MCP (Model Context Protocol) remote server settings (optional)
# MCP_STATEFUL_MODE=false
# MCP_JSON_RESPONSE=true
# MCP_ALLOWED_HOSTS=localhost,127.0.0.1,127.0.0.1:8080
```

Execute `cargo run` command.

```shell
cargo run
```

## REST API

REST API は Axum で実装されており、JSON 形式で山岳データを提供します。

詳細なリクエスト/レスポンス仕様は [Postman Public API Network](#postman-public-api-network) を参照してください。

### エンドポイント

| メソッド | パス | 概要 |
| --- | --- | --- |
| GET | `/api/v1/` | API の基本情報（`MOUNTAINS_URL` / `DOCUMENTS_URL` など） |
| GET | `/api/v1/mountains` | 条件指定の山岳一覧取得 |
| GET | `/api/v1/mountains/{id}` | 山岳IDで1件取得 |
| GET | `/api/v1/mountains/{id}/surroundings` | 指定した山岳の周辺山岳を取得 |
| GET | `/api/v1/mountains/geosearch` | 緯度経度の矩形範囲で山岳を検索 |
| GET | `/api/v1/hc` | ヘルスチェック |
| GET | `/api/v1/hc/mongo` | MongoDB ヘルスチェック |

### `/api/v1/mountains` の主なクエリパラメータ

| パラメータ | 説明 |
| --- | --- |
| `name` | 山岳名（部分一致） |
| `prefecture` | 都道府県ID |
| `tag` | タグID |
| `offset` | 取得開始位置 |
| `limit` | 取得件数 |
| `sort` | ソート条件 |

### レスポンス形式

成功時は `JsonMountain` / `JsonMountainsResponse` などの JSON、エラー時は `JsonErrorResponse`（`messages` 配列を含む）を返します。MCP ツールも同じスキーマの文字列を返します。

## MCP (Model Context Protocol) Server

[Model Context Protocol](https://modelcontextprotocol.io/) の Streamable HTTP transport によるエンドポイントも提供しています。

Cursor などの MCP 対応クライアントから、山岳データを検索・取得するツールを呼び出すことができます。

### エンドポイント

- `POST /mcp` - MCP Streamable HTTP エンドポイント

### 提供ツール

| ツール名 | 説明 |
| --- | --- |
| `get_mountain` | 山岳IDを指定して山岳情報を1件取得 |
| `find_mountains` | 山岳名・都道府県・タグなどの条件で山岳を検索 |
| `find_mountains_by_box` | 緯度経度の矩形範囲を指定して山岳を検索 |
| `find_surroundings` | 指定した山岳の周辺にある山岳を検索 |

レスポンスは REST API と同じスキーマ（`JsonMountain` など）の JSON 文字列として返されます。

### サーバー設定（環境変数）

`.env` で MCP サーバーの挙動を調整できます（いずれも任意）。

| 環境変数 | デフォルト | 説明 |
| --- | --- | --- |
| `MCP_STATEFUL_MODE` | `false` | Streamable HTTP のセッション管理を有効化するか |
| `MCP_JSON_RESPONSE` | `true` | レスポンスを JSON で返すか（`false` で SSE） |
| `MCP_ALLOWED_HOSTS` | `localhost,127.0.0.1,::1,$HOST` | 許可する `Host` ヘッダ。カンマ区切りで指定。未設定の場合は `HOST` 環境変数の値を含む既定値が使われる |

## Postman Public API Network

Postman Public API Network で API を公開しています。

[https://www.postman.com/codemountains-dev/mountix](https://www.postman.com/codemountains-dev/mountix)

## Architecture

- mountix-driver (driver or controller)
  - ルーターとサーバーの起動を実装する
  - Axum の機能を利用してエンドポイントとサーバーの起動までを実装する
  - 内部的に行われた処理の結果、どのようなステータスコードを返すかをハンドリングしたり、JSON のシリアライズ・デシリアライズも担当する
  - `rmcp` クレートを利用した MCP サーバー（`/mcp` エンドポイント）の公開もこのレイヤーで実装する
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
