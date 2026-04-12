# depface 仕様書

## 1. プロジェクト概要

Axum ベースの REST API サーバ。ユーザー認証（JWT）とユーザー管理機能を提供し、フロントエンド（React 等の SPA）と組み合わせて使用することを想定している。

---

## 2. 技術スタック

| 分類 | 技術 |
|---|---|
| 言語 | Rust (edition 2021) |
| Web フレームワーク | Axum 0.7 |
| 非同期ランタイム | Tokio 1.0 |
| DB ドライバ | sqlx 0.8 (PostgreSQL) |
| パスワードハッシュ | Argon2 (argon2 0.5) |
| JWT | jsonwebtoken 9.0 |
| シリアライズ | serde / serde_json |
| ロギング | tracing / tracing-subscriber / tracing-appender |
| 環境変数 | dotenvy |

---

## 3. ディレクトリ構成

```
depface/
├── src/
│   ├── main.rs               # エントリーポイント・起動処理
│   ├── lib.rs                # クレートルート（テスト用公開）
│   ├── router.rs             # ルーティング定義
│   ├── logging.rs            # ログ初期化
│   ├── auth/                 # 認証ユーティリティ
│   │   ├── mod.rs
│   │   ├── jwt.rs            # JWT 生成・検証
│   │   └── hash.rs           # Argon2 パスワードハッシュ
│   ├── db/                   # データベース層
│   │   ├── mod.rs
│   │   ├── pool.rs           # DB 接続プール
│   │   ├── user.rs           # UserRepository
│   │   └── token_blacklist.rs # TokenBlacklistRepository
│   ├── handlers/             # HTTP ハンドラー（リクエスト/レスポンス）
│   │   ├── mod.rs
│   │   ├── auth.rs           # ログイン・ログアウト
│   │   └── user.rs           # ユーザー登録
│   ├── middleware/           # ミドルウェア
│   │   ├── mod.rs
│   │   └── auth.rs           # JWT 認証エクストラクター（AuthUser）
│   ├── models/               # データ構造定義
│   │   ├── mod.rs
│   │   ├── auth.rs           # LoginRequest / LoginResponse
│   │   └── user.rs           # User / CreateUserRequest 等
│   ├── services/             # ビジネスロジック
│   │   ├── mod.rs
│   │   ├── service.rs        # AppService（状態コンテナ）
│   │   ├── auth.rs           # ログイン・ログアウトロジック
│   │   ├── user.rs           # ユーザー登録ロジック
│   │   └── tasks_service.rs  # 定時実行タスク（未実装）
│   └── tasks/                # 定時実行タスク定義（未実装）
│       └── mod.rs
├── migrations/               # DB マイグレーション（sqlx）
│   ├── 20260405163001_create_users.sql
│   └── 20260406000000_create_token_blacklist.sql
├── logs/                     # アプリログ出力先（実行時生成）
├── static/                   # フロントエンド静的ファイル配信ディレクトリ
├── .env                      # 環境変数（要作成）
└── Cargo.toml
```

---

## 4. 環境変数

`.env` ファイルをプロジェクトルートに配置する。

| 変数名 | 説明 | 例 |
|---|---|---|
| `DATABASE_URL` | PostgreSQL 接続 URL | `postgres://user:password@localhost:5432/depface_db` |
| `JWT_SECRET` | JWT 署名シークレット（任意の文字列） | `your_secret_key` |

---

## 5. データベース設計

スキーマ名: `main`

### 5.1 users テーブル

ユーザー情報を管理するテーブル。

| カラム | 型 | 制約 | 説明 |
|---|---|---|---|
| id | BIGSERIAL | PRIMARY KEY | 自動採番 ID |
| user_id | VARCHAR(255) | NOT NULL, UNIQUE | ログイン用ユーザー ID |
| user_name | VARCHAR(255) | NOT NULL | 表示名 |
| email | VARCHAR(255) | NOT NULL, UNIQUE | メールアドレス |
| password_hash | TEXT | NOT NULL | Argon2 ハッシュ化済みパスワード |
| created_at | TIMESTAMPTZ | NOT NULL, DEFAULT NOW() | 作成日時 |
| updated_at | TIMESTAMPTZ | NOT NULL, DEFAULT NOW() | 更新日時（トリガーで自動更新） |

- `updated_at` は `update_users_updated_at` トリガーにより UPDATE 時に自動更新される。

### 5.2 token_blacklist テーブル

ログアウト済みの JWT トークンを管理するテーブル。

| カラム | 型 | 制約 | 説明 |
|---|---|---|---|
| id | BIGSERIAL | PRIMARY KEY | 自動採番 ID |
| token | TEXT | NOT NULL, UNIQUE | 無効化済み JWT トークン |
| expires_at | TIMESTAMPTZ | NOT NULL | トークンの有効期限 |

- `expires_at` にインデックス (`idx_token_blacklist_expires_at`) を設定。
- `expires_at > NOW()` の条件でのみ「有効なブラックリスト」と判定する。期限切れレコードは論理的に無視される。

---

