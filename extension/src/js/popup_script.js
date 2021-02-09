export function postMessage() {
  browser.runtime.sendMessage({
    direction: "from-popup-script",
    message: "Message from popup script."
  });
}

function callInPopup(message) {
  console.log("BS says: " + message)
}

const callBs = async () => {
  let bs = await browser.runtime.getBackgroundPage();
  console.log(`Called getItem on BS: ${bs.getItem()}`);
  bs.callPopup = callInPopup;
};

callBs();

window.addEventListener("message", function (event) {
  if (
    event.data &&
    event.data.direction === "from-background-script") {

    console.log(`PS: received message: "${event.data.message}"`);

  } else {
    console.log(`PS: received event: "${event}"`);
  }
});

