use blake2::Blake2b512;
use blake2::Digest;

const SALT: &str = "u3>eF*dBF9x6]J5}ZrecFbDv97mC0Yj:dwq_:k^jW@zM226mLPVhrhB8h,]jonFXgb.";

pub trait Cryptable {
    fn get_value(&self) -> String;
    fn hashed(&self, hashed_val: String) -> HashedValue;
}

pub struct HashedValue {
    pub value: String,
}

impl HashedValue {
    pub fn get_hashed_value(&self) -> String {
        self.value.clone()
    }
}

pub fn crypto_blake2<T: Cryptable>(input: T) -> HashedValue {
    // Add Salt to password.
    let salted_input = input.get_value() + SALT;

    // Generate Blake2b512 hash.
    let mut hasher = Blake2b512::new();
    hasher.update(salted_input);

    // Get hash results.
    let result = hasher.finalize();

    let hashed_val = hex::encode(result);

    input.hashed(hashed_val)
}
