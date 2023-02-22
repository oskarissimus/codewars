const expandedForm = num => num
    .toString()
    .split('')
    .reverse()
    .map((value, index) => value.padEnd(index + 1, "0"))
    .filter(v => Number(v))
    .reverse()
    .join(" + ")