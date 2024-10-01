use rand::Rng;


pub fn get_word_from_vec(words: Vec<&str>) -> Option<String> {
    if words.is_empty() {
        return None
    }

    let word_index = rand::thread_rng().gen_range(0..words.len());

    Some(words[word_index].into())
}
