import React from 'react';

import './App.css';

// interface InPage {
//   add: (num: number) => number
// }


// Page Script
function helloWorld() {
  // Need to send message to content script

  // @ts-ignore
  console.log(window.call_backend("Hello"));


  // window.postMessage({
  //   direction: "from-page-script",
  //   message: "Message from the page"
  // }, "*");
}

function App() {

  console.log("Page Script: Hello World");
  return (
    <div className="App">
      <header className="App-header">
        <button onClick={helloWorld}>Hello World</button>
      </header>
    </div>
  );
}

export default App;
