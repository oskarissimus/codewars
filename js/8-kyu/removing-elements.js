function removeEveryOther(arr) {
    const output = []
    for (let i = 0; i < arr.length; i += 2) {
        output.push(arr[i])
    }
    return output
}