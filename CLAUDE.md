# CLAUDE.md

このファイルは、このリポジトリでコードを扱う際にClaude Codeにガイダンスを提供します。

## プロジェクト概要

Mountixは、クリーンアーキテクチャパターンを使用してRustで構築された日本の山岳データAPIです。百名山データを含む山岳情報のRESTエンドポイントを提供し、MongoDBをデータストアとして使用しています。

## 開発コマンド

### ビルドと実行

- **ビルド**: `cargo build`
- **実行**: `cargo run`  
- **チェック**: `cargo check`
- **フォーマット**: `cargo fmt`
- **リント**: `cargo clippy`
- **テスト**: `cargo test`
- **カバレッジ**: `cargo llvm-cov`

### 環境設定

`sample.env`を`.env`にコピーして設定:
- `DATABASE_URL`にMongoDB接続文字列
- `DATABASE_NAME=mountix_db`
- サーバーのホスト/ポート設定

### データベースセットアップ

開発環境（devcontainer）の場合、起動時にマイグレーションが完了します。

```bash
cd ./migrations/
./migrate.sh
cd ../
```

## インフラストラクチャ

インフラストラクチャは[Render](https://render.com/)を採用しています。

## アーキテクチャ

このプロジェクトは依存性逆転を伴う階層化クリーンアーキテクチャに従います:

### レイヤー構造（上位から下位）
1. **mountix-driver** (Controller/Presentation)
   - Axum Webフレームワークのセットアップとルーティング
   - HTTPハンドラーとJSONシリアライゼーション
   - エントリーポイント: `startup/mod.rs`にサーバー設定
   - ルート: `/api/v1/mountains`, `/api/v1/hc` (ヘルスチェック)

2. **mountix-app** (Use Case/Application)
   - ビジネスロジックとアプリケーションワークフロー
   - kernelとadapterレイヤー間の調整
   - ユースケース実装を含む

3. **mountix-kernel** (Domain/Core)
   - ドメインモデルとビジネスルール
   - リポジトリトレイト定義（adapterで実装）
   - 外部依存性のないコアビジネスロジック

4. **mountix-adapter** (Infrastructure)
   - MongoDB永続化実装
   - 外部サービス統合
   - kernelのリポジトリトレイトを実装

### 依存関係ルール

- 上位レイヤーは下位レイヤーに依存可能
- 下位レイヤーは上位レイヤーに依存不可
- kernelとadapterはDIP（依存性逆転の原則）を使用

## 主要技術

- **Webフレームワーク**: Axum with tokio async runtime
- **データベース**: MongoDB（`DATABASE_URL`経由で接続）
- **ログ**: tracingによるJSON出力
- **環境**: dotenvy（.envファイル読み込み）
- **CORS**: API アクセス用に設定済み

## APIエンドポイント

- `GET /api/v1/mountains` - フィルタリング付き山岳リスト
- `GET /api/v1/mountains/{id}` - 特定の山岳取得
- `GET /api/v1/mountains/{id}/surroundings` - 周辺の山岳取得
- `GET /api/v1/mountains/geosearch` - 地理的検索
- `GET /api/v1/hc` - ヘルスチェック
- `GET /api/v1/hc/mongo` - MongoDBヘルスチェック

## 開発ノート

- 4つのクレートからなるRustワークスペースを使用
- 日本の山岳データドメイン（百名山）に従う
- 実行前に環境設定が必要
- クラウドデプロイにはRenderとMongoDB Atlasを推奨
