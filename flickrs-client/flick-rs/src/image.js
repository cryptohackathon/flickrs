import React from "react";

import img0 from "./images/img0.jpg"

class Image extends React.Component {
    render() {
        return (
            <img src={img0} alt="this is an image" className="img-fluid rounded" />
        );
    }
}

export default Image;