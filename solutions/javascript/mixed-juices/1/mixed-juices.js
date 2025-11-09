// @ts-check

const PREP_TIME = {
  'Pure Strawberry Joy': 0.5,
  'Energizer': 1.5,
  'Green Garden': 1.5,
  'Tropical Island': 3.0,
  'All or Nothing': 5.0,
};

/**
 * Determines how long it takes to prepare a certain juice.
 *
 * @param {string} name
 * @returns {number} time in minutes
 */
export function timeToMixJuice(name) {
  return PREP_TIME[name] ?? 2.5;
}

const WEDGES_PER_LIME = {
  'small': 6,
  'medium': 8,
  'large': 10,
}
/**
 * Calculates the number of limes that need to be cut
 * to reach a certain supply.
 *
 * @param {number} wedgesNeeded
 * @param {string[]} limes
 * @returns {number} number of limes cut
 */
export function limesToCut(wedgesNeeded, limes) {
  let idx = 0;
  
  while (wedgesNeeded > 0 && idx < limes.length) {
    wedgesNeeded -= WEDGES_PER_LIME[limes[idx++]] ?? 0;
  }

  return idx;
}

/**
 * Determines which juices still need to be prepared after the end of the shift.
 *
 * @param {number} timeLeft
 * @param {string[]} orders
 * @returns {string[]} remaining orders after the time is up
 */
export function remainingOrders(timeLeft, orders) {
  while (orders.length > 0 && timeLeft > 0) {
    timeLeft -= timeToMixJuice(orders.shift());
  }

  return orders;
}
