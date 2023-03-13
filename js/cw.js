function linear(x) {
    const a = 3;
    const b = 10;
    return a * x + b
}


const callbackFunction = function () {
    console.log("jestem funkcją callback")
}


function doCallback(x, callbackFunction) {
    if (x === 0) callbackFunction()
    console.log("nie robie callbacka")
}

const callback = n => { console.log(n) }
[1, 2, 3, 4].forEach(callback)


