import React from 'react';

import './App.css';

// interface InPage {
//   add: (num: number) => number
// }


// Page Script
async function helloWorld() {
  // Need to send message to content script

  // @ts-ignore
  let message = await window.call_backend("Hello");
  console.log(`PS: received from IPS: ${message}`);


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
        <button onClick={async () => { await helloWorld(); }
        }>Hello World</button>
      </header>
    </div>
  );
}

export default App;
