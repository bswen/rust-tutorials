use std::collections::HashMap;

pub fn test_hashmaps() {
    //test1();
    count_word();
}

fn count_word() {
    let text = "Don't say hello to boys again, boys. Just don't say it.";
    let mut map = HashMap::new();
    let v:Vec<&str> = text.split_terminator(&[' ', ',','.'][..]).collect();
    for word in v {
        let count = map.entry(word.to_lowercase()).or_insert(0);
        *count+=1;
    }
    println!("{:?}",map);
}