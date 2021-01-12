import React from "react";
import { Col, Row } from 'react-bootstrap';

import Image from "./image";
import ImageDescription from "./image_description"

class ImageList extends React.Component {
    render() {
        return this.props.imgs && (
            this.props.imgs.map((e, i) => {
                return (
                    <Row className="my-3 py-3 border rounded shadow">
                        <Col xs={8}>
                            <Image data={this.props.imgs[i]}></Image>
                        </Col>
                        <Col xs={4}>
                            <ImageDescription></ImageDescription>
                        </Col>
                    </Row>
                );
            })
        );
    }
}

export default ImageList;