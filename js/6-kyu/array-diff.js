class Item {
    constructor(index, value) {
        this.index = index
        this.value = value
    }
}

class Items {
    constructor(tab) {
        this.items = []
        for (let i = 0; i < tab.length; i++) {
            this.items.push(new Item(i, tab[i]))
        }
    }
    sortByValue() {
        this.items.sort((a, b) => a.value - b.value)
    }
    sortByIndex() {
        this.items.sort((a, b) => a.index - b.index)
    }
    getValues() {
        return this.items.map(item => item.value)
    }
}

function subtract(a, b) {
    let bi = 0
    let output = new Items([])
    for (let ai = 0; ai < a.items.length; ai++) {
        function biIsOutOfBound() {
            return bi >= b.items.length
        }
        function biIsInRightPlace() {
            const left = ai === 0 ? -Infinity : a.items[ai - 1].value
            const right = ai === a.items.length - 1 ? Infinity : a.items[ai + 1].value
            const current = b.items[bi].value
            return left <= current && current <= right
        }
        function moveBiToRightPlace() {
            while (!biIsOutOfBound() && !biIsInRightPlace()) {
                bi++
            }
        }
        function currentItemsAreEqual() {
            return a.items[ai].value === b.items[bi].value
        }
        moveBiToRightPlace()
        if (biIsOutOfBound() || !currentItemsAreEqual()) {
            output.items.push(a.items[ai])
        } else {
            bi++
        }
    }
    return output
}

const first = new Items([1, 2, 2])
const second = new Items([2])
first.sortByValue()
second.sortByValue()
const result = subtract(first, second)
result.sortByIndex()
console.log(result)