function getDivisorsCnt(n) {
    let divisors = 0
    for (let divisor = 1; divisor <= n; divisor++) {
        if (n % divisor === 0) {
            divisors++
        }
    }
    return divisors
}