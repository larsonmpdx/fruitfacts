// date: unix seconds
export function formatPatentDate(date) {
    date = new Date(date*1000);
    const now = new Date();

    let output = date.toLocaleDateString("en-US", { year: 'numeric', month: 'long', day: 'numeric' });
    if (date < now) {
        output += " (expired)";
    }

    return output;
};