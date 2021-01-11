import logo from './logo.svg';
import './App.css';

import React from "react";

class App extends React.Component {
  componentDidMount() {
    this.loadWasm();
  }

  loadWasm = async () => {
    try {
      const wasm = await import('flick-rs-wasm');
      this.setState({ wasm });
      wasm.end_to_end_conjunction();
    } catch (err) {
      console.error(`Unexpected error in loadWasm. [Message: ${err.message}]`);
    }
  };

  render() {

    return (
      <h1>Hello World</h1>

    );
  }
}

export default App;
