use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        s: String,
    }
    let mut map: HashMap<&str, usize> = HashMap::new();
    map.insert("tourist", 3858);
    map.insert("ksun48", 3679);
    map.insert("Benq", 3658);
    map.insert("Um_nik", 3648);
    map.insert("apiad", 3638);
    map.insert("Stonefeang", 3630);
    map.insert("ecnerwala", 3613);
    map.insert("mnbvmar", 3555);
    map.insert("newbiedmy", 3516);
    map.insert("semiexp", 3481);

    println!("{}", map[&s[0..]]);
    
}