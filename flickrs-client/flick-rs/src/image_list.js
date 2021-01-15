import React from "react";
import { Card } from 'react-bootstrap';
import NotificationManager from "react-notifications/lib/NotificationManager";


class ImageList extends React.Component {
  render() {
    const { imgs } = this.props;
    return (
      imgs.map((e, i) => {
        return (
          <Card className="my-3 shadow">
            <img src={imgs[i].url} className="card-img-top" />
            <div class="card-body">
              {imgs[i].descr}
            </div>
          </Card>
        );
      })
    );
  }
}

export default ImageList;