const romanNumerals = [
  ["", "M", "MM", "MMM"],
  ["", "C", "CC", "CCC", "CD", "D", "DC", "DCC", "DCCC", "CM"],
  ["", "X", "XX", "XXX", "XL", "L", "LX", "LXX", "LXXX", "XC"],
  ["", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX"],
];

export const toRoman = (num) =>
  romanNumerals.reduce(
    (chars, arr, idx) => (chars += arr[Math.floor(num / 10 ** (3 - idx)) % 10]),
    "",
  );
