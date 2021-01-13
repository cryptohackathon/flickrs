import logo from './logo.svg';
import './App.css';

import React from "react";

import { Col, Row } from 'react-bootstrap';
import ImageList from './image_list';

class App extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
      imgs: [],
    };

    this.updateImages = this.updateImages.bind(this);
  }

  loadWasm = async () => {
    try {
      const wasm = await import('flick-rs-wasm');
      this.setState({ wasm });
    } catch (err) {
      console.error(`Unexpected error in loadWasm. [Message: ${err.message}]`);
    }
  };

  updateImages(imgs) {
    this.setState(state => ({
      imgs: imgs,
    }));
  }

  componentDidMount() {
    this.loadWasm();

    // Get the images
    const requestOptions = {
      method: 'GET'
    };
    fetch('/images', requestOptions)
      .then(response => response.json())
      .then(data => {
        console.log(data["images"]);
        this.setState({ imgs: data["images"] });
      });
  }


  render() {
    const { imgs } = this.state;
    return imgs && (
      <ImageList imgs={this.state.imgs}></ImageList>
    );
  }
}

export default App;