## 6. 起動処理

`main.rs` の起動シーケンス：

1. `.env` 読み込み
2. ロギング初期化（コンソール + ファイル出力、JST タイムスタンプ）
3. PostgreSQL 接続プール確立（最大 5 接続）
4. `UserRepository` / `TokenBlacklistRepository` / `AppService` を初期化
5. 初期ユーザー登録チェック（`seed_initial_user`）
6. ルーター作成 → `127.0.0.1:3000` でサーバー起動

### 初期ユーザー自動登録

起動時に `main.users` のレコード数が 0 件の場合、以下の管理者ユーザーを自動登録する。

| 項目 | 値 |
|---|---|
| user_id | `mjscs` |
| user_name | `MjsAdmin` |
| email | `admin@example.com` |
| password | `MJS369CS` |

---

## 7. API 仕様

ベース URL: `http://127.0.0.1:3000`

### 7.1 ログイン

**認証不要**

```
POST /api/auth/login
Content-Type: application/json
```

**リクエストボディ:**

```json
{
  "user_id": "string",
  "password": "string"
}
```

**レスポンス:**

| ステータス | 条件 | ボディ |
|---|---|---|
| 200 OK | 認証成功 | `{ "token": "JWT文字列", "user_id": "string" }` |
| 401 Unauthorized | ユーザー不存在 / パスワード不一致 | `{ "error": "ユーザーIDまたはパスワードが正しくありません" }` |

---

### 7.2 ログアウト

**認証必須** (`Authorization: Bearer <token>`)

```
POST /api/auth/logout
Authorization: Bearer <token>
```

**レスポンス:**

| ステータス | 条件 | ボディ |
|---|---|---|
| 200 OK | ログアウト成功 | `{ "message": "ログアウトしました" }` |
| 401 Unauthorized | トークンなし / 無効 / ブラックリスト済み | `{ "error": "string" }` |
| 500 Internal Server Error | DB エラー等 | `{ "error": "string" }` |

---

### 7.3 ユーザー登録

**認証必須** (`Authorization: Bearer <token>`)

```
POST /api/users
Authorization: Bearer <token>
Content-Type: application/json
```

**リクエストボディ:**

```json
{
  "user_id": "string",
  "user_name": "string",
  "email": "string",
  "password": "string"
}
```

**レスポンス:**

| ステータス | 条件 | ボディ |
|---|---|---|
| 201 Created | 登録成功 | `{ "user_id": "string", "user_name": "string", "email": "string" }` |
| 400 Bad Request | user_id / email 重複 | `{ "error": "このIDまたはメールアドレスは既に使用されています" }` |
| 400 Bad Request | その他 DB エラー | `{ "error": "ユーザー登録に失敗しました" }` |
| 401 Unauthorized | トークンなし / 無効 / ブラックリスト済み | `{ "error": "string" }` |

---

### 7.4 静的ファイル配信

`static/` ディレクトリ以下のファイルを配信する。存在しないパスへのリクエストはすべて `static/index.html` にフォールバックする（SPA ルーティング対応）。

---

## 8. 認証方式

### JWT

- アルゴリズム: HS256（デフォルト）
- 有効期限: 発行から **24 時間**
- ペイロード: `{ "sub": "<user_id>", "iat": <発行時刻>, "exp": <有効期限> }`
- 環境変数 `JWT_SECRET` で署名

### 認証フロー（保護エンドポイント）

```
リクエスト
  ↓
AuthUser エクストラクター（middleware/auth.rs）
  1. Authorization ヘッダーから Bearer トークンを抽出
  2. JWT 署名・有効期限を検証
  3. token_blacklist テーブルを参照し、ブラックリスト済みでないか確認
  4. OK → ハンドラーに { user_id, token } を渡す
  5. NG → 401 を返す
```

### ログアウトの仕組み

JWT はステートレスのため、ログアウト時にトークンを即時無効化するために **トークンブラックリスト** を使用する。

- ログアウト時: トークンと残り有効期間（TTL）を `token_blacklist` テーブルに保存
- トークンがすでに期限切れの場合: DB への書き込みをスキップ
- 認証時: ブラックリストに存在 **かつ** `expires_at > NOW()` であれば拒否

---

## 9. パスワードハッシュ

- アルゴリズム: **Argon2id**（Argon2 デフォルト）
- ソルト: リクエストごとにランダム生成（`SaltString::generate`）
- 同一パスワードでもハッシュ結果は毎回異なる

---

## 10. ロギング

- 出力先: **コンソール** と **ファイル** の両方
- ファイルパス: `logs/app.log`（日次ローリング）
- タイムスタンプ形式: `YYYY-MM-DD HH:MM:SS`（JST）
- ログレベル: 環境変数 `RUST_LOG` で制御、未設定時は `info`

---

## 11. 未実装・今後の予定

| 項目 | ファイル | 状態 |
|---|---|---|
| 定時実行タスク | `src/tasks/mod.rs` | 未実装 |
| タスクサービスロジック | `src/services/tasks_service.rs` | 未実装 |
