-- 20260413021000_create_table.sql

-- postgresqlである点に注意してください。

-- galileopt_assets テーブルの作成
-- Galileopt登録済みの物件データを保存するためのテーブル
-- まずは仮作成のため、必要に応じてカラムを追加・変更する可能性があります
CREATE TABLE main.galileopt_assets (
    id          BIGSERIAL    PRIMARY KEY,
    asset_id TEXT NOT NULL UNIQUE,               -- Galileoptの物件ID
    long_name       TEXT         NOT NULL,           -- 物件の正式名称
    short_name      TEXT         NOT NULL           -- 物件の略称
);

-- linkage_histories テーブルの作成
-- 物件データの連携履歴を保存するためのテーブル
CREATE TABLE main.linkage_histories (
    id          BIGSERIAL    PRIMARY KEY,
    linkage_id UUID NOT NULL UNIQUE,               -- 連携ID（UUID形式）
    asset_type CHAR(2) NOT NULL,           -- 物件種別（例: "FA"=固定資産 "LA"=リース）
    csv_file_name TEXT NOT NULL,           -- 連携に使用したCSVファイル名
    csv_file_path TEXT NOT NULL,           -- 連携に使用したCSVファイルの保存パス
    link_status VARCHAR(20) NOT NULL,           -- SUCCESS, FAILURE
    galileopt_result_fa TEXT,           -- Galileoptからの連携結果（固定資産）
    galileopt_result_la TEXT,           -- Galileoptからの連携結果（リース）
    executed_by TEXT NOT NULL,           -- 連携を実行したユーザーID
    linked_at TIMESTAMPTZ NOT NULL DEFAULT NOW(), -- 連携日時
    completed_at TIMESTAMPTZ
);

-- transaction_assets テーブルの作成
-- 本システムで登録／編集した物件データを保存するためのテーブル
-- こちらも仮作成のため、必要に応じてカラムを追加・変更する可能性があります
CREATE TABLE main.transaction_assets (
    id          BIGSERIAL    PRIMARY KEY,
    asset_id TEXT NOT NULL UNIQUE,               -- 本システムの物件ID
    long_name       TEXT         NOT NULL,           -- 物件の正式名称
    short_name      TEXT         NOT NULL,           -- 物件の略称
    linkage_history_id BIGINT REFERENCES main.linkage_histories(id) ON DELETE SET NULL -- 連携履歴テーブルへの外部キー
);
