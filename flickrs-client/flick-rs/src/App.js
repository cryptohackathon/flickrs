import './App.css';

import React from "react";

// import { Col, Row } from 'react-bootstrap';
import ImageList from './image_list';
import Registration from './registration';
import Profile from './profile';
import Upload from './upload';
import { Container, Row } from 'react-bootstrap';

import { toast } from 'react-toastify';
import 'bootstrap/dist/css/bootstrap.min.css';

import * as Icon from 'react-bootstrap-icons';

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
      this.setState(state => {
        state.imgs.push({
          success: false,
          url: null,
          descr: null,
        });
        const imgs = state.imgs;
        return {
          imgs
        };
      });
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
          success: true,
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

    const req = data["ids"].map(this.getImage);

    Promise.all(req).then(() => {
      toast.success("Everything downloaded! 🎉");
    }).catch((e) => console.log(e));

    // data["ids"].forEach(async (id) => {
    //   await this.getImage(id);
    // });
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
        <Container>
          <h1>Flick.rs</h1>
          <p>
            Flick.rs is a picture sharing application,
              as <a href="https://github.com/cryptohackathon/flickrs/">our submission</a> to
            the <a href="https://cryptohackathon.eu">FENTEC Crypto Hackathon</a>.
            Feel free to take a look at <a href="https://github.com/cryptohackathon/flickrs/raw/master/presentation.pdf">our presentation</a>!
            It demonstrates the feasibility of running <a href="https://en.wikipedia.org/wiki/Attribute-based_encryption">attribute-based encryption</a> schemes in the web browser.
          </p>
          <p>
            The technology stack is built around the <a href="https://eprint.iacr.org/2018/753">ABE scheme of Yan Michalevsky and Marc Joye 2018</a>,
              which we refer to as DIPPE, for Decentralised Inner Product Predicate Encryption.
            We have <a href="https://gitlab.com/etrovub/smartnets/cife-rs">implemented most of the scheme in Rust</a>,
              and <a href="https://gitlab.com/etrovub/smartnets/flickrs">wrote this web application around it</a>.
            The <a href="https://gitlab.com/etrovub/smartnets/flickrs/-/tree/master/flickrs-server">server side code</a> is written using <a href="https://api.rocket.rs/">Rocket</a>,
              while the client uses <a href="https://reactjs.org/">React</a> for the user interface,
              and <a href="https://webassembly.org/">WebAssembly</a> with <a href="https://github.com/rustwasm/wasm-pack/">wasm-pack</a>.
            You&apos;re reading this correctly,
              your browser is running all the fancy cryptography,
              including <a href="https://github.com/dabch/rabe-bn">BN254 elliptic curve pairings</a>,
              and <a href="https://docs.rs/chacha20poly1305/">ChaCha20Poly1305</a>!
          </p>
          <p>
            While the DIPPE scheme is built around multiple authorities, our demonstrator has one single authority, running on our server.
            When you request your attributes down below, the authority hands out decryption keys to your browser.
            You can decrypt the pictures that have only a subset of the attributes that you request;
            this policy is called a &quot;conjunction policy&quot;.
            The DIPPE scheme allows for multiple other policies, but these remain unimplemented in our implementation.
          </p>
          <p class="bs-callout bs-callout-danger">
            <div class="media">
              <Icon.ExclamationTriangle className="mr-3" size={64} />
              <div class="media-body">
                <h5 class="mt-0">Hourly wipe</h5>
                <p>
                  To give everyone a chance to play with this demo, <span class="font-weight-bold">we wipe and reset the data every hour</span>.
                  This is also to keep the load on the servers down.
                </p>
              </div>
            </div>
          </p>
        </Container>
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
          <Row>
            <ImageList
              imgs={imgs}
            ></ImageList>
          </Row>
        </div>
      </React.Fragment>
    );
  }
}

export default App;
