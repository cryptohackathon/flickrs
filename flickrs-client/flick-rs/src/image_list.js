import React from "react";
import { Card, Col } from 'react-bootstrap';

import failed_img from "./images/img0.jpg"

import * as Icon from 'react-bootstrap-icons';

class ImageList extends React.Component {
  render() {
    const { imgs } = this.props;
    return (
      imgs.map((e, i) => {

        if (imgs[i].success) {
          return (
            <Col className="col-lg-4 col-md-6">
              <Card className="my-3 shadow">
                <img src={imgs[i].url} className="card-img-top" />
                <div class="card-body">
                  {imgs[i].descr}
                </div>
              </Card>
            </Col>
          );
        } else {
          return (
            <Col className="col-lg-4 col-md-6">
              <Card className="my-3 shadow">
                <Icon.ShieldFillX className="p-3" width="auto" size="128px" />
                <div class="card-body">
                  Unable to decrypt this image 😥
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
