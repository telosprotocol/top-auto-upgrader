use rand::SeedableRng;
use rsa::{PaddingScheme, PublicKey, RsaPrivateKey};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct EnvConfigJson {
    machine_id: String,
}

impl EnvConfigJson {
    pub(crate) fn encrypt(&self, pswd: String) -> String {
        let machine_id_u64 =
            u64::from_str_radix(&self.machine_id[0..15], 16).expect("Failed to parse machine id");
        let mut rng = rand::rngs::StdRng::seed_from_u64(machine_id_u64);
        // rsa 2048bits can encrypt no longer than 245 bytes' password
        let priv_key = RsaPrivateKey::new(&mut rng, 2048).expect("Failed to generate a key");
        let pub_key = priv_key.to_public_key();

        let mut enc_rng = rand::thread_rng();
        let padding = PaddingScheme::new_pkcs1v15_encrypt();
        let enc_data = pub_key
            .encrypt(&mut enc_rng, padding, pswd.as_bytes())
            .expect("RSA encrypt failed");
        String::from(hex::encode(enc_data))
    }

    pub(crate) fn decrypt(&self, encrypted_data: &str) -> String {
        let machine_id_u64 =
            u64::from_str_radix(&self.machine_id[0..15], 16).expect("Failed to parse machine id");
        let mut rng = rand::rngs::StdRng::seed_from_u64(machine_id_u64);
        let priv_key = RsaPrivateKey::new(&mut rng, 2048).expect("Failed to generate a key");
        let padding = PaddingScheme::new_pkcs1v15_encrypt();
        let dec_data = priv_key
            .decrypt(
                padding,
                &hex::decode(encrypted_data).expect("Hex decode failed"),
            )
            .expect("RSA decrypt failed");

        String::from_utf8(dec_data).expect("non utf8 data")
    }
}

#[cfg(test)]
mod test {
    use super::EnvConfigJson;
    use rand::{
        distributions::{Alphanumeric, DistString},
        Rng,
    };

    #[test]
    fn test_config_rsa_crypto() {
        let mut rng = rand::thread_rng();
        let rand_machine_id = (0..)
            .map(|_| rng.sample(Alphanumeric) as char)
            .filter(|c| c.is_ascii_hexdigit())
            .take(16)
            .collect();
        let env_config = EnvConfigJson {
            machine_id: rand_machine_id,
        };

        let rand_pswd = Alphanumeric.sample_string(&mut rng, 10);

        let enc = env_config.encrypt(rand_pswd.clone());

        let dec = env_config.decrypt(&enc);

        assert_eq!(rand_pswd, dec);
    }
}
