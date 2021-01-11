import React from "react";
import { Col, Row } from 'react-bootstrap';

import Image from "./image";
import ImageDescription from "./image_description"

let images = [0, 1];

class ImageList extends React.Component {
    render() {
        return (
            images.map((e, i) => {
                return (
                    <Row className="my-3 py-3 border rounded shadow">
                        <Col xs={8}>
                            <Image></Image>
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