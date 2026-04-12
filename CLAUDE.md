 # プロジェクト概要

  Axum ベースの REST API サーバ。ユーザー認証（JWT）とユーザー管理機能を提供する。
  フロントエンド（React SPA）の静的ファイル配信も兼ねており、`static/` ディレクトリを配信 /
  ビルド成果物を配置して使用する。

  ---

  # 技術スタック

  ## バックエンド（Rust）

  | 分類 | 技術 |
  |---|---|
  | 言語 | Rust (edition 2021) |
  | Web フレームワーク | Axum 0.7 |
  | 非同期ランタイム | Tokio 1.0 |
  | DB | sqlx 0.8 + PostgreSQL |
  | パスワードハッシュ | Argon2id (argon2 0.5) |
  | 認証 | JWT (jsonwebtoken 9.0, HS256, 24時間有効) |
  | シリアライズ | serde / serde_json |
  | ロギング | tracing / tracing-subscriber / tracing-appender |
  | 環境変数 | dotenvy |
  | エラー処理 | thiserror |

  ## フロントエンド（front/）

  | 分類 | 技術 |
  |---|---|
  | 言語 | TypeScript |
  | UI ライブラリ | React 19 |
  | ビルドツール | Vite 8 |
  | ルーティング | React Router v7 |
  | スタイリング | Tailwind CSS v4 |
  | アイコン | lucide-react |
  | Markdown 表示 | react-markdown + remark-gfm |

  開発時は Vite の dev サーバー（デフォルト `http://localhost:5173`）が `/api` へのリクエストを `http://127.0.0.1:3000`
  にプロキシする。
  本番時は `front/` をビルドして成果物を `static/` に配置し、Axum が配信する。

  ---

  # ページ構成（フロントエンド）

  | パス | ページ | 認証 |
  |---|---|---|
  | `/` | LoginPage | 不要 |
  | `/menu` | MenuPage | 必須 |
  | `/users/add` | UserAddPage | 必須 |
  | `/doc/1` | DocPage | 不要 |

  認証状態は `AuthContext` で管理し、未認証の場合は `/` へリダイレクト。

  ---

  # よく使うコマンド

  ## バックエンド

  ```bash
  cargo run        # 起動
  cargo test       # テスト
  cargo clippy     # Lint
  cargo fmt        # フォーマット
  cargo check      # ビルド確認

  フロントエンド

  cd front
  npm run dev      # 開発サーバー起動
  npm run build    # ビルド（成果物: front/dist/）
  npm run lint     # ESLint

  ---
  環境変数（.env）

  ┌──────────────┬───────────────────────────────┐
  │    変数名    │             説明              │
  ├──────────────┼───────────────────────────────┤
  │ DATABASE_URL │ PostgreSQL 接続 URL           │
  ├──────────────┼───────────────────────────────┤
  │ JWT_SECRET   │ JWT 署名シークレット          │
  ├──────────────┼───────────────────────────────┤
  │ RUST_LOG     │ ログレベル（未設定時は info） │
  └──────────────┴───────────────────────────────┘

  ---
  お願い

  - 不足している情報があれば、作業前に質問してください
  - 勉強を兼ねて作成しているので直接ファイル操作することはさけて、どのファイルにどのように記述するといった指示ベースで回
  答してください。

  ---