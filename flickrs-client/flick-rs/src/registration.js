import React from "react";
import * as Icon from 'react-bootstrap-icons';

import { Button, Form, FormCheck, Badge } from 'react-bootstrap';
import FormCheckInput from "react-bootstrap/esm/FormCheckInput";
import FormCheckLabel from "react-bootstrap/esm/FormCheckLabel";
import { v4 as uuidv4 } from 'uuid';

import { NotificationManager } from 'react-notifications';

class Registration extends React.Component {
  constructor(props) {
    super(props);

    this.state = {
      gid: uuidv4(),
      attrs: null,
      selected_attrs: [],
    }
  }

  componentDidMount() {
    fetch("/api/attributes", { method: "GET" })
      .then(response => response.json())
      .then(data => {
        this.setState({ attrs: data["attributes"] })
      });
  }

  handleRegistration() {

    const attrs = this.state.selected_attrs.map(x => parseInt(x));

    if (attrs.length === 0) {
      NotificationManager.warning('Please select at least one attribute', null, 5000);
      return;
    }

    const data = JSON.stringify({ gid: this.state.gid, attributes: attrs });

    fetch("/api/user/register",
      {
        credentials: "same-origin",
        mode: "same-origin",
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: data,
      }
    ).then(resp => {
      if (resp.status === 200) {
        return resp.json();
      } else {
        console.log("Status: " + resp.status)
        return Promise.reject("server")
      }
    }).then(data => {
      console.log(data);
      this.props.onRegistration(this.state.gid, this.state.selected_attrs, data.registration_key);
    });

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
      <React.Fragment>
        <h1 class="text-center">Registration</h1>
        <p>
          Register your profile.
          The attributes you select here, will determine the pictures you can see.
        </p>
        <div class="bs-callout bs-callout-info">
          <div class="media">
            <Icon.ShieldCheck className="mr-3" size={64} />
            <div class="media-body">
              <h5 class="mt-0">Security information</h5>
              <p>
                The attributes you select here, will be sent to the server.
                The server is the <span class="font-weight-bold">single authority</span>,
                and will return your decryption key.
                In a real deployment of this ABE scheme, there can be multiple distinct authorities.
              </p>
            </div>
          </div>
        </div>
        <Form>
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
          <Button onClick={(event) => this.handleRegistration()}>Select attributes</Button>
        </Form>
      </React.Fragment>
    );
  }
}

export default Registration;
