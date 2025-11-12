export const primeFactors = (num) => {
  if (num < 2) {
    return [];
  } else if (num < 4) {
    return [num];
  }

  const factors = [];
  const primes = Array.from({ length: Math.ceil(Math.sqrt(num)) + 1 }).fill(true);

  for (let idx = 2; idx < primes.length; idx += idx % 2 ? 2 : 1) {
    if (primes[idx] === false) continue;

    while (num % idx === 0) {
      factors.push(idx);
      num /= idx;
    }

    for (let multiple = idx << 1; multiple < primes.length; multiple += idx) {
      primes[idx] = false;
    }
  }

  if (num !== 1) factors.push(num);

  return factors;
};
