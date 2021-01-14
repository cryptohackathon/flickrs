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

        if (data === undefined) {
          console.log("Cannot decrypt");
          return;
        } else {
          console.log("before json: " + data);
          data = JSON.parse(data);
          console.log("after json: " + data);

          let blob = new Blob([data.img]);
          let urlCreator = window.URL || window.webkitURL;
          let imageUrl = urlCreator.createObjectURL(blob);

          return (
            <Card className="my-3 shadow">
              <img src={imageUrl} className="card-img-top" />
              <div class="card-body">
                {data.descirption}
              </div>
            </Card>
          );
        }
      })
    );
  }
}

export default ImageList;