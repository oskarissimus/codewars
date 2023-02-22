function sum(numbers) {
    return numbers.reduce((partialSum, number) => partialSum + number, 0)
};