import React from 'react';

import './App.css';

// Page Script
function helloWorld() {
  // Need to send message to content script
  console.log("Hello World");

  window.postMessage({
    direction: "from-page-script",
    message: "Message from the page"
  }, "*");
}

function App() {
  return (
    <div className="App">
      <header className="App-header">
        <button onClick={helloWorld}>Hello World</button>
      </header>
    </div>
  );
}

export default App;
