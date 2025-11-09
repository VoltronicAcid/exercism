// @ts-check

/**
 * Calculates the total bird count.
 *
 * @param {number[]} birdsPerDay
 * @returns {number} total bird count
 */
export function totalBirdCount(birdsPerDay) {
  let total = 0;

  for (let idx = 0; idx < birdsPerDay.length; idx += 1) {
    total += birdsPerDay[idx];
  }

  return total;
}

/**
 * Calculates the total number of birds seen in a specific week.
 *
 * @param {number[]} birdsPerDay
 * @param {number} week
 * @returns {number} birds counted in the given week
 */
export function birdsInWeek(birdsPerDay, week) {
  let total = 0;
  const START = (week - 1) * 7;
  const END = START + 7;

  for (let idx = START; idx < END; idx += 1) {
    total += birdsPerDay[idx];
  }

  return total;
}

/**
 * Fixes the counting mistake by increasing the bird count
 * by one for every second day.
 *
 * @param {number[]} birdsPerDay
 * @returns {void} should not return anything
 */
export function fixBirdCountLog(birdsPerDay) {
  for (let idx = 0; idx < birdsPerDay.length; idx += 2) {
    birdsPerDay[idx] += 1;
  }
}
