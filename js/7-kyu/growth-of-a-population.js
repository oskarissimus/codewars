function nbYear(populationAtTheBeginning, yearlyPopulationGrowthPercent, migration, targetPopulation) {
    console.log(populationAtTheBeginning, yearlyPopulationGrowthPercent, migration, targetPopulation)
    let population = populationAtTheBeginning
    let yearsPassed = 0
    do {
        population = populationAfterAnotherYear(population, yearlyPopulationGrowthPercent, migration)
        yearsPassed++
    } while (population < targetPopulation)
    return yearsPassed
}

function populationAfterAnotherYear(populationAtTheBeginning, yearlyPopulationGrowthPercent, migration) {
    const yearlyPopulationGrowthNumber = populationAtTheBeginning * yearlyPopulationGrowthPercent / 100
    const yearlyPopulationGrowthNumberRounded = Math.floor(yearlyPopulationGrowthNumber)
    return populationAtTheBeginning + yearlyPopulationGrowthNumberRounded + migration
}