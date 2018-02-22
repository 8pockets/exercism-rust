use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, &str> {
    if !valid_nucleotide(nucleotide) {
        return Err("Invalid nucleotide");
    }
    if !dna.chars().all(valid_nucleotide) {
        return Err("Invalid Symbol found!");
    }
    Ok(dna.chars().filter(|s| *s == nucleotide).count())
}

pub fn nucleotide_counts(sequence: &str) -> Result<HashMap<char, usize>, &str> {
    let mut letters = HashMap::new();
    letters.insert('A', count('A', sequence)?);
    letters.insert('C', count('C', sequence)?);
    letters.insert('G', count('G', sequence)?);
    letters.insert('T', count('T', sequence)?);
    Ok(letters)
}

fn valid_nucleotide(nucleo: char) -> bool {
    "ACGT".contains(nucleo)
}
