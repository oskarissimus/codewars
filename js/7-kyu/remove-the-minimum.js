function removeSmallest(numbersInput) {
    const numbers = [...numbersInput]
    let indexOfMin = 0
    let min = numbers[0]

    for (let i = 0; i < numbers.length; i++) {
        if (numbers[i] < min) {
            indexOfMin = i
            min = numbers[i]
        }
    }
    numbers.splice(indexOfMin, 1)
    return numbers
}