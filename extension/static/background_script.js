// background-script.js

browser.runtime.onMessage.addListener(notify);

function notify(event) {
  if (event.direction === "from-content-script") {
    console.log(`BS: received message from CS: '${event.message}'`);
    setItem(event.message);
  }
}

let item = "not set";

function setItem(i) {
  item = i;
}
