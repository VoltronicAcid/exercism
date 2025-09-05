const DNA_MAP = {
  'G': 'C', 'C': 'G', 'T': 'A', 'A': 'U',
};
const NUCLEOTIDES = ['G', 'C', 'T', 'A"'];

export function toRna(dna: string): string {
  const rna = dna.split('').map((ch) => {
    if (!Object.keys(DNA_MAP).includes(ch)) {
      throw('Invalid input DNA.');
    }

    return DNA_MAP[ch];
  });

  return rna.join("");
}
