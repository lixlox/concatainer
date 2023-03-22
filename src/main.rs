use std::{collections::HashSet, env::args, fs::File, io::BufReader, path::Path};

fn list_concat(arr: &[Vec<String>]) -> Vec<String> {
    let mut result = HashSet::new();
    let first = &arr[0];
    let second = &arr[1];

    for first_str in first {
        for second_str in second {
            let concat = format!("{first_str}{second_str}");
            let concat_bis = format!("{second_str}{first_str}");
            result.insert(concat);
            result.insert(concat_bis);
        }
    }
    let mut sorted: Vec<String> = result.into_iter().collect();
    sorted.sort();
    sorted
}

//[["", "a", "b"],["aa", "ab"]]

// aa, ab, aaa, aab, aba, baa, aab, bab, abb

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() != 2 {
        println!("Invalid usage");
        return;
    }
    let path = Path::new(&args[1]);
    let reader = BufReader::new(File::open(path).unwrap());
    let content: Vec<Vec<String>> = serde_json::from_reader(reader).unwrap();
    let result = list_concat(&content);
    println!("{:?}", result);
}
