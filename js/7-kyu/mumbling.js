function accum(s) {
    return [...s].map(
        (letter, index) => {
            const firstLetter = letter.toUpperCase()
            const rest = letter.toLowerCase().repeat(index)
            return firstLetter + rest
        }
    )
        .join("-")
}