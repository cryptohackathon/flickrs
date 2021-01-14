import './App.css';

import React from "react";
import * as Icon from 'react-bootstrap-icons';

// import { Col, Row } from 'react-bootstrap';
import ImageList from './image_list';
import Registration from './registration';
import Profile from './profile';
import Upload from './upload';
import { Container, Col, Row } from 'react-bootstrap';

import 'bootstrap/dist/css/bootstrap.min.css';

class App extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
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

  getImage(id) {
    const api_str = "/api/" + id;
    console.log(api_str);
    fetch(api_str, { method: "GET" })
      .then(response => {
        console.log(response);
        return response.json();
      })
      .then(data => {
        console.log(data["image"]);

        this.setState(state => {
          state.imgs.push(data["image"]);
          const imgs = state.imgs;
          return {
            imgs
          };
        })
      })
  }

  getImages() {
    // Get the images, better would be one by one.
    fetch('/api/images_list', { method: "GET" })
      .then(response => response.json())
      .then(data => {
        data["ids"].forEach(id => {
          this.getImage(id);
        });
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

    // TODO: just for now to display something.
    this.getImages();
  }

  render() {
    console.log(this.state);
    let lhs;
    if (this.state.isRegistered) {
      lhs = <Profile gid={this.state.gid} attrs={this.state.attrs}></Profile>;
    } else {
      lhs = <Registration onRegistration={this.onRegistration}></Registration>;
    }
    return (
      <React.Fragment>
        <Container>
          <Row className="my-3 py-3 border rounded shadow">
            <Col>
              {lhs}
            </Col>
            <Col>
              <Upload></Upload>
            </Col>
          </Row >
        </Container>
        <Container>
          <ImageList imgs={this.state.imgs} wasm={this.state.wasm}></ImageList>
        </Container>
      </React.Fragment>
    );
  }
}

export default App;
