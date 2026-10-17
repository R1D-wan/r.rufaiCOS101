fn main() {
    // candidates as a vector of tuples: (name, years of experience)
    let candidates: Vec<(&str, u32)> = vec![
        ("Adaeze", 4),
        ("Tunde", 9),
        ("Chiamaka", 6),
        ("Ibrahim", 12),
        ("Funke", 7),
    ];

    // start with the first candidate as the highest seen so far
    let mut best = candidates[0];

    // walk through the rest, keeping the highest seen so far
    for &(name, years) in candidates.iter() {
        if years > best.1 {
            best = (name, years);
        }
    }

    println!("Candidates interviewed: {}", candidates.len());
    println!(
        "Most experienced developer: {} with {} years",
        best.0, best.1
    );
}