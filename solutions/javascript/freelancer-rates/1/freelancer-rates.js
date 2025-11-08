// @ts-check

const HOURS_PER_DAY = 8;
const DAYS_PER_MONTH = 22;

/**
 * The day rate, given a rate per hour
 *
 * @param {number} ratePerHour
 * @returns {number} the rate per day
 */
export function dayRate(ratePerHour) {
  return HOURS_PER_DAY * ratePerHour;
}

/**
 * Calculates the number of days in a budget, rounded down
 *
 * @param {number} budget: the total budget
 * @param {number} ratePerHour: the rate per hour
 * @returns {number} the number of days
 */
export function daysInBudget(budget, ratePerHour) {
  return Math.floor(budget / dayRate(ratePerHour))
}

/**
 * Calculates the discounted rate for large projects, rounded up
 *
 * @param {number} ratePerHour
 * @param {number} numDays: number of days the project spans
 * @param {number} discount: for example 20% written as 0.2
 * @returns {number} the rounded up discounted rate
 */
export function priceWithMonthlyDiscount(ratePerHour, numDays, discount) {
  const DAY_RATE = dayRate(ratePerHour);
  const FULL_MONTHS = Math.floor(numDays / DAYS_PER_MONTH);
  const DAYS = numDays % DAYS_PER_MONTH;
  const DISCOUNT = 1 - discount;

  return Math.ceil(FULL_MONTHS * DAYS_PER_MONTH * DAY_RATE * DISCOUNT + DAYS * DAY_RATE);
}
