//
// This is only a SKELETON file for the 'Gigasecond' exercise. It's been provided as a
// convenience to get you started writing code faster.
//

export const gigasecond = (startDate) => {
  const gig = new Date();
  gig.setTime(startDate.getTime() + 1000000000000);

  return gig;
};
