import React from "react";
import { Col, Container, Row } from "react-bootstrap";

class ImageDescription extends React.Component {
    render() {
        return (
            <Container>
                <Row>
                    <Col>
                        <h1>{this.props.wasm.get_image_title()}</h1>
                    </Col>
                </Row>
            </Container>
        );
    }
}

export default ImageDescription;