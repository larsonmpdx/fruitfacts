module.exports = Object.freeze({
  // julian days for the first day of each month (for leap years)
  MONTH_START_DAYS: [
    {
      name: 'Jan',
      minus_quarter: 7.75,
      day: 1,
      plus_quarter: 7.75
    },
    {
      name: 'Feb',
      minus_quarter: 7.75,
      day: 32,
      plus_quarter: 7.25
    },
    {
      name: 'Mar',
      minus_quarter: 7.25,
      day: 61,
      plus_quarter: 7.25
    },
    {
      name: 'Apr',
      minus_quarter: 7.75,
      day: 92,
      plus_quarter: 7.5
    },
    {
      name: 'May',
      minus_quarter: 7.5,
      day: 122,
      plus_quarter: 7.75
    },
    {
      name: 'Jun',
      minus_quarter: 7.75,
      day: 153,
      plus_quarter: 7.5
    },
    {
      name: 'Jul',
      minus_quarter: 7.5,
      day: 183,
      plus_quarter: 7.75
    },
    {
      name: 'Aug',
      minus_quarter: 7.75,
      day: 214,
      plus_quarter: 7.75
    },
    {
      name: 'Sep',
      minus_quarter: 7.75,
      day: 245,
      plus_quarter: 7.5
    },
    {
      name: 'Oct',
      minus_quarter: 7.5,
      day: 275,
      plus_quarter: 7.75
    },
    {
      name: 'Nov',
      minus_quarter: 7.75,
      day: 306,
      plus_quarter: 7.5
    },
    {
      name: 'Dec',
      minus_quarter: 7.5,
      day: 336,
      plus_quarter: 7.75
    }
  ],

  FRUIT_BAR_COLORS: {
    Apple: { fill: '#D1D783' },
    Apricot: { fill: '#F3870E' },
    'Asian Pear': { fill: '#DDBF56' },
    'Euro Pear': { fill: '#BCCC52' },
    'Euro Plum': { fill: '#a68ca1' },
    'Japanese Plum': { fill: '#df96a2' },
    'Sour Cherry': { fill: '#ff4d64' },
    'Sweet Cherry': { fill: '#f98a99' },
    Nectarine: { fill: '#db7470' },
    Peach: { fill: '#FCDC75' },
    Fig: {
      fill: '#a78b96' // todo - match the icon when it's made
    },
    Apriplum: { fill: '#f3a149' },
    Plumcot: { fill: '#e29d97' },
    Blueberry: { fill: '#88a4dd' },
    'Hardy Kiwi': {
      fill: '#91AE2E' // todo - match the icon when it's made
    },
    'Fuzzy Kiwi': {
      fill: '#86BF09' // todo - match the icon when it's made
    },
    // todo - finish remainder of colors
    Muscadine: { fill: '#dbc3bd' },
    Grape: { fill: '#e2a19d' },
    Elderberry: { fill: '#E6E6FA' },
    Strawberry: { fill: '#E6E6FA' },
    Raspberry: { fill: '#f2c0c7' },
    Blackberry: { fill: '#b9bfc6' },
    Gooseberry: { fill: '#feca81' },
    Currant: { fill: '#E6E6FA' },
    Saskatoon: { fill: '#E6E6FA' },
    Hazelnut: { fill: '#E6E6FA' },
    Walnut: { fill: '#E6E6FA' },
    Almond: { fill: '#E4813C' },
    Chestnut: { fill: '#E6E6FA' },
    Pistachio: { fill: '#E6E6FA' },
    Pecan: { fill: '#E6E6FA' },
    Citrus: { fill: '#E6E6FA' },
    'Sweet Orange': { fill: '#E6E6FA' },
    Lemon: { fill: '#E6E6FA' },
    Lime: { fill: '#E6E6FA' },
    Kumquat: { fill: '#E6E6FA' },
    Tangelo: { fill: '#E6E6FA' },
    Clementine: { fill: '#E6E6FA' },
    Grapefruit: { fill: '#E6E6FA' },
    Mandarin: { fill: '#E6E6FA' },
    Quince: { fill: '#E6E6FA' },
    Pawpaw: { fill: '#E6E6FA' },
    Crabapple: { fill: '#E6E6FA' },
    Jujube: { fill: '#E6E6FA' },
    Persimmon: { fill: '#E6E6FA' },
    'Nanking Cherry': { fill: '#E6E6FA' },
    Mulberry: { fill: '#E6E6FA' },
    Pomegranate: { fill: '#f986b6' },
    Olive: { fill: '#cbe368' },
    'Prunus Rootstock': { fill: '#E6E6FA' },
    Hops: { fill: '#C9D274' }
  }
});
