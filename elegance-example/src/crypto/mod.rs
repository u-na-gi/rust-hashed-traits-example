use blake2::Blake2b512;
use blake2::Digest;

const SALT: &str = "u3>eF*dBF9x6]J5}ZrecFbDv97mC0Yj:dwq_:k^jW@zM226mLPVhrhB8h,]jonFXgb.";

pub trait Cryptable {
    type Hashed: Hashed;

    fn get_value(&self) -> String;
    fn hashed(&self, hashed_val: String) -> Self::Hashed;
}

pub trait Hashed {
    fn get_hashed_value(&self) -> String;
}

pub fn crypto_blake2<T: Cryptable>(input: T) -> T::Hashed
where
    T: Cryptable,
    T::Hashed: Hashed,
{
     // Add salt to password.
    let salted_input = input.get_value() + SALT;

    // Generate Blake2b512 hash.
    let mut hasher = Blake2b512::new();
    hasher.update(salted_input);

    // Get hash results.
    let result = hasher.finalize();

    let hashed_val = hex::encode(result);

    input.hashed(hashed_val)
}
