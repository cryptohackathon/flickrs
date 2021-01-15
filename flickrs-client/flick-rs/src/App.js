import './App.css';

import React from "react";

// import { Col, Row } from 'react-bootstrap';
import ImageList from './image_list';
import Registration from './registration';
import Profile from './profile';
import Upload from './upload';
import { Row } from 'react-bootstrap';

import { toast } from 'react-toastify';
import 'bootstrap/dist/css/bootstrap.min.css';

class App extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
      isRegistered: false,
      imgs: [],
      wasm: null,
      gid: null,
      attrs: [],
      selected_attrs: [],
      interested_attrs: [],
      upk: null,
      server_key: null,
      total_attrs: null,
    };

    this.onRegistration = this.onRegistration.bind(this);
    this.getImages = this.getImages.bind(this);
    this.getImage = this.getImage.bind(this);
  }

  componentDidMount() {
    this.loadWasm();
    this.serverSetup();
    this.loadAttrs();
  }

  async loadWasm() {
    try {
      const wasm = await import('flick-rs-wasm');
      this.setState({ wasm });
    } catch (err) {
      console.error(`Unexpected error in loadWasm. [Message: ${err.message}]`);
    }
  };

  async serverSetup() {
    const resp = await fetch("/api/setup", { method: "GET" });

    if (resp.status !== 200) {
      toast.error("Error connecting to server: status " + resp.status);
    }

    const data = await resp.json();
    this.setState({ server_key: data["server_key"] });
  }

  async loadAttrs() {
    const resp = await fetch("/api/attributes", { method: "GET" });

    if (resp.status !== 200) {
      toast.error("Error connecting to server: status " + resp.status);
    }

    const data = await resp.json();
    this.setState({ total_attrs: data["attributes"].length })
  }

  async getImage(id) {
    const resp = await fetch("/api/" + id, { method: "GET" });

    if (resp.status !== 200) {
      toast.error("Error connecting to server: status " + resp.status);
    }

    const data = await resp.json();

    const { wasm, upk, interested_attrs, gid, total_attrs } = this.state;

    console.log("Trying to decrypt");

    let decrypted = wasm.open(upk, interested_attrs, gid, data.image, total_attrs);

    if (decrypted === undefined || decrypted === null) {
      // Don't show when you cannot decrypt
      // console.log("Cannot decrypt");
      // toast.warning("Cannot decrypt image");
      return;
    } else {

      decrypted = new TextDecoder("utf-8").decode(new Uint8Array(decrypted));
      decrypted = JSON.parse(decrypted);

      let blob = new Blob([new Uint8Array(decrypted.img)]);
      let urlCreator = window.URL || window.webkitURL;
      const imageUrl = urlCreator.createObjectURL(blob);

      const descr = decrypted.description;

      this.setState(state => {
        state.imgs.push({
          url: imageUrl,
          descr: descr,
        });
        const imgs = state.imgs;
        return {
          imgs
        };
      });
    }
  }

  async getImages() {
    toast.info("Downloading and decrypting images...");

    const resp = await fetch('/api/images_list', { method: "GET" });

    if (resp.status !== 200) {
      toast.error("Error connecting to server: status " + resp.status);
    }

    const data = await resp.json();

    data["ids"].forEach(async (id) => {
      await this.getImage(id);
    });
  }

  /// Is called by the Registration component
  async onRegistration(gid, attrs, key) {
    this.setState({
      isRegistered: true,
      gid: gid,
      interested_attrs: attrs.map(x => parseInt(x) - 1),
      upk: key,
    });

    await this.getImages();
  }

  render() {
    let lhs;
    if (this.state.isRegistered) {
      lhs = <Profile gid={this.state.gid} attrs={this.state.interested_attrs}></Profile>;
    } else {
      lhs = <Registration onRegistration={this.onRegistration}></Registration>;
    }

    const {
      wasm,
      server_key,
      selected_attrs,
      total_attrs,
    } = this.state;

    const {
      imgs,
      upk: registration_key,
      interested_attrs,
      gid,
    } = this.state;

    return (
      <React.Fragment>
        <div class="container-md">
          <Row className="my-3 py-3 border rounded shadow">
            <div className="py-3 col-md-6">
              {lhs}
            </div>
            <div className="py-3 col-md-6">
              <Upload
                getImage={this.getImage}
                wasm={wasm}
                server_key={server_key}
                selected_attrs={selected_attrs}
                total_attrs={total_attrs}
              ></Upload>
            </div>
          </Row >
        </div>
        <div class="container-md">
          <ImageList
            imgs={imgs}
          ></ImageList>
        </div>
      </React.Fragment>
    );
  }
}

export default App;
