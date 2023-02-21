function digitize(n) {
    if (n === 0) return [0]
    const output = []
    while (n !== 0) {
        const digit = n % 10
        output.push(digit)
        n = Math.floor(n / 10)
    }
    return output
}