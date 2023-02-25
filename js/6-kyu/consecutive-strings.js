function longestConsec(strarr, k) {
    const n = strarr.length
    if (n === 0 || k > n || k <= 0) return ""
    const joined = []
    for (let i = 0; i <= n - k; i++) {
        joined.push(strarr.slice(i, i + k).join(""))
    }
    let output = joined[0]
    for (s of joined) {
        if (s.length > output.length) {
            output = s
        }
    }
    return output
}