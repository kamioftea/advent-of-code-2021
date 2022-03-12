const fs = require('fs/promises');
const path = require('path');

async function buildDay(file, day) {
    const contents = await fs.readFile(file, 'utf-8')
    const line = contents.split(/[\n\r]+/)[0]
    // This is my solution for [Advent of Code - Day 1 - _Sonar Sweep_](https://adventofcode.com/2021/day/1)
    const [,title, url] = line.match(/\[Advent of Code - Day \d+ - _([^_]+)_]\(([^)]+)\)/) ?? []

    return {day, title, url};
}

async function buildSolutionData() {
    const solutions = [];
    const dir = await fs.opendir(path.join('..', 'src'));
    for await (const entry of dir) {
        const matches = entry.name.match(/day_(\d+)\.rs/)
        if(entry.isFile() && matches) {
            solutions.push(await buildDay(path.join(dir.path, entry.name), parseInt(matches[1])))
        }
    }
    return solutions
}

module.exports = async function() {
    return {solutions: [...(await buildSolutionData())].sort((a, b) => a.day - b.day)}
}
