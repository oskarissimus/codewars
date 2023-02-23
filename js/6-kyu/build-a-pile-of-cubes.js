function findNb(m) {
    const oddNumbersCount = Math.sqrt(m)
    if (Math.floor(oddNumbersCount) !== oddNumbersCount) return -1
    // equation is n**2 + n + (-1 * C) = 0
    const C = -2 * oddNumbersCount
    const A = 1
    const B = 1
    const Δ = B * B - 4 * A * C
    if (Δ < 0) return -1
    const n1 = (-B - Math.sqrt(Δ)) / (2 * A)
    const n2 = (-B + Math.sqrt(Δ)) / (2 * A)
    if (Math.floor(n2) !== n2) return -1
    return n2
}