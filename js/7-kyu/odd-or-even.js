function oddOrEven(array) {
    if (array.length === 0) return "even"
    const parityOfSum = array.reduce((parity, element) => parity % 2 + element % 2)
    return parityOfSum % 2 === 0 ? "even" : "odd"
}