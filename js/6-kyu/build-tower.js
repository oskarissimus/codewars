function towerBuilder(nFloors) {
    const totalWidth = nFloors * 2 - 1
    const output = []
    for (let lineNo = 0; lineNo < nFloors; lineNo++) {
        const starsLen = lineNo * 2 + 1
        const paddingLen = (totalWidth - starsLen) / 2
        const stars = "*".repeat(starsLen)
        const padding = " ".repeat(paddingLen)
        const line = padding + stars + padding
        output.push(line)
    }
    return output
}


console.log(towerBuilder(3))