import React from "react";
import { Col, Container, Row } from "react-bootstrap";

class ImageDescription extends React.Component {
    constructor(props) {
        super(props)
        this.state = {
            wasm: null
        };
    }

    componentDidMount() {
        this.get_wasm();
    }

    get_wasm = async () => {
        try {
            const wasm = await import('flick-rs-wasm');
            this.setState({ wasm });
        } catch (err) {
            console.error(`Unexpected error in loadWasm. [Message: ${err.message}]`);
        }
    };

    render() {
        return this.state.wasm && (
            <Container>
                <Row>
                    <Col>
                        <h1>{this.state.wasm.get_image_title()}</h1>
                    </Col>
                </Row>
            </Container>
        );
    }
}

export default ImageDescription;