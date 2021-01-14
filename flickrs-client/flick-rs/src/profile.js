import React from "react";
import { Row, Col, Form, Button, FormCheck } from "react-bootstrap";
import FormCheckInput from "react-bootstrap/esm/FormCheckInput";
import FormFileInput from "react-bootstrap/esm/FormFileInput";

class Profile extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
      attrs: null,
    };
  }

  render() {
    let attrs = this.props.attrs;

    return (
      <React.Fragment>
        <h1 class="text-center">Your profile</h1>
        <table class="table">
          <tr>
            <th scope="row">Global ID</th>
            <td>{this.props.gid}</td>
          </tr>
          <tr>
            <th scope="row">Your attributes</th>
            <td>{attrs}</td>
          </tr>
        </table>
      </React.Fragment>
    );
  }
}

export default Profile;
