// content-script.js

window.addEventListener("message", function (event) {
  if (event.source === window &&
    event.data &&
    event.data.direction === "from-page-script") {

    console.log("CS: received message from Page: \"" + event.data.message + "\"");

    browser.runtime.sendMessage({
        direction: "from-content-script",
        message: event.data.message
      }
    );
  }
});