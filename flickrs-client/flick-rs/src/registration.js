import React from "react";

import { Button, Col, Form, FormCheck, Row } from 'react-bootstrap';
import FormCheckInput from "react-bootstrap/esm/FormCheckInput";
import FormCheckLabel from "react-bootstrap/esm/FormCheckLabel";
import { v4 as uuidv4 } from 'uuid';

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
        fetch("/attributes", { method: "GET" })
            .then(response => response.json())
            .then(data => {
                console.log(data);
                this.setState({ attrs: data["attributes"] })
            });
    }

    handleClick(event) {
        //Do the registration.

        const attrs = this.state.selected_attrs.map(x => parseInt(x));

        const data = JSON.stringify({ gid: this.state.gid, attributes: attrs });

        fetch("/user/register",
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
            this.props.onRegistration(this.state.gid, this.state.selected_attrs, 0);
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
            <Row>
                <Col>
                    <h1>Login page</h1>
                    <p>
                        Welcome to Flick-rs.
                        Please select one or more attributes you are interested in.
                    </p>
                    <Form>
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
                        <Button onClick={(event) => this.handleClick(event)}>Continue</Button>
                    </Form>
                </Col>
            </Row >
        );
    }
}

export default Registration;