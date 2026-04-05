-- migrations/20260405163001_create_users.sql
-- Add migration script here

-- mainスキーマの作成
CREATE SCHEMA IF NOT EXISTS main;

-- ユーザーテーブルの作成
CREATE TABLE main.users (
    id BIGSERIAL PRIMARY KEY,           -- 自動インクリメントするID
    user_id VARCHAR(255) NOT NULL UNIQUE, -- ユーザーID（重複不可）
    user_name VARCHAR(255) NOT NULL,   -- ユーザー名
    email VARCHAR(255) NOT NULL UNIQUE, -- ログイン用メールアドレス（重複不可）
    password_hash TEXT NOT NULL,        -- ハッシュ化されたパスワード
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP, -- 作成日時
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP  -- 更新日時
);

CREATE OR REPLACE FUNCTION main.update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_users_updated_at
    BEFORE UPDATE ON main.users
    FOR EACH ROW
    EXECUTE PROCEDURE main.update_updated_at_column();