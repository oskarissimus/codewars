function persistence(num) {
    let counter = 0
    while (num > 9) {
        num = String(num).split('').reduce((prev, curr) => prev * curr)
        counter++
    }
    return counter
}