use crypto::{Cryptable, HashedValue};

pub mod crypto;

// テスト用のCryptableな型を定義
pub struct TestCryptable {
    value: String,
}

impl Cryptable for TestCryptable {
    fn get_value(&self) -> String {
        self.value.clone()
    }
    fn hashed(&self, hashed_val: String) -> HashedValue {
        HashedValue { value: hashed_val }
    }
}

#[cfg(test)]
mod tests {
    use crypto::crypto_blake2;

    use super::*;

    #[test]
    fn test_crypto_blake2() {
        // テスト用のCryptableオブジェクトを作成
        let test_input = TestCryptable {
            value: String::from("test_password"),
        };

        // Blake2bでハッシュを生成
        let hashed_value = crypto_blake2(test_input);

        let expected_hash = "55060be0b317809da16c83b885555501202f14a2fac170ff977ecbee366c15b62938417c533a3770d020a4e78bbdb0f51dac1b3d6b5b5a9558941b58cb6a3bb3".to_string();

        assert_eq!(hashed_value.get_hashed_value(), expected_hash);
    }
}
