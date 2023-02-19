function SeriesSum(n) {
    // Happy Coding ^_^
    let sum = 0
    for (let i = 0; i < n; i++) {
        const denominator = i * 3 + 1
        sum += (1 / denominator)
    }
    return sum.toFixed(2)
}