function comp(array1, array2) {
    if (!Array.isArray(array1) || !Array.isArray(array2)) return false
    if (array1.length === 0 || array2.length === 0) return true
    array1.sort((a, b) => a - b)
    array2.sort((a, b) => a - b)

    for (let i = 0; i < array1.length; i++) {
        if (array1[i] * array1[i] !== array2[i]) return false
    }
    return true
}