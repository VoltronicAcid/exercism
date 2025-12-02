export const convert = (digits, fromBase, toBase) => {
  if (fromBase < 2) throw new Error("Wrong input base");
  if (toBase < 2) throw new Error("Wrong output base");
  if (digits.length === 0) throw new Error("Input has wrong format");
  if (digits[0] === 0 && digits.length > 1) throw new Error("Input has wrong format");

  let base10Value = digits.reduce((total, num, idx) => {
    if (num < 0 || num >= fromBase) throw new Error("Input has wrong format");

    return fromBase ** (digits.length - 1 - idx) * num + total;
  }, 0);

  const toBaseDigits = [];
  while (true) {
    toBaseDigits.unshift(base10Value % toBase);
    base10Value = Math.floor(base10Value / toBase);

    if (base10Value === 0) break;
  }

  return toBaseDigits;
};
