const toSortedChars = (str) => JSON.stringify(
  str
    .toLowerCase()
    .split('')
    .sort((a, b) => a.localeCompare(b))
);

export const findAnagrams = (word, candidates) => {
  const normalized = toSortedChars(word);

  return candidates
    .filter(
      str => word.toLowerCase() !== str.toLowerCase()
        && toSortedChars(str) === normalized
    );
};
