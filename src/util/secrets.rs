use secrecy::SecretString;
use std::collections::HashMap;

#[must_use]
pub fn to_secret_map<S: ::std::hash::BuildHasher + Default>(
    map: HashMap<String, String, S>,
) -> HashMap<String, SecretString, S> {
    map.into_iter()
        .map(|(k, v)| (k, SecretString::from(v)))
        .collect()
}
