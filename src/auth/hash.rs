// src/auth/hash.rs

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

/// 生パスワードを Argon2 でハッシュ化する
pub fn hash_password(password: &str) -> Result<String, String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| format!("ハッシュ化に失敗しました: {}", e))?
        .to_string();

    Ok(password_hash)
}

/// パスワードの検証
pub fn verify_password(password: &str, hash: &str) -> Result<bool, String> {
    let parsed_hash =
        PasswordHash::new(hash).map_err(|e| format!("ハッシュ形式が不正です: {}", e))?;

    let argon2 = Argon2::default();

    Ok(argon2
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_hashing_and_verification() {
        let password = "my_secure_password_123";
        println!("テスト用パスワード: {}", password);

        // 1. ハッシュ化の実行
        let hash = hash_password(password).expect("ハッシュ化に失敗しました");
        println!("生成されたハッシュ値: {}", hash);

        // 2. 「複雑なハッシュ値」になっているかの検証
        // 生パスワードが含まれていないこと
        assert!(
            !hash.contains(password),
            "ハッシュ値に生パスワードが含まれています"
        );

        // Argon2 の標準形式（$argon2id$...）で始まっていること
        assert!(
            hash.starts_with("$argon2id$"),
            "ハッシュ形式が Argon2id ではありません: {}",
            hash
        );

        // 十分な長さ（複雑さ）があること（通常 90文字前後になります）
        assert!(
            hash.len() > 50,
            "ハッシュ値が短すぎます。複雑さが足りない可能性があります"
        );

        // 3. 正しいパスワードでの検証
        let is_valid = verify_password(password, &hash).expect("検証処理自体に失敗しました");
        assert!(is_valid, "正しいパスワードなのに検証に失敗しました");

        // 4. 間違ったパスワードでの検証
        let is_invalid =
            verify_password("wrong_password", &hash).expect("検証処理自体に失敗しました");
        assert!(
            !is_invalid,
            "間違ったパスワードなのに検証をパスしてしまいました"
        );
    }

    #[test]
    fn test_salt_is_random() {
        let password = "same_password";

        // 同じパスワードでも、2回ハッシュ化したらソルトが違うので結果が変わるはず
        let hash1 = hash_password(password).unwrap();
        let hash2 = hash_password(password).unwrap();

        assert_ne!(
            hash1, hash2,
            "同じパスワードで同じハッシュ値が生成されました。ソルトがランダムではありません"
        );
    }
}
