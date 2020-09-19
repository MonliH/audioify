import React, { useEffect } from "react";
import {greet} from "wasm-audioify";

function App() {
  useEffect(() => {
    greet();
  }, []);
  return <div className="App"></div>;
}

export default App;
