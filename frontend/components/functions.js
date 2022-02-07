// date: unix seconds
// date_estimated: true if defined, this means it was estimated from a table of patent issue years
export function formatPatentDate(date, date_estimated) {
    date = new Date(date*1000);
    const now = new Date();

    let output;
    if(date_estimated) {
        output = date.toLocaleDateString("en-US", { year: 'numeric' }) + " (estimated)";
    } else {
        output = date.toLocaleDateString("en-US", { year: 'numeric', month: 'long', day: 'numeric' });
    }

    if (date < now) {
        output += " (expired)";
    }

    return output;
};