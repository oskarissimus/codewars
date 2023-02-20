String.prototype.toJadenCase = function () {
    const capitalizedWords = []
    for (word of this.split(" ")) {
        const firstLetter = word.slice(0, 1)
        const rest = word.slice(1)
        const capitalizedFirstLetter = firstLetter.toUpperCase()
        const capitalizedWord = capitalizedFirstLetter + rest
        capitalizedWords.push(capitalizedWord)
    }
    return capitalizedWords.join(" ")
};