var uniqueInOrder = function (iterable) {
    let previous
    const output = []
    for (item of iterable) {
        if (previous !== item || previous === undefined) {
            output.push(item)
        }
        previous = item
    }
    return output
}