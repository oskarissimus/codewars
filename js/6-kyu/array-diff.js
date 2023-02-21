function arrayDiff(a, b) {
    let output = [...a]
    for (valueToRemove of b) {
        output = output.filter(value => value !== valueToRemove)
    }
    return output
}