import StepOneContent from "./step-one-content";
import StepTwoContent from "./step-two-content";
import {useState} from "react";
import "./runner.css";


function Runner() {
    const [commands, setCommands] = useState([] as string[]);

    const getCommands = (cmds: string[]) => {
        setCommands(cmds);
    }

    return (
        <>
          <div className="container">
                <h1> NPM Dev Runner (beta) </h1>

                <StepOneContent liftCommands={getCommands}/>
                <StepTwoContent commands={commands}/>

            </div>
        </>
    );
}

export default Runner;