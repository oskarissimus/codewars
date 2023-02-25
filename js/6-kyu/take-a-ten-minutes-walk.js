function isValidWalk(walk) {
    if (walk.length !== 10) return false
    const n = walk.filter(e => e === "n").length
    const s = walk.filter(e => e === "s").length
    const w = walk.filter(e => e === "w").length
    const e = walk.filter(e => e === "e").length
    if (n === s && w === e) return true
    return false
}