use serde::{de::Visitor, Deserialize};

use std::fs;
#[derive(Deserialize, Debug)]
struct Torrent {
    announce: String,
    info: Info,
}
#[derive(Deserialize, Debug)]
struct Info {
    name: String,
    #[serde(rename = "piece length")]
    plength: usize,
    pieces: Hashes,
    #[serde(flatten)]
    keys: Keys,
}

#[derive(Debug)]
struct Hashes(Vec<[u8; 20]>);
struct HashVisitor;

impl<'de> Visitor<'de> for HashVisitor {
    type Value = Hashes;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("need value modulus 20")
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if v.len() % 20 != 0 {
            return Err(E::custom(format!("length is {}", v.len())));
        }
        Ok(Hashes(
            v.chunks_exact(20)
                .map(|x| x.try_into().expect("guaranteed to be length 20"))
                .collect(),
        ))
    }
}

impl<'de> Deserialize<'de> for Hashes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_bytes(HashVisitor)
    }
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum Keys {
    SingleFile { length: usize },
    MultiFile { files: File },
}

#[derive(Deserialize, Debug)]
struct File {
    length: usize,
    path: Vec<String>,
}

fn main() {
    let file = "sample.torrent";
    let contents = fs::read(file).expect("cannot read torrent file");
    let x: Torrent = serde_bencode::from_bytes(&contents).unwrap();
    println!("{x:?}");
}
