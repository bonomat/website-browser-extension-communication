// background-script.js

browser.runtime.onMessage.addListener(notify);

let msg = "Hellow world";

function notify(message) {
  console.log(`Background script received message from content script: '${message.message}'`);
  msg = message.message;
}

export function getMsg() {
  return msg;
}
