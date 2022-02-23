const groupAnagrams = (strs: string[]): string[][] => {
  let letters: string[]
  let sorted: string[]
  let sortedWord: string
  let anagramMap: {[key: string]: string[]} = {}

  for (let word of strs) {
    letters = word.split("")
    sorted = letters.sort()
    sortedWord = sorted.join("")

    if (sortedWord in anagramMap) {
      anagramMap[sortedWord].push(word)
    } else {
      anagramMap[sortedWord] = [word]
    }
  }

  return Object.values(anagramMap)
}

export default groupAnagrams
