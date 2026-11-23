fn SingleThreadedCharacterGenerator(length: usize, alphabet: Vec<&str>) {

    let total = alphabet.len().pow(length as u32);

    for mut i in 0..total  {
        let mut combination = Vec::with_capacity(length);
        for _ in 0..length {
            let mut index = i % alphabet.len();
            combination.push(alphabet[index]);
            i = i / alphabet.len();
        }
        combination.reverse();
        print!("{} ", combination.join(""));
    }
}