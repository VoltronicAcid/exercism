const romanNumerals = [
  ["", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX"],
  ["", "X", "XX", "XXX", "XL", "L", "LX", "LXX", "LXXX", "XC"],
  ["", "C", "CC", "CCC", "CD", "D", "DC", "DCC", "DCCC", "CM"],
  ["", "M", "MM", "MMM"],
];

export const toRoman = (num) =>
  romanNumerals.reduceRight(
    (output, chars, exp) => (output += chars[Math.floor(num / 10 ** exp) % 10]),
    ""
  );
