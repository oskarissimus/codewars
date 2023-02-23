function getMiddle(s) {
    if (s.length % 2 === 1) return s.charAt(s.length / 2)
    return s.slice(s.length / 2 - 1, s.length / 2 + 1)
}