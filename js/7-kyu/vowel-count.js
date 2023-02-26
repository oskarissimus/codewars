function getCount(str) {
    return str.split("").filter(e => "aieou".includes(e)).length
}