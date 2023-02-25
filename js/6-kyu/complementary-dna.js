function DNAStrand(dna) {
    const complements = new Map([
        ["A", "T"],
        ["T", "A"],
        ["G", "C"],
        ["C", "G"],
    ])
    return dna.split("").map(nucleobase => complements.get(nucleobase)).join("")
}