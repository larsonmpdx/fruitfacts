const puppeteer = require('puppeteer-extra')

const { DEFAULT_INTERCEPT_RESOLUTION_PRIORITY, executablePath } = require('puppeteer')
const AdblockerPlugin = require('puppeteer-extra-plugin-adblocker')
puppeteer.use(
  AdblockerPlugin({
    interceptResolutionPriority: DEFAULT_INTERCEPT_RESOLUTION_PRIORITY,
    blockTrackers: true,
    blockTrackersAndAnnoyances: true
  })
)

const args = process.argv.slice(2);

const USAGE_STRING = 'node index.js [web address to save] [screenshot path]';
if(args.length < 2) {
    console.log(USAGE_STRING);
    process.exit(1);
}
const web_address = args[0];
const screenshot_path = args[1];

puppeteer.launch({ headless: true, ignoreHTTPSErrors: true, executablePath: executablePath() }).then(async browser => {
  const page = await browser.newPage()
  await page.goto(web_address)

  await page.waitForTimeout(5 * 1000)
  await page.screenshot({ path: screenshot_path, fullPage: false })

  console.log(`puppeteer finished`)
  await browser.close()
})
