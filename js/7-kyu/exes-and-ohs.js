function XO(s) {
    const x = s.split("").filter(e => e === "X" || e === 'x').length
    const o = s.split("").filter(e => e === "O" || e === 'o').length
    return x === o
}