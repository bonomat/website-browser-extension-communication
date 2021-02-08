
// content-script.js

window.addEventListener("message", function(event) {
  if (event.source === window &&
    event.data &&
    event.data.direction === "from-page-script") {

    console.log("Content script received message: \"" + event.data.message + "\"");

    browser.runtime.sendMessage(event.data);
  }
});