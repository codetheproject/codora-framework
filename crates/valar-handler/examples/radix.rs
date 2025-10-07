use radix_trie::Trie;

fn main() {
    let mut trie = radix_trie::Trie::new();

    trie.insert("sign_in", String::from("First Sign in"));
    trie.insert("sign_in_cookie", String::from("Second Sign in"));
    trie.insert("sign_in_handler", String::from("Third Sign in"));

    if let Some(_value) = trie.get_raw_descendant("sign_in") {
        // _value.val
    }

    let mut trie = Trie::new();
    trie.insert("bär", 1);
    trie.insert("bären", 2);

    // assert_eq!(*trie.get("bär").unwrap(), 1);
    // let values = trie
    //     .get_raw_descendant("bä")
    //     .unwrap()
    //     .values()
    //     .collect::<HashSet<_>>();
    // assert_eq!([1, 2].iter().collect::<HashSet<_>>(), values);
}
