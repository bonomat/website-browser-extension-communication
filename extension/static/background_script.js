// background-script.js

browser.runtime.onMessage.addListener(notify);

function notify(message) {
  console.log(`Background script received message from content script: '${message.message}'`);
}
