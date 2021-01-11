import logo from './logo.svg';
import './App.css';

import React from "react";

import Image from "./image";
import { Col, Row } from 'react-bootstrap';
import ImageList from './image_list';

class App extends React.Component {
  componentDidMount() {
    this.loadWasm();
  }

  loadWasm = async () => {
    try {
      const wasm = await import('flick-rs-wasm');
      this.setState({ wasm });
    } catch (err) {
      console.error(`Unexpected error in loadWasm. [Message: ${err.message}]`);
    }
  };

  render() {
    return (
      <ImageList></ImageList>
    );
  }
}

export default App;
