function countSheeps(arrayOfSheep) {
    return arrayOfSheep.reduce(
        (partialSum, isShipPresent) => partialSum + (isShipPresent === true ? 1 : 0)
    )
}