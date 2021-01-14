import React from "react";
import { Row, Col, Form, Button, FormCheck, Badge } from "react-bootstrap";
import FormCheckInput from "react-bootstrap/esm/FormCheckInput";
import FormCheckLabel from "react-bootstrap/esm/FormCheckLabel";
import FormFileInput from "react-bootstrap/esm/FormFileInput";
import FormFileLabel from "react-bootstrap/esm/FormFileLabel";
import { NotificationManager } from "react-notifications";

import * as Icon from 'react-bootstrap-icons';

function wait(ms) {
  var start = new Date().getTime();
  var end = start;
  while (end < start + ms) {
    end = new Date().getTime();
  }
}

class Upload extends React.Component {
  constructor(props) {
    super(props);

    this.state = {
      uploading: false,
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
      NotificationManager.warning('Please select an image before uploading', null, 5000);
      return;
    }

    if (this.state.selected_attrs.length === 0) {
      NotificationManager.warning('Please select one or more attributes before uploading', null, 5000);
      return;
    }

    const { wasm, server_key, total_attrs } = this.props;
    const selected_attrs = this.state.selected_attrs.map(x => parseInt(x) - 1);

    this.setState({ uploading: true });

    let reader = new FileReader();
    reader.readAsArrayBuffer(this.state.file);
    let thiz = this;
    reader.onload = function (evt) {
      console.log(evt.target.result);
      let blob = JSON.stringify({
        img: Array.from(new Uint8Array(evt.target.result)),
        type: thiz.state.file.type,
        description: "this is a description"
      });

      blob = wasm.seal(server_key, blob, selected_attrs, total_attrs);

      fetch("/api/upload", {
        method: "POST",
        body: new File([new Uint8Array(blob)], "contents"),
      }).then(resp => {
        if (resp.status === 200) {
          return resp.json();
        } else {
          console.log("Status: " + resp.status);
          return Promise.reject("server");
        }
      }).then(data => {

        NotificationManager.success("Image uploaded! 🎉", null, 5000);

        thiz.setState({ uploading: false }); //, file: null, selected_attrs: [] });
      })
    };
    reader.onerror = function (evt) {
      // XXX
    }
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

    let spinner = null;

    if (this.state.uploading) {
      spinner = <span id="upload_spinner" class="spinner-border spinner-border-sm mr-1 d-none" role="status" aria-hidden="true"></span>;
    }

    return attrs && (
      <React.Fragment>
        <h1 class="text-center">Upload</h1>
        <p>
          Upload an image.
          The attributes you select here, will determine who can see the uploaded picture.
        </p>
        <div class="bs-callout bs-callout-info">
          <div class="media">
            <Icon.ExclamationTriangle className="mr-3" size={64} />
            <div class="media-body">
              <h5 class="mt-0">Important</h5>
              <p>
                <span class="font-weight-bold">Don't upload any sensitive information</span>.
                Other users who select the same attributes that are used for encrypting this image will have access to the image.
              </p>
            </div>
          </div>
        </div>
        <Form>
          <FormFileInput id="form-file" onChange={(event) => this.handleSelectedFile(event)}></FormFileInput>
          <br></br>
          {
            attrs.map((e, i) => {
              return (
                <FormCheck className="form-check-inline">
                  <FormCheckInput value={attrs[i].id}
                    onChange={(event) => this.handleChecked(event)}></FormCheckInput>
                  <FormCheckLabel>
                    <Badge className="bg-secondary text-white mr-1 p-1">{attrs[i].name}</Badge>
                  </FormCheckLabel>
                </FormCheck>);
            })
          }
          <br></br>
          <br></br>
          <Button onClick={(event) => this.handleUpload(event)}>
            {
              spinner
            }
            Upload
          </Button>
        </Form>
      </React.Fragment>
    );
  }
}

export default Upload;
