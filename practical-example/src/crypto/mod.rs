use blake2::Blake2b512;
use blake2::Digest;

const SALT: &str = "u3>eF*dBF9x6]J5}ZrecFbDv97mC0Yj:dwq_:k^jW@zM226mLPVhrhB8h,]jonFXgb.";

pub trait Cryptable {
    fn get_value(&self) -> String;
    fn hashed(&self, hashed_val: String) -> HashedValue;
}

pub struct HashedValue {
    pub value: String
}

impl HashedValue {
    pub fn get_hashed_value(&self) -> String {
        self.value.clone()
    }
}


pub fn crypto_blake2<T: Cryptable>(input: T) -> HashedValue {
    // ソルトをパスワードに追加
    let salted_input = input.get_value() + SALT;

    // Blake2bハッシュを作成
    let mut hasher = Blake2b512::new();
    hasher.update(salted_input);

    // ハッシュ結果を取得
    let result = hasher.finalize();

    let hashed_val = hex::encode(result);

    input.hashed(hashed_val)

  
}