import React from "react";
import { Col, Row, Container, Card } from 'react-bootstrap';
import { CardText } from "react-bootstrap-icons";

import ImageDescription from "./image_description"

class ImageList extends React.Component {
  render() {
    return this.props.imgs && (
      this.props.imgs.map((e, i) => {

        let arrayBufferView = new Uint8Array(this.props.imgs[i]);
        let blob = new Blob([arrayBufferView]);
        let urlCreator = window.URL || window.webkitURL;
        let imageUrl = urlCreator.createObjectURL(blob);

        return (
          <Card className="my-3 shadow">
            <img src={imageUrl} className="card-img-top" />
            <div class="card-body">
              {this.props.wasm.get_image_title()}
            </div>
          </Card>
        );
      })
    );
  }
}

export default ImageList;