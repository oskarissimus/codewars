function wave(str) {
    const output = []
    for (let i = 0; i < str.length; i++) {
        if (str[i] === " ") continue
        const newStr = str.split("")
        newStr[i] = newStr[i].toUpperCase()
        output.push(newStr.join(""))
    }
    return output
}

console.log(wave("hello"))