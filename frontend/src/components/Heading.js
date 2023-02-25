import React from "react";

export default class Heading extends React.Component {
    render() {
        return (
            <h1 className="text-2xl font-medium">{this.props.children}</h1>
        );
    }
}