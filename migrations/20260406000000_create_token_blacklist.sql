-- migrations/20260406000000_create_token_blacklist.sql

-- トークンブラックリストテーブルの作成
CREATE TABLE main.token_blacklist (
    id         BIGSERIAL    PRIMARY KEY,
    token      TEXT         NOT NULL UNIQUE,               -- 無効化されたJWTトークン
    expires_at TIMESTAMPTZ  NOT NULL                       -- トークンの有効期限
);

-- 有効期限切れのレコードを効率的に検索・削除するためのインデックス
CREATE INDEX idx_token_blacklist_expires_at ON main.token_blacklist (expires_at);
