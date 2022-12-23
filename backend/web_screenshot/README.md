* used to generate screenshots of reference websites
* normally run from a rust script that looks for references specifying a web link screenshot that don't already have an associated *.jpg
* note Dec 2022: I chose not to use the existing rust interfaces to puppeteer because they didn't have a clear story for ad blocking and were 2nd class citizens to the node stuff around puppeteer in general. this will probably improve over time
