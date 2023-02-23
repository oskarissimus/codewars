function sortArray(array) {
    const odds = extractOdds(array)
    const indexesOfOdds = extractIndexesOfOdds(array)
    odds.sort((a, b) => a - b)
    const outputArray = injectOddsToOriginalArray(odds, indexesOfOdds, array)
    return outputArray
}

function extractOdds(array) {
    return array.filter(e => e % 2)
}

function extractIndexesOfOdds(array) {
    return array.map((e, i) => e % 2 ? i : null).filter(e => e !== null)
}

function injectOddsToOriginalArray(odds, indexesOfOdds, array) {
    const outputArray = [...array]
    for (let i = 0; i < odds.length; i++) {
        const indexOfOdd = indexesOfOdds[i]
        const valOfOdd = odds[i]
        outputArray[indexOfOdd] = valOfOdd
    }
    return outputArray
}

console.log(sortArray([5, 8, 6, 3, 4]))