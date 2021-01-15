import React from "react";
import { Card, Col } from 'react-bootstrap';


class ImageList extends React.Component {
  render() {
    const { imgs } = this.props;
    return (
      imgs.map((e, i) => {
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
      })
    );
  }
}

export default ImageList;
