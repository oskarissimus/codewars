function points(games) {
    return games.map(game => {
        const [x, y] = game.split(':')
        if (x > y) return 3
        if (x < y) return 0
        return 1
    }).reduce((previousSum, element) => previousSum + element)
}