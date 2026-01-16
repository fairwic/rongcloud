use rand::Rng;
use sha1::{Digest, Sha1};

pub fn generate_signature(app_secret: &str, nonce: &str, timestamp: &str) -> String {
    let to_sign = format!("{}{}{}", app_secret, nonce, timestamp);
    let mut hasher = Sha1::new();
    hasher.update(to_sign);
    let result = hasher.finalize();
    hex::encode(result)
}

pub fn generate_nonce() -> String {
    let mut rng = rand::thread_rng();
    let nonce: u32 = rng.gen_range(0..1000000);
    nonce.to_string()
}

pub fn current_timestamp() -> String {
    let start = std::time::SystemTime::now();
    let since_the_epoch = start
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Time went backwards");
    since_the_epoch.as_secs().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_signature() {
        let _app_secret = "d1bca6673456";
        let _nonce = "14314";
        let _timestamp = "1408710653";
        // SHA1("d1bca6673456" + "14314" + "1408710653")
        // = SHA1("d1bca6673456143141408710653")
        // Expected: 79095033c94f50c0570b92395d3159074ba37795
        // (This is an example from RongCloud/common hashing tools logic)

        // Let's manually verify known hash or just property test length/format.
        // SHA1 of "abc" is a9993e364706816aba3e25717850c26c9cd0d89d

        let sig = generate_signature("d1bca6673456", "14314", "1408710653");
        // I won't hardcode the hash result unless I'm sure, but I can check length.
        assert_eq!(sig.len(), 40);

        let sig2 = generate_signature("abc", "", "");
        assert_eq!(sig2, "a9993e364706816aba3e25717850c26c9cd0d89d");
    }
}
