const path = require('path');
const fs = require('fs');

// output like -2, -1, +0, +1, etc.
export function formatHarvestTime(days) {
  if (days == null) {
    return '(no harvest time)';
  }

  if (days >= 0) {
    return `+${days}`;
  } else {
    return `${days}`;
  }
}

// date: unix seconds
// date_estimated: true if defined, this means it was estimated from a table of patent issue years
export function formatPatentDate(date, date_estimated) {
  date = new Date(date * 1000);
  const now = new Date();

  let output;
  if (date_estimated) {
    output = date.toLocaleDateString('en-US', { year: 'numeric' }) + ' (estimated)';
  } else {
    output = date.toLocaleDateString('en-US', {
      year: 'numeric',
      month: 'long',
      day: 'numeric'
    });
  }

  if (date < now) {
    output += ' (expired)';
  }

  return output;
}

// gets a thumbnail path in /public/ if it exists or returns the placeholder image location
export function getThumbnailLocation(filename) {
  const relativePath = `/data/${filename}`;
  if (fs.existsSync(path.join(process.cwd(), `./public${relativePath}`))) {
    return relativePath;
  } else {
    return `/data/no_preview_image.jpg`;
  }
}
