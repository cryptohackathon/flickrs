import React from "react";
import { Col, Row } from 'react-bootstrap';

import ImageDescription from "./image_description"

class ImageList extends React.Component {
  render() {
    return this.props.imgs && (
      this.props.imgs.map((e, i) => {

        let arrayBufferView = new Uint8Array(this.props.imgs[i]);
        let blob = new Blob([arrayBufferView], { type: "image/png" });
        let urlCreator = window.URL || window.webkitURL;
        let imageUrl = urlCreator.createObjectURL(blob);

        return (
          <Row className="my-3 py-3 border rounded shadow">
            <Col xs={8}>
              <img src={imageUrl} className="img-fluid rounded" />
            </Col>
            <Col xs={4}>
              <ImageDescription wasm={this.props.wasm}></ImageDescription>
            </Col>
          </Row>
        );
      })
    );
  }
}

export default ImageList;