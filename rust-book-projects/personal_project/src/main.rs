use std::collections::HashMap;

fn most_common(words: Vec<&str>) -> Option<&K> {
    // i will make a hashmap that stores or update if there is the value
    // and then it will pick out the one with the biggest one
    let mut map: HashMap<&str, usize> = HashMap::new();
    let iter = words.iter();

    iter.for_each(|(key, value)
        .map(|&key| {
            *map.entry(key).or_insert(0) += 1
        }
    ));

    map.into_iter().for_each(|(word, size)| {
        .iter()
        .max_by(|a, b| a.1.cmp(&b.1))
        .map(|(k, _v)| k)
    });
    
}


fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn most_common_word() {
        assert_eq!(
            most_common(vec!["rust", "go", "rust", "python", "rust", "go"]),
            Some("rust")
        );
    }

    #[test]
    fn first_word_wins_tie() {
        assert_eq!(
            most_common(vec!["apple", "banana", "apple", "banana"]),
            Some("apple")
        );
    }

    #[test]
    fn empty_input() {
        assert_eq!(most_common(vec![]), None);
    }

    #[test]
    fn single_word() {
        assert_eq!(most_common(vec!["rust"]), Some("rust"));
    }

    #[test]
    fn all_same() {
        assert_eq!(
            most_common(vec!["rust", "rust", "rust"]),
            Some("rust")
        );
    }
}

