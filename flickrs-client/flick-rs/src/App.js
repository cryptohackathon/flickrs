import './App.css';

import React from "react";

// import { Col, Row } from 'react-bootstrap';
import ImageList from './image_list';
import Registration from './registration';

class App extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
      wasm: null,
      isRegistered: false,
      imgs: [],
      gid: null,
      attrs: null,
      registration_key: null,
      server_key: null,
    };

    this.onRegistration = this.onRegistration.bind(this);
    this.getImages = this.getImages.bind(this);
  }

  componentDidMount() {
    this.loadWasm();
    this.serverSetup();
  }

  async loadWasm() {
    try {
      const wasm = await import('flick-rs-wasm');
      this.setState({ wasm });
    } catch (err) {
      console.error(`Unexpected error in loadWasm. [Message: ${err.message}]`);
    }
  };

  serverSetup() {
    fetch("/api/setup", { method: "GET" })
      .then(response => response.json())
      .then(data => this.setState({ server_key: data["server_key"] }));
  }

  getImages() {
    // Get the images, better would be one by one.
    fetch('/api/images', { method: "GET" })
      .then(response => response.json())
      .then(data => {
        console.log(data["images"]);
        this.setState({ imgs: data["images"] });
      });
  }

  /// Is called by the Registration component
  onRegistration(gid, attrs, key) {
    this.setState({
      isRegistered: true,
      gid: gid,
      attrs: attrs,
      registration_key: key,
    });

    this.state.wasm.try_deserde(key);

    // TODO: just for now to display something.
    this.getImages();
  }

  render() {
    console.log(this.state);
    if (this.state.isRegistered) {
      const { imgs } = this.state;
      return imgs && (
        <ImageList imgs={this.state.imgs} wasm={this.state.wasm}></ImageList>
      );
    } else {
      return <Registration onRegistration={this.onRegistration}></Registration>
    }
  }
}

export default App;
