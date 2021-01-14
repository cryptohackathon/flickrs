import React from "react";
import { Col, Row, Container } from 'react-bootstrap';

import ImageDescription from "./image_description"

class ImageList extends React.Component {
  render() {
    return this.props.imgs && (
      this.props.imgs.map((e, i) => {
        // let bytes = new Uint8Array(this.props.imgs[i]);

        const { wasm, upk, av, gid, attributes } = this.props;

        // console.log("Bytes: " + bytes);
        console.log(upk);
        console.log(av);
        console.log(gid);
        console.log(attributes);

        let data = wasm.open(upk, av, gid, this.props.imgs[i], attributes);

        if (data === undefined) {
          console.log("Cannot decrypt");
          return;
        } else {
          // console.log(JSON.parse(data));

          // let blob = new Blob([arrayBufferView]);
          // let urlCreator = window.URL || window.webkitURL;
          // let imageUrl = urlCreator.createObjectURL(blob);

          return (
            <Row xs={1} sm={2} className="my-3 py-3 border rounded shadow">
              <Col>
                {/* <img src={imageUrl} className="img-fluid rounded" /> */}
              </Col>
              <Col>
                <ImageDescription wasm={this.props.wasm}></ImageDescription>
              </Col>
            </Row>
          );
        }
      })
    );
  }
}

export default ImageList;