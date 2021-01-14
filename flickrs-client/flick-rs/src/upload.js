import React from "react";
import { Row, Col, Form, Button, FormCheck } from "react-bootstrap";
import FormCheckInput from "react-bootstrap/esm/FormCheckInput";
import FormCheckLabel from "react-bootstrap/esm/FormCheckLabel";
import FormFileInput from "react-bootstrap/esm/FormFileInput";
import FormFileLabel from "react-bootstrap/esm/FormFileLabel";

class Upload extends React.Component {
  constructor(props) {
    super(props);

    this.state = {
      file: null,
      attrs: null,
      selected_attrs: [],
    };

    this.handleUpload = this.handleUpload.bind(this);
    this.handleSelectedFile = this.handleSelectedFile.bind(this);
  }

  componentDidMount() {
    fetch("/api/attributes", { method: "GET" })
      .then(response => response.json())
      .then(data => {
        console.log(data);
        this.setState({ attrs: data["attributes"] })
      });
  }


  handleSelectedFile(event) {
    console.log(event.target.files[0]);
    this.setState({
      file: event.target.files[0],
    });
  }

  handleUpload(event) {
    if (this.state.file === null) {
      console.log("Don't upload null");
      return;
    }

    fetch("/api/upload", {
      method: "POST",
      body: new Blob([this.state.file], { type: this.state.file.type }),
    }).then(resp => {
      if (resp.status === 200) {
        return resp.json();
      } else {
        console.log("Status: " + resp.status);
        return Promise.reject("server");
      }
    }).then(data => {
      console.log(data);
    })
  }

  handleChecked(event) {
    if (event.target.checked) {
      // add attr to list
      this.setState(state => {
        state.selected_attrs.push(event.target.value);
        const selected_attrs = state.selected_attrs;

        return {
          selected_attrs
        };
      });
    } else {
      // remove attr from list
      this.setState(state => {
        const selected_attrs = state.selected_attrs.filter(attr => attr !== event.target.value);

        return {
          selected_attrs
        };
      });
    }
  }

  render() {
    const { attrs } = this.state;
    return attrs && (
      <Row>
        <Col>
          <h1>Upload</h1>
          <Form>
            <FormFileLabel>Select an image to upload to Flick-rs</FormFileLabel>
            <FormFileInput onChange={(event) => this.handleSelectedFile(event)}></FormFileInput>
            <br></br>
            {
              attrs.map((e, i) => {
                return (
                  <FormCheck className="form-check-inline">
                    <FormCheckInput value={attrs[i].id}
                      onChange={(event) => this.handleChecked(event)}></FormCheckInput>
                    <FormCheckLabel>{attrs[i].name}</FormCheckLabel>
                  </FormCheck>);
              })
            }
            <br></br>
            <Button onClick={(event) => this.handleUpload(event)}>Upload</Button>
          </Form>
        </Col>
      </Row>
    );
  }
}

export default Upload;