function enough(cap, on, wait) {
    if (on + wait <= cap) return 0
    const left = cap - on
    return wait - left
}