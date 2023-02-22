function expandedForm(num) {
    const output = []
    const digits = String(num).split('').reverse()
    for (let i = 0; i < digits.length; i++) {
        if (digits[i] !== "0") {
            const outputComponent = String(digits[i]).padEnd(i + 1, "0")
            output.unshift(outputComponent)
        }

    }
    return output.join(" + ")
}