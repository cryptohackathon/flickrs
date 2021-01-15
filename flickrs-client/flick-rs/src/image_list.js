import React from "react";
import { Col, Row, Container, Card } from 'react-bootstrap';
import { CardText } from "react-bootstrap-icons";

import ImageDescription from "./image_description"

class ImageList extends React.Component {
  render() {
    return this.props.imgs && (
      this.props.imgs.map((e, i) => {
        const { wasm, upk, av, gid, attributes } = this.props;

        let data = wasm.open(upk, av, gid, this.props.imgs[i], attributes);

        if (data === null) {
          console.log("Cannot decrypt");
          return;
        } else {
          data = new TextDecoder("utf-8").decode(new Uint8Array(data));
          data = JSON.parse(data);

          let blob = new Blob([new Uint8Array(data.img)]);
          let urlCreator = window.URL || window.webkitURL;
          let imageUrl = urlCreator.createObjectURL(blob);

          return (
          <Col className="col-lg-4 col-md-6">
            <Card className="my-3 shadow">
              <img src={imageUrl} className="card-img-top" />
              <div class="card-body">
                {data.descirption}
              </div>
            </Card>
          </Col>
          );
        }
      })
    );
  }
}

export default ImageList;
