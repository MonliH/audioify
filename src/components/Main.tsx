import * as React from "react";
import { CodeEditor, start_code } from "./Editor";
import { StartButton } from "./Start";
import { MusicPlayerDisplay } from "./MusicPlayer";
import * as Tone from "tone";

type MainProps = {}
type MainState = {
    code: string,
    inited: boolean,
    currentPos: number
}

export class Main extends React.Component<MainProps, MainState> {
    synth: Tone.PolySynth;
    constructor(props: MainProps) {
        super(props)
        
        this.state = { code: start_code, inited: false, currentPos: 0 };

        this.synth = new Tone.PolySynth().toDestination();

        Tone.Transport.bpm.value = 400;

        this.generateMusic = this.generateMusic.bind(this);
        this.onStartAudioify = this.onStartAudioify.bind(this);
        this.onTextChange = this.onTextChange.bind(this);
        this.handleNewNote = this.handleNewNote.bind(this);
    }

    generateMusic(): Array<object> {
        let time = 0;
        let part = [];
        let word = "";
        for (const char of this.state.code) {
            word += char;
            if (word === "function" || word === "def" || word === "fn" || word === "func" || word === "fun") {
                part.push({"time": time, "note": ["A2", "E3"], "duration": "1n"})
                word = "";
            }
            
            if (word === "return" || word === "ret") {
                part.push({"time": time, "note": ["D2", "C3"], "duration": "1n"})
                word = "";
            }


            if (char == " ") {
                word = "";
            }

            if (word === "null" || word === "None" || word == "none") {
                part.push({"time": time, "note": ["G2", "D3"], "duration": "1n"})
                word = "";
 
            }

            const char_code = char.charCodeAt(0);
            if (char !== " ") {
                const current = char_code*2;
                part.push({"time": time, "note": [current, current*4/3,], "duration": "8n"});
            }
            time += 0.2;
        }

        return part;
    }

    handleNewNote(time: number, note: any) {
        this.setState({ currentPos: Math.floor(time*5) });

        this.synth.triggerAttackRelease(note.note, note.duration, time, note.velocity);
    }

    onStartAudioify(_: React.MouseEvent<HTMLButtonElement>) {
        if (!this.state.inited) {
            Tone.start();
            this.setState({inited: true})
        }
        
        const generated = this.generateMusic();

        let part = new Tone.Part(this.handleNewNote, generated);

        part.start();

        Tone.Transport.start();
        this.setState({
            currentPos: 0
        })
    }

    onTextChange(code: string) {
        this.setState({code})
    }

    render() {
        return <div>
            <div style={{
                display: "flex"
            }}>
                <CodeEditor onTextChange={this.onTextChange} code={this.state.code}></CodeEditor>
                <MusicPlayerDisplay currentPos={this.state.currentPos} text={this.state.code}></MusicPlayerDisplay>
            </div>

            <div id="button-menu">
                <StartButton onClick={this.onStartAudioify}></StartButton>
            </div>
        </div>
    }
}

