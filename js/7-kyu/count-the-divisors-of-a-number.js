class Sieve {
    constructor(n) {
        this.sieve = []
        for (let i = 0; i <= n; i++) {
            this.sieve.push(true)
        }
    }

    makeHoles(divisor) {
        for (let i = divisor; i < this.sieve.length; i += divisor) {
            this.sieve[i] = false
        }
    }
}


function calculateDivisorPower(n, divisor) {
    let power = 0
    do {
        power++
        n /= divisor
    } while (n % divisor === 0)
    return power
}

function getDivisorsCnt(n) {
    if (n === 1) return 1
    let sieve = new Sieve(n)
    let totalDivisors = 1
    let primeDivisors = 0
    for (let divisorCandidate = 2; divisorCandidate <= n; divisorCandidate++) {
        if (sieve.sieve[divisorCandidate] && (n % divisorCandidate) === 0) {
            sieve.makeHoles(divisorCandidate)
            primeDivisors++
            const primeDivisorPower = calculateDivisorPower(n, divisorCandidate)
            totalDivisors *= (primeDivisorPower + 1)
        }
    }
    return totalDivisors
}

// console.log(getDivisorsCnt(1)) // 1
console.log(getDivisorsCnt(10)) // 4
console.log(getDivisorsCnt(11)) // 2
console.log(getDivisorsCnt(54)) // 8

