var number = function (array) {
    //your awesome code here
    const output = []
    for ([index, line] of array.entries()) {
        const outputLine = `${index + 1}: ${line}`
        output.push(outputLine)
    }
    return output
}