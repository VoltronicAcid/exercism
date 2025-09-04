const COLORS = [
      'black',
      'brown',
      'red',
      'orange',
      'yellow',
      'green',
      'blue',
      'violet',
      'grey',
      'white',
    ];
    
export function decodedResistorValue(colors: string[]): string {
  const num = COLORS.indexOf(colors[0]) * 10 + COLORS.indexOf(colors[1]);
  const total = num * (10 ** COLORS.indexOf(colors[2]));

  if (total >= 1000000000) {
    return `${total / 1000000000} gigaohms`;
  } else if (total >= 1000000) {
    return `${total / 1000000} megaohms`;
  } else if (total >= 1000) {
    return `${total / 1000} kiloohms`;
  } else {
    return `${total} ohms`;
  }
}
