function sumArray(array) {
    if (Array.isArray(array) && array.length > 2) {
        const max = Math.max(...array)
        const min = Math.min(...array)
        const arraySum = array.reduce((partialSum, element) => partialSum + element)
        return arraySum - max - min
    }
    return 0

}
