import React from "react";

import img0 from "./images/img0.jpg"

class Image extends React.Component {
    constructor(props) {
        super(props);
        this.state = {
            data: [],
        };
        this.render();
    }

    render() {
        const { data } = this.state;
        console.log("Rendering: " + data);
        return data && (
            <img img-src={data} alt="this is an image" className="img-fluid rounded" />
        );
    }
}

export default Image;