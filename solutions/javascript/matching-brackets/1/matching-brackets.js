export const isPaired = (str) => {
  const closers = new Map()
    .set('}', '{')
    .set(']', '[')
    .set(')', '(');
  const openers = Array.from(closers.values());
  const stack = [];

  for (const ch of str.split('')) {
    if (closers.has(ch)) {
      if (stack.length === 0 || stack.pop() !== closers.get(ch)) return false;

    } else if (openers.includes(ch)) {
      stack.push(ch);
    }
  }

  return stack.length === 0;
};
