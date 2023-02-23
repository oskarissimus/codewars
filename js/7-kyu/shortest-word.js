function findShort(s) {
    const lengths = s.split(" ").map(e => e.length)
    return Math.min(...lengths)
}