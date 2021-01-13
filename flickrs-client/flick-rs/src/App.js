import './App.css';

import React from "react";

// import { Col, Row } from 'react-bootstrap';
// import ImageList from './image_list';
import Registration from './registration';

class App extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
      isRegistered: false,
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
    // const requestOptions = {
    //   method: 'GET'
    // };
    // fetch('/images', requestOptions)
    //   .then(response => response.json())
    //   .then(data => {
    //     console.log(data["images"]);
    //     this.setState({ imgs: data["images"] });
    //   });
  }

  onRegistration(guid, attrs, key) {
    console.log(guid);
    console.log(attrs);
    console.log(key);
  }


  render() {
    // const { imgs } = this.state;
    // return imgs && (
    //   <ImageList imgs={this.state.imgs}></ImageList>
    // );

    return <Registration onRegistration={this.onRegistration}></Registration>
  }
}

export default App;
