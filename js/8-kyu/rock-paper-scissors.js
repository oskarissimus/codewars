const rps = (p1, p2) => {
    if (p1Wins(p1, p2)) {
        return "Player 1 won!"
    }
    if (p1Wins(p2, p1)) {
        return "Player 2 won!"
    }
    return "Draw!"
};
function p1Wins(p1, p2) {
    return p1 === "scissors" && p2 === "paper" ||
        p1 === "paper" && p2 === "rock" ||
        p1 === "rock" && p2 === "scissors"
}