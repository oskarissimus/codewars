function digPow(n, p) {
    const digits = n.toString().split("")
    let sum = 0
    for (let i = 0; i < digits.length; i++) {
        sum = sum + Number(digits[i]) ** (p + i)
    }
    if (sum % n === 0) return sum / n
    return -1
}