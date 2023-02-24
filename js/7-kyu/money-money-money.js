function calculateYears(principal, interest, tax, desired) {
    let currentBalance = principal
    let years = 0
    while (currentBalance < desired) {
        const accuredInterest = currentBalance * interest
        const taxValue = tax * accuredInterest
        currentBalance = currentBalance + accuredInterest - taxValue
        years++
    }
    return years
}

console.log(calculateYears(1000, 0.05, 0.18, 1000))