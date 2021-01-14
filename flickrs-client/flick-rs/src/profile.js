import React from "react";
import { Row, Col, Form, Button, FormCheck, Badge } from "react-bootstrap";
import FormCheckInput from "react-bootstrap/esm/FormCheckInput";
import FormFileInput from "react-bootstrap/esm/FormFileInput";

class Profile extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
      attribute_names: [],
    };
  }

  componentDidMount() {
    this.props.attrs.forEach(i => {
      fetch("/api/attributes/" + i).then(data => data.json()).then(name => {
        console.log(name["attribute"]);
        this.setState(state => {
          let { attribute_names } = state;
          attribute_names.push(name["attribute"].name);
          return {
            attribute_names
          };
        });
      })
    });
  }

  render() {
    let attrs = this.state.attribute_names;

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
            <td>{attrs.map((e, i) => {
              return (
                <Badge className="rounded-pill bg-primary text-white mr-1">{attrs[i]}</Badge>
              );
            })}</td>
          </tr>
        </table>
      </React.Fragment>
    );
  }
}

export default Profile;
