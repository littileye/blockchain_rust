use bincode;
use serde::{Deserialize, Serialize};
use crypto::digest::Digest;
use crypto::sha3::Sha3;

pub fn my_serialize<T: ?Sized>(value: &T) -> Vec<u8>
    where T: Serialize,
{
    let serialized =  bincode::serialize(value).unwrap();
    serialized
}

pub fn my_deserialize<'a, T>(bytes: &'a[u8]) -> T 
    where T: Deserialize<'a>,
{
    let deserialized = bincode::deserialize(bytes).unwrap();
    deserialized
}

pub fn get_hash(value: &[u8]) -> String {
    let mut hasher = Sha3::sha3_256();
    hasher.input(value);
    hasher.result_str()
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[cfg(test)]
mod tests {
    
    use crate::coder::Point;
    use crate::coder::{my_serialize, my_deserialize};

    use super::get_hash;
    #[test]
    fn coder_works() {
        let point = Point {x: 1, y: 1 };
        let se = my_serialize(&point);
        let de: Point = my_deserialize(&se);
        println!("se = {:?},de = {:?}", se, de);
        assert_eq!(de, point);
    }
    #[test]
    fn hash_work() {
        let value = String::from("value");
        let value = my_serialize(&value);
        println!("value = {:?}", value);
        let hash = get_hash(&value[..]);
        println!("hash = {:?}", hash);
        assert_eq!(hash,String::from("11e0211d66a5486ab013f58079a3137484cf2cc269724f5d1b98d4c2742b2f72")); 
    }
}