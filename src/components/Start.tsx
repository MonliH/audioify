import * as React from "react";
import "../css/button.css";

type StartProps = {
    onClick: (event: React.MouseEvent<HTMLButtonElement>) => void 
}

export class StartButton extends React.Component<StartProps, {}> {
    constructor(props: StartProps) {
        super(props)
    }

    render() {
        return <button onClick={this.props.onClick} id="start-button">Audioifiy It!</button> 
    }
}

